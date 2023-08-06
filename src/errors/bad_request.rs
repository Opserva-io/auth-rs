use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::SystemTime;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct BadRequest {
    message: String,
    timestamp: String,
    #[serde(rename(serialize = "errorCode", deserialize = "errorCode"))]
    error_code: u16,
}

impl BadRequest {
    /// # Summary
    ///
    /// Create a new BadRequest.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message.
    ///
    /// # Example
    ///
    /// ```
    /// let bad_request = BadRequest::new("Bad Request");
    /// ```
    /// # Returns
    ///
    /// * `BadRequest` - The new BadRequest.
    ///
    pub fn new(message: &str) -> BadRequest {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        BadRequest {
            message: String::from(message),
            timestamp: now,
            error_code: 400,
        }
    }
}
