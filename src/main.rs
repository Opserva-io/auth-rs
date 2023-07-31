use crate::components::env_reader::EnvReader;
use crate::web::controller::Controller;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web as a_web, App, HttpServer};
use dotenvy::dotenv;

mod components;
mod configuration;
mod errors;
mod repository;
mod services;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = EnvReader::read_configuration().await;

    let addr = config.address.clone();
    let port = config.port.clone();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(a_web::Data::new(config.clone()))
            .wrap(Cors::permissive())
            .configure(Controller::configure_routes)
    })
    .bind((addr, port))?
    .run()
    .await
}
