use crate::repository::permission::permission_model::Permission;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PermissionDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl From<Permission> for PermissionDto {
    /// # Summary
    ///
    /// Convert a Permission entity into a PermissionDto.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let permission = Permission::new(
    ///    String::from("id"),
    ///    String::from("name"),
    ///    Some(String::from("description")),
    /// );
    ///
    /// let permission_dto = PermissionDto::from(permission);
    /// ```
    ///
    /// # Returns
    ///
    /// * `PermissionDto` - The new PermissionDto.
    fn from(permission: Permission) -> Self {
        PermissionDto {
            id: permission.id.to_hex(),
            name: permission.name,
            description: permission.description,
            created_at: permission.created_at.to_rfc3339(),
            updated_at: permission.updated_at.to_rfc3339(),
        }
    }
}

impl From<&Permission> for PermissionDto {
    /// # Summary
    ///
    /// Convert a reference Permission entity into a PermissionDto.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let permission = Permission::new(
    ///    String::from("id"),
    ///    String::from("name"),
    ///    Some(String::from("description")),
    /// );
    ///
    /// let permission_dto = PermissionDto::from(permission);
    /// ```
    ///
    /// # Returns
    ///
    /// * `PermissionDto` - The new PermissionDto.
    fn from(value: &Permission) -> Self {
        PermissionDto {
            id: value.id.to_hex(),
            name: value.name.clone(),
            description: value.description.clone(),
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SimplePermissionDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl From<Permission> for SimplePermissionDto {
    /// # Summary
    ///
    /// Convert a Permission entity into a SimplePermissionDto.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let permission = Permission::new(
    ///    String::from("id"),
    ///    String::from("name"),
    ///    Some(String::from("description")),
    /// );
    ///
    /// let permission_dto = SimplePermissionDto::from(permission);
    /// ```
    ///
    /// # Returns
    ///
    /// * `SimplePermissionDto` - The new SimplePermissionDto.
    fn from(permission: Permission) -> Self {
        SimplePermissionDto {
            id: permission.id.to_hex(),
            name: permission.name,
            description: permission.description,
        }
    }
}

impl From<&Permission> for SimplePermissionDto {
    /// # Summary
    ///
    /// Convert a reference Permission entity into a SimplePermissionDto.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let permission = Permission::new(
    ///   String::from("id"),
    ///   String::from("name"),
    ///   Some(String::from("description")),
    /// );
    ///
    /// let permission_dto = SimplePermissionDto::from(permission);
    /// ```
    ///
    /// # Returns
    ///
    /// * `SimplePermissionDto` - The new SimplePermissionDto.
    fn from(value: &Permission) -> Self {
        SimplePermissionDto {
            id: value.id.to_hex(),
            name: value.name.clone(),
            description: value.description.clone(),
        }
    }
}
