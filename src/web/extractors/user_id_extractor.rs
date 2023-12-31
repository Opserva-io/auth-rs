use crate::configuration::config::Config;
use actix_web::HttpRequest;
use log::error;
use mongodb::bson::oid::ObjectId;

/// # Summary
///
/// Get the User ID from the Authorization header.
///
/// # Arguments
///
/// * `req` - The HttpRequest.
/// * `config` - The Config.
///
/// # Example
///
/// ```
/// let user_id = get_user_id_from_token(&req, &config).await;
/// ```
///
/// # Returns
///
/// * `Option<String>` - The User ID.
pub async fn get_user_id_from_token(req: &HttpRequest, config: &Config) -> Option<ObjectId> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return match config.services.jwt_service.verify_jwt_token(token) {
                    Ok(subject) => {
                        return match ObjectId::parse_str(subject) {
                            Ok(e) => Some(e),
                            Err(e) => {
                                error!("Failed to parse Object ID: {}", e);
                                None
                            }
                        };
                    }
                    Err(e) => {
                        error!("Failed to verify JWT token: {}", e);
                        None
                    }
                };
            }
        }
    }

    None
}
