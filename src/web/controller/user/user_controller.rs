use crate::configuration::config::Config;
use crate::errors::bad_request::BadRequest;
use crate::errors::internal_server_error::InternalServerError;
use crate::repository::permission::permission_repository::Error as PermissionError;
use crate::repository::role::role_repository::Error as RoleError;
use crate::repository::user::user_model::User;
use crate::repository::user::user_repository::Error;
use crate::services::password::password_service::PasswordService;
use crate::web::controller::role::role_controller::get_role_dto_from_role;
use crate::web::dto::role::role_dto::RoleDto;
use crate::web::dto::search::search_request::SearchRequest;
use crate::web::dto::user::create_user::CreateUser;
use crate::web::dto::user::update_password::{AdminUpdatePassword, UpdatePassword};
use crate::web::dto::user::update_user::{UpdateOwnUser, UpdateUser};
use crate::web::dto::user::user_dto::UserDto;
use crate::web::extractors::user_id_extractor;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use argon2::PasswordHash;
use log::error;
use mongodb::bson::oid::ObjectId;
use std::fmt::{Display, Formatter};

pub enum ConvertError {
    RoleError(RoleError),
    PermissionError(PermissionError),
}

impl Display for ConvertError {
    /// # Summary
    ///
    /// Display the error
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the display
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::RoleError(e) => write!(f, "{}", e),
            ConvertError::PermissionError(e) => write!(f, "{}", e),
        }
    }
}

