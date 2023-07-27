use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdatePermission {
    pub name: String,
    pub description: Option<String>,
}
