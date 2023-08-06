use crate::configuration::config::Config;
use crate::errors::bad_request::BadRequest;
use crate::errors::internal_server_error::InternalServerError;
use crate::repository::permission::permission_repository::Error as PermissionError;
use crate::repository::role::role_model::Role;
use crate::repository::role::role_repository::Error;
use crate::web::dto::permission::permission_dto::PermissionDto;
use crate::web::dto::role::create_role::CreateRole;
use crate::web::dto::role::role_dto::RoleDto;
use crate::web::dto::role::update_role::UpdateRole;
use crate::web::dto::search::search_request::SearchRequest;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use log::error;

/// # Summary
///
/// Convert a Role into a RoleDto
///  
/// # Arguments
///
/// * `role` - A Role
/// * `config` - A reference to the Config
///
/// # Example
///
/// ```
/// let role = Role::new("role1".to_string(), None);
/// let role_dto = get_role_dto_from_role(role, &config);
/// ```
///
/// # Returns
///
/// * `Result<RoleDto, PermissionError>` - The result containing the RoleDto or the PermissionError that occurred
pub async fn get_role_dto_from_role(
    role: Role,
    config: &Config,
) -> Result<RoleDto, PermissionError> {
    let mut role_dto = RoleDto::from(role.clone());
    if role.permissions.is_some() {
        role_dto.permissions =
            match find_permission_dto_from_permissions(role.permissions.unwrap(), config).await {
                Ok(d) => d,
                Err(e) => return Err(e),
            };
    }

    Ok(role_dto)
}

/// # Summary
///
/// Find an optional vector of PermissionDto from a vector of permissions
///
/// # Arguments
///
/// * `permissions` - A vector of permissions
/// * `pool` - The actix-web shared data
///
/// # Example
///
/// ```
/// let permissions = vec!["permission1".to_string(), "permission2".to_string()];
/// let pool = web::Data::new(Config::new());
///
/// let permission_dto_list = find_permission_dto_from_permissions(permissions, &pool);
/// ```
///
/// # Returns
///
/// * `Option<Vec<PermissionDto>` - The optional vector of PermissionDto
/// * `PermissionError` - The PermissionError that occurred
pub async fn find_permission_dto_from_permissions(
    permissions: Vec<String>,
    config: &Config,
) -> Result<Option<Vec<PermissionDto>>, PermissionError> {
    let mut permission_dto_list: Vec<PermissionDto> = vec![];
    let permissions = match config
        .services
        .permission_service
        .find_by_id_vec(permissions, &config.database)
        .await
    {
        Ok(d) => d,
        Err(e) => return Err(e),
    };

    if permissions.is_empty() {
        return Ok(None);
    }

    for p in permissions {
        permission_dto_list.push(PermissionDto::from(p));
    }

    Ok(Some(permission_dto_list))
}

