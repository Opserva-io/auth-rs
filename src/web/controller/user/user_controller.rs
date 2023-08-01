use crate::configuration::config::Config;
use crate::errors::bad_request::BadRequest;
use crate::errors::internal_server_error::InternalServerError;
use crate::repository::permission::permission_repository::Error as PermissionError;
use crate::repository::role::role_repository::Error as RoleError;
use crate::repository::user::user::User;
use crate::repository::user::user_repository::Error;
use crate::web::dto::role::role_dto::RoleDto;
use crate::web::dto::user::create_user::CreateUser;
use crate::web::dto::user::update_password::{AdminUpdatePassword, UpdatePassword};
use crate::web::dto::user::update_user::UpdateUser;
use crate::web::dto::user::user_dto::UserDto;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use log::error;
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
        let roles = match pool
            .services
            .role_service
            .find_by_id_vec(user.roles.clone().unwrap(), &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                return Err(ConvertError::RoleError(e));
            }
        };

        if !roles.is_empty() {
            let mut role_dto_list: Vec<RoleDto> = vec![];

            for r in &roles {
                let role_dto =
                    match crate::web::controller::role::role_controller::get_role_dto_from_role(
                        r.clone(),
                        pool,
                    )
                    .await
                    {
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

#[post("/")]
#[has_permissions("CAN_CREATE_USER")]
pub async fn create(user_dto: web::Json<CreateUser>, pool: web::Data<Config>) -> HttpResponse {
    if user_dto.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty usernames are not allowed"));
    }

    if user_dto.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty passwords are not allowed"));
    }

    if user_dto.email.is_empty() {
        return HttpResponse::BadRequest()
            .json(BadRequest::new("Empty email addresses are not allowed"));
    }

    let user_dto = user_dto.into_inner();

    if user_dto.roles.is_some() {
        match validate_roles(&user_dto.roles, &pool).await {
            Ok(_) => (),
            Err(e) => {
                error!("Error validating roles: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    }

    let mut user = User::from(user_dto);

    let password = &user.password.as_bytes();
    let salt = match SaltString::from_b64(&pool.salt) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating salt: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to generate salt"));
        }
    };

    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(password, &salt) {
        Ok(e) => e.to_string(),
        Err(e) => {
            error!("Error hashing password: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    user.password = password_hash;

    let res = match pool
        .services
        .user_service
        .create(user, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error creating User: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
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

#[get("/")]
#[has_permissions("CAN_READ_USER")]
pub async fn find_all(pool: web::Data<Config>) -> HttpResponse {
    let res = match pool.services.user_service.find_all(&pool.database).await {
        Ok(d) => d,
        Err(e) => {
            error!("Error finding all Users: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let mut user_dto_list: Vec<UserDto> = vec![];
    for u in &res {
        let user_dto = match convert_user_to_dto(u.clone(), &pool).await {
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

#[put("/{id}")]
#[has_permissions("CAN_UPDATE_USER")]
pub async fn update(
    id: web::Path<String>,
    user_dto: web::Json<UpdateUser>,
    pool: web::Data<Config>,
) -> HttpResponse {
    let id = id.into_inner();

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

    if user_dto.email.is_empty() {
        return HttpResponse::BadRequest()
            .json(BadRequest::new("Empty email addresses are not allowed"));
    }

    let user_dto = user_dto.into_inner();

    if user_dto.roles.is_some() {
        match validate_roles(&user_dto.roles, &pool).await {
            Ok(_) => (),
            Err(e) => {
                error!("Error validating roles: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    }

    user.username = user_dto.username;
    user.email = user_dto.email;
    user.first_name = user_dto.first_name;
    user.last_name = user_dto.last_name;
    user.roles = user_dto.roles;
    user.enabled = user_dto.enabled;

    let res = match pool
        .services
        .user_service
        .update(user, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error updating User: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
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

#[put("/{id}/password")]
#[has_permissions("CAN_UPDATE_SELF")]
pub async fn update_password(
    id: web::Path<String>,
    update_password: web::Json<UpdatePassword>,
    pool: web::Data<Config>,
) -> HttpResponse {
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

    let update_password = update_password.into_inner();
    if update_password.old_password.is_empty() {
        return HttpResponse::BadRequest()
            .json(BadRequest::new("Empty old passwords are not allowed"));
    }

    if update_password.new_password.is_empty() {
        return HttpResponse::BadRequest()
            .json(BadRequest::new("Empty new passwords are not allowed"));
    }

    let argon2 = Argon2::default();
    let salt = match SaltString::from_b64(&pool.salt) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating salt: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to generate salt"));
        }
    };

    let old_password_hash = &update_password.old_password.as_bytes();
    let old_password_hash = match argon2.hash_password(old_password_hash, &salt) {
        Ok(e) => e.to_string(),
        Err(e) => {
            error!("Error hashing password: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    if old_password_hash != user.password {
        return HttpResponse::Forbidden().finish();
    }

    let new_password_hash = &update_password.new_password.as_bytes();
    let new_password_hash = match argon2.hash_password(new_password_hash, &salt) {
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
        .update_password(&user.id, &new_password_hash, &pool.database)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error updating password: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[put("/{id}/password/admin")]
#[has_permissions("CAN_UPDATE_USER")]
pub async fn admin_update_password(
    id: web::Path<String>,
    admin_update_password: web::Json<AdminUpdatePassword>,
    pool: web::Data<Config>,
) -> HttpResponse {
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

    if admin_update_password.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty passwords are not allowed"));
    }

    let argon2 = Argon2::default();
    let salt = match SaltString::from_b64(&pool.salt) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating salt: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to generate salt"));
        }
    };

    let password_hash = &admin_update_password.password.as_bytes();
    let password_hash = match argon2.hash_password(password_hash, &salt) {
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
        .update_password(&user.id, &password_hash, &pool.database)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error updating password: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[delete("/{id}")]
#[has_permissions("CAN_UPDATE_USER")]
pub async fn delete(id: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    match pool
        .services
        .user_service
        .delete(&id.into_inner(), &pool.database)
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
