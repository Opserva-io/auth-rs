use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub password: String,
}
