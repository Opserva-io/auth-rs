use crate::configuration::config::Config;
use crate::configuration::jwt_config::JwtConfig;
use crate::web::controller::Controller;
use actix_cors::Cors;
use actix_web::{web as a_web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod configuration;
mod errors;
mod repository;
mod services;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let addr = match env::var("SERVER_ADDR") {
        Ok(d) => d,
        Err(_) => panic!("No address specified"),
    };

    let port = match env::var("SERVER_PORT") {
        Ok(d) => {
            let res: u16 = d.trim().parse().expect("PORT must be a number");
            res
        }
        Err(_) => panic!("No port specified"),
    };

    let conn_string = match env::var("DB_CONNECTION_STRING") {
        Ok(d) => d,
        Err(_) => panic!("No connection string specified"),
    };

    let database = match env::var("DB_DATABASE") {
        Ok(d) => d,
        Err(_) => panic!("No database specified"),
    };

    let permission_collection = match env::var("DB_PERMISSION_COLLECTION") {
        Ok(d) => d,
        Err(_) => panic!("No permission collection specified"),
    };

    let role_collection = match env::var("DB_ROLE_COLLECTION") {
        Ok(d) => d,
        Err(_) => panic!("No role collection specified"),
    };

    let user_collection = match env::var("DB_USER_COLLECTION") {
        Ok(d) => d,
        Err(_) => panic!("No user collection specified"),
    };

    let salt = match env::var("HASH_SALT") {
        Ok(d) => d,
        Err(_) => panic!("No salt specified"),
    };

    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(d) => d,
        Err(_) => panic!("No JWT secret specified"),
    };

    let jwt_expiration = match env::var("JWT_EXPIRATION") {
        Ok(d) => {
            let res: usize = d.trim().parse().expect("JWT_EXPIRATION must be a number");
            res
        }
        Err(_) => panic!("No JWT expiration specified"),
    };

    let config = Config::new(
        &conn_string,
        &database,
        permission_collection,
        role_collection,
        user_collection,
        salt,
        JwtConfig::new(jwt_secret, jwt_expiration),
    )
    .await;

    HttpServer::new(move || {
        App::new()
            .app_data(a_web::Data::new(config.clone()))
            .wrap(Cors::permissive())
            .configure(Controller::configure_routes)
    })
    .bind((addr, port))?
    .run()
    .await
}
