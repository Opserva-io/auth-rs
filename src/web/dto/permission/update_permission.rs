use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdatePermission {
    pub name: String,
    pub description: Option<String>,
}
