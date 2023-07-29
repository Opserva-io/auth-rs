use crate::configuration::config::Config;
use crate::errors::internal_server_error::InternalServerError;
use crate::web::dto::login::login_request::LoginRequest;
use crate::web::dto::login::login_response::LoginResponse;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

#[post("/")]
pub async fn login(
    login_request: web::Json<LoginRequest>,
    pool: web::Data<Config>,
) -> HttpResponse {
    if login_request.email.is_empty() {
        return HttpResponse::BadRequest().json("Email is required");
    }
    if login_request.password.is_empty() {
        return HttpResponse::BadRequest().json("Password is required");
    }

    let user = match pool
        .services
        .user_service
        .find_by_email(&login_request.email, &pool.database)
        .await
    {
        Ok(u) => match u {
            Some(user) => user,
            None => {
                return HttpResponse::BadRequest().finish();
            }
        },
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    let salt = match SaltString::from_b64(&pool.salt) {
        Ok(s) => s,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to generate salt"));
        }
    };

    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(&login_request.password.as_bytes(), &salt) {
        Ok(e) => e.to_string(),
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Failed to hash password"));
        }
    };

    if password_hash != user.password {
        return HttpResponse::BadRequest().finish();
    }

    match pool.services.jwt_service.generate_jwt_token(&user.email) {
        Some(t) => HttpResponse::Ok().json(LoginResponse::new(t)),
        None => HttpResponse::InternalServerError()
            .json(InternalServerError::new("Failed to generate JWT token")),
    }
}

#[get("/current")]
pub async fn current_user(req: HttpRequest, pool: web::Data<Config>) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str["Bearer ".len()..].to_string();

                let username = match pool.services.jwt_service.verify_jwt_token(&token) {
                    Ok(user) => user,
                    Err(e) => {
                        return HttpResponse::Forbidden().finish();
                    }
                };

                let user = match pool
                    .services
                    .user_service
                    .find_by_email(&username, &pool.database)
                    .await
                {
                    Ok(u) => match u {
                        Some(user) => user,
                        None => {
                            return HttpResponse::Forbidden().finish();
                        }
                    },
                    Err(_) => {
                        return HttpResponse::Forbidden().finish();
                    }
                };
            }
        }
    }

    HttpResponse::Forbidden().finish()
}
