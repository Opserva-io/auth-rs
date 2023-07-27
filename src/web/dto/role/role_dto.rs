use crate::repository::role::role::Role;
use crate::web::dto::permission::permission_dto::PermissionDto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoleDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<PermissionDto>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl From<Role> for RoleDto {
    fn from(value: Role) -> Self {
        RoleDto {
            id: value.id,
            name: value.name,
            description: value.description,
            permissions: None,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<&Role> for RoleDto {
    fn from(value: &Role) -> Self {
        RoleDto {
            id: value.id.clone(),
            name: value.name.clone(),
            description: value.description.clone(),
            permissions: None,
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
        }
    }
}
