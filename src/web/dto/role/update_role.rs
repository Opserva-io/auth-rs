use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateRole {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}
