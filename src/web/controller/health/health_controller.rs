use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
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

#[utoipa::path(
    get,
    path = "/health/",
    responses(
        (status = 200, description = "OK", body = HealthResponse),
    ),
    tag = "Health",
)]
#[get("/")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse::new("UP"))
}
