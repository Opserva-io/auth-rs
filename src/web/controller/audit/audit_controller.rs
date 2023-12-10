use crate::configuration::config::Config;
use crate::errors::internal_server_error::InternalServerError;
use crate::web::dto::audit::audit_dto::AuditDto;
use crate::web::dto::search::search_request::SearchRequest;
use actix_web::{get, web, HttpResponse};
use actix_web_grants::protect;
use log::error;

#[utoipa::path(
    get,
    path = "/api/v1/audits/",
    params(
        ("text" = Option<String>, Query, description = "The text to search for", nullable = true),
        ("limit" = Option<i64>, Query, description = "The limit of audits to retrieve", nullable = true),
        ("page" = Option<i64>, Query, description = "The page", nullable = true),
    ),
    responses(
        (status = 200, description = "OK", body = Vec<AuditDto>),
        (status = 204, description = "No Content"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Audits",
    security(
        ("Token" = [])
    )
)]
#[get("/")]
#[protect("CAN_READ_AUDIT")]
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
            .audit_service
            .search(&t, limit, page, &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Error while searching for audits: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        },
        None => match pool
            .services
            .audit_service
            .find_all(limit, page, &pool.database)
            .await
        {
            Ok(d) => d,
            Err(e) => {
                error!("Error while finding all audits: {}", e);
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        },
    };

    if res.is_empty() {
        return HttpResponse::NoContent().finish();
    }

    let dto_list = res.into_iter().map(|p| p.into()).collect::<Vec<AuditDto>>();

    HttpResponse::Ok().json(dto_list)
}

#[utoipa::path(
    get,
    path = "/api/v1/audits/{id}",
    params(
        ("id" = String, Path, description = "The ID of the Audit"),
    ),
    responses(
        (status = 200, description = "OK", body = AuditDto),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error", body = InternalServerError),
    ),
    tag = "Audits",
    security(
        ("Token" = [])
    )
)]
#[get("/{id}")]
#[protect("CAN_READ_AUDIT")]
pub async fn find_by_id(path: web::Path<String>, pool: web::Data<Config>) -> HttpResponse {
    let res = match pool
        .services
        .audit_service
        .find_by_id(&path, &pool.database)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!("Error while finding Audit with ID {}: {}", path, e);
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(AuditDto::from(res))
}
