use crate::configuration::config::Config;
use crate::errors::bad_request::BadRequest;
use crate::errors::internal_server_error::InternalServerError;
use crate::repository::permission::permission::Permission;
use crate::repository::permission::permission_repository::Error;
use crate::web::dto::permission::create_permission::CreatePermission;
use crate::web::dto::permission::permission_dto::PermissionDto;
use crate::web::dto::permission::update_permission::UpdatePermission;
use crate::web::dto::search::search_request::SearchRequest;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use log::error;

#[post("/")]
pub async fn create_permission(
    pool: web::Data<Config>,
    info: web::Json<CreatePermission>,
) -> HttpResponse {
    if info.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Empty name"));
    }

    let new_permission = Permission::from(info.into_inner());

    let res = match pool
        .services
        .permission_service
        .create(new_permission, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            error!("Error while creating Permission: {}", e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[get("/")]
#[has_permissions("CAN_READ_PERMISSION")]
pub async fn find_all_permissions(
    search: web::Query<SearchRequest>,
    pool: web::Data<Config>,
) -> HttpResponse {
    let res;

    if search.text.is_none() {
        res = match pool
            .services
            .permission_service
            .find_all(&pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Error while finding all permissions: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    } else {
        res = match pool
            .services
            .permission_service
            .search(&search.text.clone().unwrap(), &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Error while searching for permissions: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    }

    let dto_list = res
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<PermissionDto>>();

    HttpResponse::Ok().json(dto_list)
}

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

#[put("/{id}")]
#[has_permissions("CAN_UPDATE_PERMISSION")]
pub async fn update_permission(
    path: web::Path<String>,
    update: web::Json<UpdatePermission>,
    pool: web::Data<Config>,
) -> HttpResponse {
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

    let res = pool
        .services
        .permission_service
        .update(permission, &pool.database)
        .await;
    let res = match res {
        Ok(p) => p,
        Err(e) => {
            error!("Error while updating Permission with ID {}: {}", path, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[delete("/{id}")]
#[has_permissions("CAN_DELETE_PERMISSION")]
pub async fn delete_permission(path: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    let res = pool
        .services
        .permission_service
        .delete(&path, &pool.database, &pool.services.role_service)
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
