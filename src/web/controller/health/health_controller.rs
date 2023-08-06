use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

impl HealthResponse {
    /// # Summary
    ///
    /// Create a new HealthResponse.
    ///
    /// # Arguments
    ///
    /// * `status` - The status of the application.
    ///
    /// # Returns
    ///
    /// * `HealthResponse` - The new HealthResponse.
    pub fn new(status: &str) -> Self {
        Self {
            status: status.to_string(),
        }
    }
}

#[get("/")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse::new("UP"))
}
