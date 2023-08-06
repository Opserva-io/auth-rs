use crate::components::env_reader::EnvReader;
use crate::web::controller::Controller;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web as a_web, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use dotenvy::dotenv;
use env_logger::Env;
use log::info;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod components;
mod configuration;
mod errors;
mod repository;
mod services;
mod web;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        // NOTE: we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

/// # Summary
///
/// The entry point to the application
///
/// # Description
///
/// The main function is the entry point to the application. It reads the configuration from the .env file
/// and starts the application server based on the configuration.
///
/// # Returns
///
/// Returns a Result of type std::io::Result<()>. If the server starts successfully, it returns Ok(()).
/// Otherwise, it returns an Err with an error message.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = EnvReader::read_configuration().await;

    let addr = config.address.clone();
    let port = config.port;

    info!("Starting server at {}:{}", addr, port);

    #[derive(OpenApi)]
    #[openapi(
        paths(
            web::controller::health::health_controller::health,
            web::controller::permission::permission_controller::create_permission,
            web::controller::permission::permission_controller::find_all_permissions,
            web::controller::permission::permission_controller::find_by_id,
            web::controller::permission::permission_controller::update_permission,
            web::controller::permission::permission_controller::delete_permission,
        ),
        components(
            schemas(
                errors::internal_server_error::InternalServerError,
                errors::bad_request::BadRequest,
                web::dto::permission::create_permission::CreatePermission,
                web::dto::permission::permission_dto::PermissionDto,
                web::dto::permission::update_permission::UpdatePermission,
                web::controller::health::health_controller::HealthResponse,
            )
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(GrantsMiddleware::with_extractor(
                web::extractors::jwt_extractor::extract,
            ))
            .app_data(a_web::Data::new(config.clone()))
            .wrap(Cors::permissive())
            .configure(Controller::configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind((addr, port))?
    .run()
    .await
}
