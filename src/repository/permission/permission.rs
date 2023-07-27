use crate::web::dto::permission::create_permission::CreatePermission;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Permission {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl From<CreatePermission> for Permission {
    /// # Summary
    ///
    /// Convert a CreatePermission into a Permission.
    ///
    /// # Arguments
    ///
    /// * `permission` - The CreatePermission to convert.
    ///
    /// # Example
    ///
    /// ```
    /// let create_permission = CreatePermission {
    ///    name: String::from("Permission Name"),
    ///    description: Some(String::from("Permission Description")),
    /// };
    ///
    /// let permission = Permission::from(create_permission);
    /// ```
    /// # Returns
    ///
    /// * `Permission` - The new Permission.
    fn from(permission: CreatePermission) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        Permission {
            id: uuid::Uuid::new_v4().to_string(),
            name: permission.name,
            description: permission.description,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}
