use crate::configuration::config::Config;
use crate::errors::bad_request::BadRequest;
use crate::errors::internal_server_error::InternalServerError;
use crate::repository::permission::permission_model::Permission;
use crate::repository::permission::permission_repository::Error;
use crate::web::dto::permission::create_permission::CreatePermission;
use crate::web::dto::permission::permission_dto::PermissionDto;
use crate::web::dto::permission::update_permission::UpdatePermission;
use crate::web::dto::search::search_request::SearchRequest;
use crate::web::extractors::user_id_extractor;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use log::error;

#[utoipa::path(
    post,
    path = "/api/v1/permissions/",
    request_body = CreatePermission,
    responses(
        (status = 200, description = "OK", body = PermissionDto),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Permissions",
    security(
        ("Token" = [])
    )
)]
#[post("/")]
#[has_permissions("CAN_CREATE_PERMISSION")]
pub async fn create_permission(
    pool: web::Data<Config>,
    info: web::Json<CreatePermission>,
    req: HttpRequest,
) -> HttpResponse {
    if info.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty name"));
    }

    let new_permission = Permission::from(info.into_inner());

    let user_id = match user_id_extractor::get_user_id_from_token(&req, &pool).await {
        Some(e) => e,
        None => {
            error!("Failed to get User ID from token");
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to get User ID from token"));
        }
    };

    let res = match pool
        .services
        .permission_service
        .create(
            new_permission,
            &user_id,
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error while creating Permission: {}", e);
            return match e {
                Error::NameAlreadyTaken => {
                    HttpResponse::BadRequest().json(BadRequest::new(&e.to_string()))
                }
                _ => HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string())),
            };
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions/",
    params(
        ("text" = Option<String>, Query, description = "The text to search for", nullable = true),
        ("limit" = Option<i64>, Query, description = "The limit of permissions to retrieve", nullable = true),
        ("page" = Option<i64>, Query, description = "The page", nullable = true),
    ),
    responses(
        (status = 200, description = "OK", body = Vec<PermissionDto>),
        (status = 204, description = "No Content"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Permissions",
    security(
        ("Token" = [])
    )
)]
#[get("/")]
#[has_permissions("CAN_READ_PERMISSION")]
pub async fn find_all_permissions(
    search: web::Query<SearchRequest>,
    pool: web::Data<Config>,
) -> HttpResponse {
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
        Some(t) => {
            match pool
                .services
                .permission_service
                .search(&t, limit, page, &pool.database)
                .await
            {
                Ok(d) => d,
                Err(e) => {
                    error!("Error while searching for permissions: {}", e);
                    return HttpResponse::InternalServerError()
                        .json(InternalServerError::new(&e.to_string()));
                }
            }
        }
        None => {
            match pool
                .services
                .permission_service
                .find_all(limit, page, &pool.database)
                .await
            {
                Ok(d) => d,
                Err(e) => {
                    error!("Error while finding all permissions: {}", e);
                    return HttpResponse::InternalServerError()
                        .json(InternalServerError::new(&e.to_string()));
                }
            }
        }
    };

    if res.is_empty() {
        return HttpResponse::NoContent().finish();
    }

    let dto_list = res.iter().map(|p| p.into()).collect::<Vec<PermissionDto>>();

    HttpResponse::Ok().json(dto_list)
}

#[utoipa::path(
    get,
    path = "/api/v1/permissions/{id}",
    params(
        ("id" = String, Path, description = "The ID of the Permission"),
    ),
    responses(
        (status = 200, description = "OK", body = PermissionDto),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Permissions",
    security(
        ("Token" = [])
    )
)]
#[get("/{id}")]
#[has_permissions("CAN_READ_PERMISSION")]
pub async fn find_by_id(path: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    let res = match pool
        .services
        .permission_service
        .find_by_id(&path, &pool.database)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!("Error while finding Permission with ID {}: {}", path, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/permissions/{id}",
    request_body = UpdatePermission,
    params(
        ("id" = String, Path, description = "The ID of the Permission"),
    ),
    responses(
        (status = 200, description = "OK", body = PermissionDto),
        (status = 400, description = "Bad Request", body = BadRequest),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Permissions",
    security(
        ("Token" = [])
    )
)]
#[put("/{id}")]
#[has_permissions("CAN_UPDATE_PERMISSION")]
pub async fn update_permission(
    path: web::Path<String>,
    update: web::Json<UpdatePermission>,
    pool: web::Data<Config>,
    req: HttpRequest,
) -> HttpResponse {
    if update.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty name"));
    }

    let user_id = match user_id_extractor::get_user_id_from_token(&req, &pool).await {
        Some(e) => e,
        None => {
            error!("Failed to get User ID from token");
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to get User ID from token"));
        }
    };

    let res = pool
        .services
        .permission_service
        .find_by_id(&path, &pool.database)
        .await;
    let mut permission = match res {
        Ok(p) => {
            if p.is_none() {
                return HttpResponse::NotFound().finish();
            }

            p.unwrap()
        }
        Err(e) => {
            error!("Error while finding Permission with ID {}: {}", path, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let update = update.into_inner();

    permission.name = update.name;
    permission.description = update.description;

    match pool
        .services
        .permission_service
        .update(
            permission,
            &user_id,
            &pool.database,
            &pool.services.audit_service,
        )
        .await
    {
        Ok(p) => HttpResponse::Ok().json(PermissionDto::from(p)),
        Err(e) => {
            error!("Error while updating Permission with ID {}: {}", path, e);
            match e {
                Error::NameAlreadyTaken => {
                    HttpResponse::BadRequest().json(BadRequest::new(&e.to_string()))
                }
                _ => HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string())),
            }
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/permissions/{id}",
    params(
        ("id" = String, Path, description = "The ID of the Permission"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Permissions",
    security(
        ("Token" = [])
    )
)]
#[delete("/{id}")]
#[has_permissions("CAN_DELETE_PERMISSION")]
pub async fn delete_permission(
    path: web::Path<String>,
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
    let res = pool
        .services
        .permission_service
        .delete(
            &path,
            &user_id,
            &pool.database,
            &pool.services.role_service,
            &pool.services.audit_service,
        )
        .await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            Error::PermissionNotFound(_) => HttpResponse::NotFound().finish(),
            _ => {
                error!("Error while deleting Permission with ID {}: {}", path, e);
                HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
            }
        },
    }
}