/// # Summary
///
/// Validate if the permissions exist in the database
///
/// # Arguments
///
/// * `permissions` - A vector of permissions
/// * `pool` - The database connection pool
///
/// # Example
///
/// ```
/// let permissions = vec!["permission1".to_string(), "permission2".to_string()];
/// let pool = web::Data::new(Config::new());
///
/// let res = validate_permissions(permissions, &pool);
/// ```
///
/// # Returns
///
/// * `Result<(), Error>` - Returns Ok if the permissions exist, otherwise returns an Error
pub async fn validate_permissions(
    permissions: Option<Vec<String>>,
    pool: &web::Data<Config>,
) -> Result<(), PermissionError> {
    if permissions.is_none() {
        return Ok(());
    }

    let permissions = permissions.unwrap();

    for p in permissions {
        match pool
            .services
            .permission_service
            .find_by_id(&p, &pool.database)
            .await
        {
            Ok(d) => match d {
                Some(_) => {}
                None => {
                    return Err(PermissionError::PermissionNotFound(p));
                }
            },
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v1/roles/",
    request_body = CreateRole,
    responses(
        (status = 200, description = "OK", body = RoleDto),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Roles",
    security(
        ("Token" = [])
    )
)]
#[post("/")]
#[has_permissions("CAN_CREATE_ROLE")]
pub async fn create(role_dto: web::Json<CreateRole>, pool: web::Data<Config>) -> HttpResponse {
    if role_dto.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty name"));
    }

    let role_dto = role_dto.into_inner();
    if role_dto.permissions.is_some() {
        match validate_permissions(role_dto.permissions.clone(), &pool).await {
            Ok(_) => (),
            Err(e) => {
                error!("Error validating permissions: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    }

    let role = Role::from(role_dto);

    let res = match pool
        .services
        .role_service
        .create(role, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error creating Role: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match get_role_dto_from_role(res, &pool).await {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            error!("Error converting Role to RoleDto: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/",
    params(
        ("text" = Option<String>, Query, description = "The text to search for", nullable = true),
    ),
    responses(
        (status = 200, description = "OK", body = Vec<RoleDto>),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Roles",
    security(
        ("Token" = [])
    )
)]
#[get("/")]
#[has_permissions("CAN_READ_ROLE")]
pub async fn find_all_roles(
    search: web::Query<SearchRequest>,
    pool: web::Data<Config>,
) -> HttpResponse {
    let res = match search.text.clone() {
        Some(t) => match pool.services.role_service.search(&t, &pool.database).await {
            Ok(d) => d,
            Err(e) => {
                error!("Error while searching for Roles: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        },
        None => match pool.services.role_service.find_all(&pool.database).await {
            Ok(d) => d,
            Err(e) => {
                error!("Error while finding all Roles: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        },
    };

    let mut role_dto_list: Vec<RoleDto> = vec![];
    for r in &res {
        let role_dto = match get_role_dto_from_role(r.clone(), &pool).await {
            Ok(d) => d,
            Err(e) => {
                error!("Error converting Role to RoleDto: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };

        role_dto_list.push(role_dto);
    }

    HttpResponse::Ok().json(role_dto_list)
}

#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    params(
        ("id" = String, Path, description = "The ID of the Role"),
    ),
    responses(
        (status = 200, description = "OK", body = RoleDto),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Roles",
    security(
        ("Token" = [])
    )
)]
#[get("/{id}")]
#[has_permissions("CAN_READ_ROLE")]
pub async fn find_by_id(path: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    let res = match pool
        .services
        .role_service
        .find_by_id(&path, &pool.database)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!("Error finding Role by ID {}: {}", path, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match get_role_dto_from_role(res, &pool).await {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            error!("Error converting Role to RoleDto: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    request_body = UpdateRole,
    params(
        ("id" = String, Path, description = "The ID of the Role"),
    ),
    responses(
        (status = 200, description = "OK", body = RoleDto),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Roles",
    security(
        ("Token" = [])
    )
)]
#[put("/{id}")]
#[has_permissions("CAN_UPDATE_ROLE")]
pub async fn update(
    path: web::Path<String>,
    update: web::Json<UpdateRole>,
    pool: web::Data<Config>,
) -> HttpResponse {
    let update = update.into_inner();

    if update.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty name"));
    }

    let mut role = match pool
        .services
        .role_service
        .find_by_id(&path, &pool.database)
        .await
    {
        Ok(data) => match data {
            Some(d) => d,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!("Error finding Role by ID {}: {}", path, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if update.permissions.is_some() {
        match validate_permissions(update.permissions.clone(), &pool).await {
            Ok(_) => (),
            Err(e) => {
                error!("Error validating permissions: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    }

    role.name = update.name;
    role.description = update.description;
    role.permissions = update.permissions;

    let res = match pool
        .services
        .role_service
        .update(role, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error updating Role: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match get_role_dto_from_role(res, &pool).await {
        Ok(dto) => HttpResponse::Ok().json(dto),
        Err(e) => {
            error!("Error converting Role to RoleDto: {}", e);
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    params(
        ("id" = String, Path, description = "The ID of the Role"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Roles",
    security(
        ("Token" = [])
    )
)]
#[delete("/{id}")]
#[has_permissions("CAN_DELETE_ROLE")]
pub async fn delete(path: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    match pool
        .services
        .role_service
        .delete(&path, &pool.database, &pool.services.user_service)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            Error::RoleNotFound(_) => HttpResponse::NotFound().finish(),
            _ => {
                error!("Error deleting Role: {}", e);
                HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
            }
        },
    }
}
