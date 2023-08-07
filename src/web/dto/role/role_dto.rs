use crate::repository::role::role_model::Role;
use crate::web::dto::permission::permission_dto::{PermissionDto, SimplePermissionDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
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
    /// # Summary
    ///
    /// Convert a Role entity into a RoleDto.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role::new(
    ///   String::from("id"),
    ///   String::from("name"),
    ///   Some(String::from("description")),
    /// );
    ///
    /// let role_dto = RoleDto::from(role);
    /// ```
    ///
    /// # Returns
    ///
    /// * `RoleDto` - The new RoleDto.
    fn from(value: Role) -> Self {
        RoleDto {
            id: value.id.to_hex(),
            name: value.name,
            description: value.description,
            permissions: None,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<&Role> for RoleDto {
    /// # Summary
    ///
    /// Convert a reference Role entity into a RoleDto.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role::new(
    ///   String::from("id"),
    ///   String::from("name"),
    ///   Some(String::from("description")),
    /// );
    ///
    /// let role_dto = RoleDto::from(&role);
    /// ```
    ///
    /// # Returns
    ///
    /// * `RoleDto` - The new RoleDto.
    fn from(value: &Role) -> Self {
        RoleDto {
            id: value.id.to_hex(),
            name: value.name.clone(),
            description: value.description.clone(),
            permissions: None,
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SimpleRoleDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<SimplePermissionDto>>,
}

impl From<Role> for SimpleRoleDto {
    /// # Summary
    ///
    /// Convert a Role entity into a SimpleRoleDto.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role::new(
    ///   String::from("id"),
    ///   String::from("name"),
    ///   Some(String::from("description")),
    /// );
    ///
    /// let role_dto = SimpleRoleDto::from(role);
    /// ```
    ///
    /// # Returns
    ///
    /// * `SimpleRoleDto` - The new SimpleRoleDto.
    fn from(value: Role) -> Self {
        SimpleRoleDto {
            id: value.id.to_hex(),
            name: value.name,
            description: value.description,
            permissions: None,
        }
    }
}

impl From<&Role> for SimpleRoleDto {
    /// # Summary
    ///
    /// Convert a reference Role entity into a SimpleRoleDto.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role::new(
    ///   String::from("id"),
    ///   String::from("name"),
    ///   Some(String::from("description")),
    /// );
    ///
    /// let role_dto = SimpleRoleDto::from(&role);
    /// ```
    ///
    /// # Returns
    ///
    /// * `SimpleRoleDto` - The new SimpleRoleDto.
    fn from(value: &Role) -> Self {
        SimpleRoleDto {
            id: value.id.to_hex(),
            name: value.name.clone(),
            description: value.description.clone(),
            permissions: None,
        }
    }
}
