use crate::configuration::config::Config;
use crate::errors::bad_request::BadRequest;
use crate::errors::internal_server_error::InternalServerError;
use crate::repository::user::user_model::User;
use crate::web::controller::user::user_controller::ConvertError;
use crate::web::dto::authentication::login_request::LoginRequest;
use crate::web::dto::authentication::login_response::LoginResponse;
use crate::web::dto::authentication::register_request::RegisterRequest;
use crate::web::dto::permission::permission_dto::SimplePermissionDto;
use crate::web::dto::role::role_dto::SimpleRoleDto;
use crate::web::dto::user::user_dto::SimpleUserDto;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use log::error;
use mongodb::bson::oid::ObjectId;

/// # Summary
///
/// Convert a User into a SimpleUserDto
///
/// # Arguments
///
/// * `user` - A User
/// * `pool` - The database connection pool
/// * `audit_service` - The AuditService
///
/// # Example
///
/// ```
/// let user_dto = convert_user_to_simple_dto(user, &pool, &audit_service).await;
/// ```
///
/// # Returns
///
/// * `Result<SimpleUserDto, ConvertError>` - The result containing the SimpleUserDto or the ConvertError that occurred
pub async fn convert_user_to_simple_dto(
    user: User,
    pool: &Config,
) -> Result<SimpleUserDto, ConvertError> {
    let mut user_dto = SimpleUserDto::from(user.clone());

    if user.roles.is_some() {
        let mut role_vec: Vec<String> = vec![];
        for r in user.roles.unwrap() {
            role_vec.push(r.to_hex());
        }

        let roles = match pool
            .services
            .role_service
            .find_by_id_vec(role_vec, &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                return Err(ConvertError::RoleError(e));
            }
        };

        if !roles.is_empty() {
            let mut role_dto_list: Vec<SimpleRoleDto> = vec![];

            for r in &roles {
                let mut role_dto = SimpleRoleDto::from(r);
                if r.permissions.is_some() {
                    let mut permission_dto_list: Vec<SimplePermissionDto> = vec![];

                    let mut p_id_vec: Vec<String> = vec![];
                    for p in r.permissions.clone().unwrap() {
                        p_id_vec.push(p.to_hex());
                    }

                    let permissions = match pool
                        .services
                        .permission_service
                        .find_by_id_vec(p_id_vec, &pool.database)
                        .await
                    {
                        Ok(d) => d,
                        Err(e) => return Err(ConvertError::PermissionError(e)),
                    };

                    if !permissions.is_empty() {
                        for p in permissions {
                            permission_dto_list.push(SimplePermissionDto::from(p));
                        }
                    }

                    if !permission_dto_list.is_empty() {
                        role_dto.permissions = Some(permission_dto_list)
                    }
                }

                role_dto_list.push(role_dto);
            }

            user_dto.roles = Some(role_dto_list);
        }
    }

    Ok(user_dto)
}

#[utoipa::path(
    post,
    path = "/api/v1/authentication/login/",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "OK", body = LoginResponse),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Authentication",
)]
#[post("/login/")]
pub async fn login(
    login_request: web::Json<LoginRequest>,
    pool: web::Data<Config>,
) -> HttpResponse {
    let login_request = login_request.into_inner();

    if login_request.username.is_empty() {
        return HttpResponse::BadRequest().json("Username is required");
    }
    if login_request.password.is_empty() {
        return HttpResponse::BadRequest().json("Password is required");
    }

    let user = match pool
        .services
        .user_service
        .find_by_username(&login_request.username, &pool.database)
        .await
    {
        Ok(u) => match u {
            Some(user) => user,
            None => {
                return HttpResponse::BadRequest().finish();
            }
        },
        Err(e) => {
            error!("Failed to find user by email: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    let password_hash = match &pool
        .services
        .password_service
        .hash_password(login_request.password)
    {
        Ok(e) => e.to_string(),
        Err(e) => {
            error!("Failed to hash password: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    if password_hash != user.password || !user.enabled {
        return HttpResponse::BadRequest().finish();
    }

    match pool
        .services
        .jwt_service
        .generate_jwt_token(&user.id.to_hex())
    {
        Some(t) => HttpResponse::Ok().json(LoginResponse::new(t)),
        None => HttpResponse::InternalServerError()
            .json(InternalServerError::new("Failed to generate JWT token")),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/authentication/register/",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Authentication",
)]
#[post("/register/")]
pub async fn register(
    register_request: web::Json<RegisterRequest>,
    pool: web::Data<Config>,
) -> HttpResponse {
    let register_request = register_request.into_inner();

    if register_request.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty usernames are not allowed"));
    }

    if register_request.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty passwords are not allowed"));
    }

    let default_roles: Option<Vec<ObjectId>> = match pool
        .services
        .role_service
        .find_by_name("DEFAULT", &pool.database)
        .await
    {
        Ok(r) => match r {
            Some(role) => Some(vec![role.id]),
            None => None,
        },
        Err(e) => {
            error!("Failed to find default role: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let mut user = User::from(register_request);

    let password_hash = match pool.services.password_service.hash_password(user.password) {
        Ok(e) => e.to_string(),
        Err(e) => {
            error!("Failed to hash password: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    user.password = password_hash;
    user.roles = default_roles;

    match pool
        .services
        .user_service
        .create(
            user,
            "AUTH-RS",
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error creating User: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/authentication/current/",
    responses(
        (status = 200, description = "OK", body = SimpleUserDto),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Authentication",
    security(
        ("Token" = [])
    )
)]
#[get("/current/")]
pub async fn current_user(req: HttpRequest, pool: web::Data<Config>) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let username = match pool.services.jwt_service.verify_jwt_token(token) {
                    Ok(user) => user,
                    Err(e) => {
                        error!("Failed to verify JWT token: {}", e);
                        return HttpResponse::Forbidden().finish();
                    }
                };

                let user = match pool
                    .services
                    .user_service
                    .find_by_id(&username, &pool.database)
                    .await
                {
                    Ok(u) => match u {
                        Some(user) => user,
                        None => {
                            return HttpResponse::Forbidden().finish();
                        }
                    },
                    Err(e) => {
                        error!("Failed to find user by ID: {}", e);
                        return HttpResponse::Forbidden().finish();
                    }
                };

                if !user.enabled {
                    return HttpResponse::Forbidden().finish();
                }

                return match convert_user_to_simple_dto(user, &pool).await {
                    Ok(u) => HttpResponse::Ok().json(u),
                    Err(e) => {
                        error!("Failed to convert User to SimpleUserDto: {}", e);
                        HttpResponse::Forbidden().finish()
                    }
                };
            }
        }
    }

    HttpResponse::Forbidden().finish()
}
