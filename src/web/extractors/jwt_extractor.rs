use crate::configuration::config::Config;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use log::error;

/// # Summary
///
/// Extract the permissions from the request.
///
/// # Arguments
///
/// * `req` - The request to extract the permissions from.
///
/// # Example
///
/// ```
/// let permissions = JwtExtractor::extract(&req).await;
/// ```
///
/// # Returns
///
/// * `Result<Vec<String>, Error>` - The permissions from the request.
pub async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let res = match req.app_data::<actix_web::web::Data<Config>>() {
        None => {
            error!("Failed to get Config from request");
            return Err(ErrorInternalServerError(
                "Failed to get Config from request",
            ));
        }
        Some(e) => e,
    };

    let mut permission_list: Vec<String> = vec![];
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match res.services.jwt_service.verify_jwt_token(token) {
                    Ok(subject) => {
                        let user = match res
                            .services
                            .user_service
                            .find_by_id(&subject, &res.database)
                            .await
                        {
                            Ok(e) => match e {
                                Some(e) => e,
                                None => {
                                    let response: Vec<String> = vec![];
                                    return Ok(response);
                                }
                            },
                            Err(e) => {
                                error!("Failed to find user by ID: {}", e);
                                return Ok(vec![]);
                            }
                        };

                        if !user.enabled {
                            let response: Vec<String> = vec![];
                            return Ok(response);
                        }

                        if user.roles.is_some() {
                            let mut role_vec: Vec<String> = vec![];
                            for r in user.roles.unwrap() {
                                role_vec.push(r.to_hex());
                            }

                            let roles = match res
                                .services
                                .role_service
                                .find_by_id_vec(role_vec, &res.database)
                                .await
                            {
                                Ok(e) => e,
                                Err(e) => {
                                    error!("Failed to find roles by id vec: {}", e);
                                    let response: Vec<String> = vec![];
                                    return Ok(response);
                                }
                            };

                            if !roles.is_empty() {
                                for r in &roles {
                                    if r.permissions.is_some() {
                                        let mut oid_vec: Vec<String> = vec![];
                                        for r in r.permissions.clone().unwrap() {
                                            oid_vec.push(r.to_hex());
                                        }
                                        let permissions = match res
                                            .services
                                            .permission_service
                                            .find_by_id_vec(oid_vec, &res.database)
                                            .await
                                        {
                                            Ok(d) => d,
                                            Err(e) => {
                                                error!(
                                                    "Failed to find permissions by id vec: {}",
                                                    e
                                                );
                                                continue;
                                            }
                                        };

                                        if !permissions.is_empty() {
                                            for p in permissions {
                                                if !permission_list.contains(&p.name) {
                                                    permission_list.push(p.name);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to verify JWT token: {}", e);
                    }
                }
            }
        }
    }

    Ok(permission_list)
}
