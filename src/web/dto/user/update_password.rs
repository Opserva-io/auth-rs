use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdatePassword {
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AdminUpdatePassword {
    pub password: String,
}
