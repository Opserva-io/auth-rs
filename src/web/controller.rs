use crate::web::controller::authentication::authentication_controller;
use crate::web::controller::health::health_controller;
use crate::web::controller::permission::permission_controller;
use crate::web::controller::role::role_controller;
use crate::web::controller::user::user_controller;
use actix_web::web;

pub mod audit;
pub mod authentication;
pub mod health;
pub mod permission;
pub mod role;
pub mod user;

pub struct Controller {}

impl Controller {
    /// # Summary
    ///
    /// Configure the routes for the web server.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The web server configuration.
    pub fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/api/v1")
                .service(
                    web::scope("/permissions")
                        .service(permission_controller::create_permission)
                        .service(permission_controller::find_all_permissions)
                        .service(permission_controller::find_by_id)
                        .service(permission_controller::update_permission)
                        .service(permission_controller::delete_permission),
                )
                .service(
                    web::scope("/roles")
                        .service(role_controller::create)
                        .service(role_controller::find_all_roles)
                        .service(role_controller::find_by_id)
                        .service(role_controller::update)
                        .service(role_controller::delete),
                )
                .service(
                    web::scope("/users")
                        .service(user_controller::create)
                        .service(user_controller::find_all)
                        .service(user_controller::find_by_id)
                        .service(user_controller::update)
                        .service(user_controller::update_password)
                        .service(user_controller::admin_update_password)
                        .service(user_controller::delete)
                        .service(user_controller::delete_self),
                )
                .service(
                    web::scope("/authentication")
                        .service(authentication_controller::login)
                        .service(authentication_controller::current_user)
                        .service(authentication_controller::register),
                )
                .service(
                    web::scope("/audits")
                        .service(audit::audit_controller::find_all)
                        .service(audit::audit_controller::find_by_id),
                ),
        );

        cfg.service(web::scope("/health").service(health_controller::health));
    }
}
