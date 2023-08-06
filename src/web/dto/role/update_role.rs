use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateRole {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}
