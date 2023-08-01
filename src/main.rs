use crate::components::env_reader::EnvReader;
use crate::web::controller::Controller;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web as a_web, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use dotenvy::dotenv;
use env_logger::Env;
use log::info;

mod components;
mod configuration;
mod errors;
mod repository;
mod services;
mod web;

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
    })
    .bind((addr, port))?
    .run()
    .await
}