/// # Summary
///
/// Validate whether the roles exist
///
/// # Arguments
///
/// * `roles` - The roles to validate
/// * `pool` - The actix-web shared data
///
/// # Example
///
/// ```
/// let roles = vec!["role1".to_string(), "role2".to_string()];
/// let res = validate_roles(&roles, &pool);
/// ```
///
/// # Returns
///
/// * `Result<(), RoleError>` - The result containing the () or the RoleError that occurred
async fn validate_roles(roles: &Option<Vec<String>>, pool: &Config) -> Result<(), RoleError> {
    if roles.is_none() {
        return Ok(());
    }

    let roles = roles.clone().unwrap();

    for role in roles {
        let res = pool
            .services
            .role_service
            .find_by_id(&role, &pool.database)
            .await;

        match res {
            Ok(d) => {
                if d.is_none() {
                    return Err(RoleError::RoleNotFound(role));
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
    }

    Ok(())
}

/// # Summary
///
/// Convert a User to a UserDto
///
/// # Arguments
///
/// * `permissions` - The permissions to validate
/// * `pool` - The actix-web shared data
///
/// # Example
///
/// ```
/// let user = User::new("username", "password", "email");
/// let res = convert_user_to_dto(user, &pool);
/// ```
///
/// # Returns
///
/// * `Result<UserDto, ConvertError>` - The result containing the UserDto or the ConvertError that occurred
async fn convert_user_to_dto(user: User, pool: &Config) -> Result<UserDto, ConvertError> {
    let mut user_dto = UserDto::from(user.clone());

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
            let mut role_dto_list: Vec<RoleDto> = vec![];

            for r in roles {
                let role_dto = match get_role_dto_from_role(r, pool).await {
                    Ok(d) => d,
                    Err(e) => {
                        return Err(ConvertError::PermissionError(e));
                    }
                };

                role_dto_list.push(role_dto);
            }

            user_dto.roles = Some(role_dto_list);
        }
    }

    Ok(user_dto)
}

#[utoipa::path(
    post,
    path = "/api/v1/users/",
    request_body = CreateUser,
    responses(
        (status = 200, description = "OK", body = UserDto),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[post("/")]
#[has_permissions("CAN_CREATE_USER")]
pub async fn create(
    user_dto: web::Json<CreateUser>,
    pool: web::Data<Config>,
    req: HttpRequest,
) -> HttpResponse {
    if user_dto.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty usernames are not allowed"));
    }

    if user_dto.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty passwords are not allowed"));
    }

    let user_id = match user_id_extractor::get_user_id_from_token(&req, &pool).await {
        Some(e) => e,
        None => {
            error!("Failed to get User ID from token");
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to get User ID from token"));
        }
    };

    let user_dto = user_dto.into_inner();

    if user_dto.roles.is_some() {
        match validate_roles(&user_dto.roles, &pool).await {
            Ok(_) => (),
            Err(e) => {
                return match e {
                    RoleError::RoleNotFound(r) => HttpResponse::BadRequest()
                        .json(BadRequest::new(&format!("Role {} not found", r))),
                    _ => {
                        error!("Error validating roles: {}", e);
                        HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()))
                    }
                };
            }
        };
    }

    let mut user = User::from(user_dto);

    let password_hash = match PasswordService::hash_password(user.password) {
        Ok(e) => e.to_string(),
        Err(e) => {
            error!("Failed to hash password: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    user.password = password_hash;

    let res = match pool
        .services
        .user_service
        .create(
            user,
            Some(user_id),
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error creating User: {}", e);
            return match e {
                Error::UsernameAlreadyTaken | Error::EmailAlreadyTaken | Error::InvalidEmail(_) => {
                    HttpResponse::BadRequest().json(BadRequest::new(&e.to_string()))
                }
                _ => HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string())),
            };
        }
    };

    match convert_user_to_dto(res, &pool).await {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            error!("Error converting User to UserDto: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/users/",
    params(
        ("text" = Option<String>, Query, description = "The text to search for", nullable = true),
        ("limit" = Option<i64>, Query, description = "The limit of users to retrieve", nullable = true),
        ("page" = Option<i64>, Query, description = "The page", nullable = true),
    ),
    responses(
        (status = 200, description = "OK", body = Vec<UserDto>),
        (status = 204, description = "No Content"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[get("/")]
#[has_permissions("CAN_READ_USER")]
pub async fn find_all(search: web::Query<SearchRequest>, pool: web::Data<Config>) -> HttpResponse {
    let search = search.into_inner();

    let mut limit = search.limit;
    let page = search.page;

    let limit_clone = limit.unwrap_or(pool.server_config.max_limit);
    if limit.is_none()
        || (limit.is_some() && limit_clone > pool.server_config.max_limit || limit_clone < 1)
    {
        limit = Some(pool.server_config.max_limit);
    }

    let res = match search.text {
        Some(t) => match pool
            .services
            .user_service
            .search(&t, limit, page, &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Error while searching for Users: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        },
        None => match pool
            .services
            .user_service
            .find_all(limit, page, &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Error while finding all Users: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        },
    };

    if res.is_empty() {
        return HttpResponse::NoContent().finish();
    }

    let mut user_dto_list: Vec<UserDto> = vec![];
    for u in res {
        let user_dto = match convert_user_to_dto(u, &pool).await {
            Ok(d) => d,
            Err(e) => {
                error!("Error converting User to UserDto: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };

        user_dto_list.push(user_dto);
    }

    HttpResponse::Ok().json(user_dto_list)
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    responses(
        (status = 200, description = "OK", body = UserDto),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[get("/{id}")]
#[has_permissions("CAN_READ_USER")]
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    let id = id.into_inner();

    let user = match pool
        .services
        .user_service
        .find_by_id(&id, &pool.database)
        .await
    {
        Ok(d) => {
            if d.is_some() {
                d.unwrap()
            } else {
                return HttpResponse::NotFound().finish();
            }
        }
        Err(e) => {
            error!("Error finding User by ID {}: {}", id, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match convert_user_to_dto(user, &pool).await {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            error!("Error converting User to UserDto: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    request_body = UpdateUser,
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    responses(
        (status = 200, description = "OK", body = UserDto),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[put("/{id}")]
#[has_permissions("CAN_UPDATE_USER")]
pub async fn update(
    id: web::Path<String>,
    user_dto: web::Json<UpdateUser>,
    pool: web::Data<Config>,
    req: HttpRequest,
) -> HttpResponse {
    let id = id.into_inner();

    let user_id = match user_id_extractor::get_user_id_from_token(&req, &pool).await {
        Some(e) => e,
        None => {
            error!("Failed to get User ID from token");
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to get User ID from token"));
        }
    };

    let mut user = match pool
        .services
        .user_service
        .find_by_id(&id, &pool.database)
        .await
    {
        Ok(d) => {
            if d.is_some() {
                d.unwrap()
            } else {
                return HttpResponse::NotFound().finish();
            }
        }
        Err(e) => {
            error!("Error finding User by ID {}: {}", id, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if user_dto.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty usernames are not allowed"));
    }

    let user_dto = user_dto.into_inner();

    if user_dto.roles.is_some() {
        match validate_roles(&user_dto.roles, &pool).await {
            Ok(_) => (),
            Err(e) => {
                return match e {
                    RoleError::RoleNotFound(r) => HttpResponse::BadRequest()
                        .json(BadRequest::new(&format!("Role {} not found", r))),
                    _ => {
                        error!("Error validating roles: {}", e);
                        HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()))
                    }
                };
            }
        };
    }

    let role_oid_vec = match user_dto.roles {
        Some(e) => {
            let mut vec = vec![];
            for r in e {
                match ObjectId::parse_str(&r) {
                    Ok(oid) => vec.push(oid),
                    Err(e) => {
                        error!("Error parsing role ID {}: {}", r, e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()));
                    }
                };
            }
            Some(vec)
        }
        None => None,
    };

    user.username = user_dto.username;
    user.email = user_dto.email;
    user.first_name = user_dto.first_name;
    user.last_name = user_dto.last_name;
    user.roles = role_oid_vec;
    user.enabled = user_dto.enabled;

    let res = match pool
        .services
        .user_service
        .update(
            user,
            Some(user_id),
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error updating User: {}", e);
            return match e {
                Error::UsernameAlreadyTaken | Error::EmailAlreadyTaken | Error::InvalidEmail(_) => {
                    HttpResponse::BadRequest().json(BadRequest::new(&e.to_string()))
                }
                _ => HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string())),
            };
        }
    };

    match convert_user_to_dto(res, &pool).await {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            error!("Error converting User to UserDto: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/self/",
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    request_body = UpdateOwnUser,
    responses(
        (status = 200, description = "OK", body = SimpleUserDto),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[put("/{id}/self/")]
#[has_permissions("CAN_UPDATE_SELF")]
pub async fn update_self(
    req: HttpRequest,
    user_dto: web::Json<UpdateOwnUser>,
    pool: web::Data<Config>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let user_id = match ObjectId::parse_str(token) {
                    Ok(oid) => oid,
                    Err(e) => {
                        error!("Error parsing user ID {}: {}", token, e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()));
                    }
                };

                let mut user = match pool
                    .services
                    .user_service
                    .find_by_username(token, &pool.database)
                    .await
                {
                    Ok(d) => {
                        if d.is_some() {
                            d.unwrap()
                        } else {
                            return HttpResponse::NotFound().finish();
                        }
                    }
                    Err(e) => {
                        error!("Error finding User by email {}: {}", token, e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()));
                    }
                };

                if user_dto.username.is_empty() {
                    return HttpResponse::BadRequest()
                        .json(BadRequest::new("Empty usernames are not allowed"));
                }

                let user_dto = user_dto.into_inner();

                user.username = user_dto.username;
                user.email = user_dto.email;
                user.first_name = user_dto.first_name;
                user.last_name = user_dto.last_name;

                let res = match pool
                    .services
                    .user_service
                    .update(
                        user,
                        Some(user_id),
                        &pool.database,
                        &pool.services.audit_service,
                    )
                    .await
                {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Error updating User: {}", e);
                        return match e {
                            Error::UsernameAlreadyTaken
                            | Error::EmailAlreadyTaken
                            | Error::InvalidEmail(_) => {
                                HttpResponse::BadRequest().json(BadRequest::new(&e.to_string()))
                            }
                            _ => HttpResponse::InternalServerError()
                                .json(InternalServerError::new(&e.to_string())),
                        };
                    }
                };

                return match crate::web::controller::authentication::authentication_controller::convert_user_to_simple_dto(res, &pool).await {
                    Ok(dto) => {
                        HttpResponse::Ok().json(dto)
                    },
                    Err(e) => {
                        error!("Error converting User to UserDto: {}", e);
                        HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
                    }
                };
            }
        }
    }
    HttpResponse::BadRequest().finish()
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/self/password/",
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    request_body = UpdatePassword,
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[put("/{id}/self/password/")]
#[has_permissions("CAN_UPDATE_SELF")]
pub async fn update_password(
    req: HttpRequest,
    update_password: web::Json<UpdatePassword>,
    pool: web::Data<Config>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let user_oid = match ObjectId::parse_str(token) {
                    Ok(oid) => oid,
                    Err(e) => {
                        error!("Error parsing user ID {}: {}", token, e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()));
                    }
                };

                let user = match pool
                    .services
                    .user_service
                    .find_by_username(token, &pool.database)
                    .await
                {
                    Ok(d) => {
                        if d.is_some() {
                            d.unwrap()
                        } else {
                            return HttpResponse::NotFound().finish();
                        }
                    }
                    Err(e) => {
                        error!("Error finding User by email {}: {}", token, e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()));
                    }
                };

                let update_password = update_password.into_inner();

                if update_password.old_password.is_empty() {
                    return HttpResponse::BadRequest()
                        .json(BadRequest::new("Empty old passwords are not allowed"));
                }

                if update_password.new_password.is_empty() {
                    return HttpResponse::BadRequest()
                        .json(BadRequest::new("Empty new passwords are not allowed"));
                }

                let parsed_hash = match PasswordHash::new(&user.password) {
                    Ok(h) => h,
                    Err(e) => {
                        error!("Failed to parse password hash: {}", e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new("Failed to parse password hash"));
                    }
                };

                if !PasswordService::verify_password(&update_password.old_password, &parsed_hash) {
                    return HttpResponse::BadRequest().finish();
                }

                let new_password_hash =
                    match PasswordService::hash_password(update_password.new_password) {
                        Ok(e) => e.to_string(),
                        Err(e) => {
                            error!("Error hashing password: {}", e);
                            return HttpResponse::InternalServerError()
                                .json(InternalServerError::new("Failed to hash password"));
                        }
                    };

                return match pool
                    .services
                    .user_service
                    .update_password(
                        &user.id.to_hex(),
                        &new_password_hash,
                        Some(user_oid),
                        &pool.database,
                        &pool.services.audit_service,
                    )
                    .await
                {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(e) => {
                        error!("Error updating password: {}", e);
                        HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()))
                    }
                };
            }
        }
    }

    HttpResponse::BadRequest().finish()
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/password/",
    request_body = AdminUpdatePassword,
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[put("/{id}/password/")]
#[has_permissions("CAN_UPDATE_USER")]
pub async fn admin_update_password(
    id: web::Path<String>,
    admin_update_password: web::Json<AdminUpdatePassword>,
    pool: web::Data<Config>,
    req: HttpRequest,
) -> HttpResponse {
    let id = id.into_inner();
    let admin_update_password = admin_update_password.into_inner();

    let user_id = match user_id_extractor::get_user_id_from_token(&req, &pool).await {
        Some(e) => e,
        None => {
            error!("Failed to get User ID from token");
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to get User ID from token"));
        }
    };

    let user = match pool
        .services
        .user_service
        .find_by_id(&id, &pool.database)
        .await
    {
        Ok(d) => {
            if d.is_some() {
                d.unwrap()
            } else {
                return HttpResponse::NotFound().finish();
            }
        }
        Err(e) => {
            error!("Error finding User by ID {}: {}", id, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if admin_update_password.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty passwords are not allowed"));
    }

    let password_hash = match PasswordService::hash_password(admin_update_password.password) {
        Ok(e) => e.to_string(),
        Err(e) => {
            error!("Error hashing password: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    match pool
        .services
        .user_service
        .update_password(
            &user.id.to_hex(),
            &password_hash,
            Some(user_id),
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error updating password: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[delete("/{id}")]
#[has_permissions("CAN_DELETE_USER")]
pub async fn delete(
    id: web::Path<String>,
    pool: web::Data<Config>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = match user_id_extractor::get_user_id_from_token(&req, &pool).await {
        Some(e) => e,
        None => {
            error!("Failed to get User ID from token");
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to get User ID from token"));
        }
    };

    match pool
        .services
        .user_service
        .delete(
            &id.into_inner(),
            Some(user_id),
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            Error::UserNotFound(_) => HttpResponse::NotFound().finish(),
            _ => {
                error!("Error deleting User: {}", e);
                HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
            }
        },
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}/self/",
    params(
        ("id" = String, Path, description = "The ID of the User"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Users",
    security(
        ("Token" = [])
    )
)]
#[delete("/{id}/self/")]
#[has_permissions("CAN_DELETE_SELF")]
pub async fn delete_self(req: HttpRequest, pool: web::Data<Config>) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let user_oid = match ObjectId::parse_str(token) {
                    Ok(oid) => oid,
                    Err(e) => {
                        error!("Error parsing user ID {}: {}", token, e);
                        return HttpResponse::InternalServerError()
                            .json(InternalServerError::new(&e.to_string()));
                    }
                };

                let username = match pool.services.jwt_service.verify_jwt_token(token) {
                    Ok(user) => user,
                    Err(e) => {
                        error!("Failed to verify JWT token: {}", e);
                        return HttpResponse::Forbidden().finish();
                    }
                };

                return match pool
                    .services
                    .user_service
                    .delete(
                        &username,
                        Some(user_oid),
                        &pool.database,
                        &pool.services.audit_service,
                    )
                    .await
                {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(e) => match e {
                        Error::UserNotFound(_) => HttpResponse::Ok().finish(),
                        _ => {
                            error!("Error deleting User: {}", e);
                            HttpResponse::InternalServerError()
                                .json(InternalServerError::new(&e.to_string()))
                        }
                    },
                };
            }
        }
    }
    HttpResponse::BadRequest().finish()
}
