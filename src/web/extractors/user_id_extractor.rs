use actix_web::HttpRequest;

/// # Summary
///
/// Get the User ID from the Authorization header.
///
/// # Arguments
///
/// * `req` - The HttpRequest.
///
/// # Example
///
/// ```
/// let user_id = get_user_id_from_token(&req, &pool).await;
/// ```
///
/// # Returns
///
/// * `Option<String>` - The User ID.
pub async fn get_user_id_from_token(req: &HttpRequest) -> Option<String> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    None
}
