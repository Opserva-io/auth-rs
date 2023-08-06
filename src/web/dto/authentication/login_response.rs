use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

impl LoginResponse {
    /// # Summary
    ///
    /// Create a new LoginResponse.
    ///
    /// # Arguments
    ///
    /// * `token` - The token of the LoginResponse.
    ///
    /// # Example
    ///
    /// ```
    /// let login_response = LoginResponse::new(String::from("token"));
    /// ```
    ///
    /// # Returns
    ///
    /// * `LoginResponse` - The new LoginResponse.
    pub fn new(token: String) -> LoginResponse {
        LoginResponse { token }
    }
}
