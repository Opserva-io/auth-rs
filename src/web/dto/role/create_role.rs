use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateRole {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<String>>,
}
