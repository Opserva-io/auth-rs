use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UpdatePassword {
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AdminUpdatePassword {
    pub password: String,
}
