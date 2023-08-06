use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreatePermission {
    pub name: String,
    pub description: Option<String>,
}
