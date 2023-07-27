use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreatePermission {
    pub name: String,
    pub description: Option<String>,
}
