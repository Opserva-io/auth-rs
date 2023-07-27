use crate::configuration::config::Config;
use crate::errors::internal_server_error::InternalServerError;
use crate::web::dto::permission::permission_dto::PermissionDto;
use actix_web::{get, web, HttpResponse, post, put, delete};
use crate::errors::bad_request::BadRequest;
use crate::repository::permission::permission::Permission;
use crate::repository::permission::permission_repository::Error;
use crate::web::dto::permission::create_permission::CreatePermission;
use crate::web::dto::permission::update_permission::UpdatePermission;

#[post("/")]
pub async fn create_permission(pool: web::Data<Config>, info: web::Json<CreatePermission>) -> HttpResponse {
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
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[get("/")]
pub async fn find_all_permissions(pool: web::Data<Config>) -> HttpResponse {
    let res = match pool
        .services
        .permission_service
        .find_all(&pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let dto_list = res
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<PermissionDto>>();

    HttpResponse::Ok().json(dto_list)
}

#[get("/{id}")]
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
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[put("/{id}")]
pub async fn update_permission(path: web::Path<String>, update: web::Json<UpdatePermission>, pool: web::Data<Config>) -> HttpResponse {
    let res = pool.services.permission_service.find_by_id(&path, &pool.database).await;
    let mut permission = match res {
        Ok(p) => {
            if p.is_none() {
                return HttpResponse::NotFound().finish();
            }

            p.unwrap()
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let update = update.into_inner();

    permission.name = update.name;
    permission.description = update.description;

    let res = pool.services.permission_service.update(permission, &pool.database).await;
    let res = match res {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(PermissionDto::from(res))
}

#[delete("/{id}")]
pub async fn delete_permission(path: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    let res = pool.services.permission_service.delete(&path, &pool.database).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            match e {
                Error::PermissionNotFound(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string())),
            }
        }
    }
}
