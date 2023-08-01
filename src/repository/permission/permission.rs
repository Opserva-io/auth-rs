use crate::web::dto::permission::create_permission::CreatePermission;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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

impl Permission {
    /// # Summary
    ///
    /// Create a new Permission.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Permission.
    /// * `description` - The description of the Permission.
    ///
    /// # Example
    ///
    /// ```
    /// let permission = Permission::new(String::from("Permission Name"), Some(String::from("Permission Description")));
    /// ```
    ///
    /// # Returns
    ///
    /// * `Permission` - The new Permission.
    pub fn new(name: String, description: Option<String>) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        Permission {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            created_at: now.clone(),
            updated_at: now,
        }
    }
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

impl Display for Permission {
    /// # Summary
    ///
    /// Display the Permission.
    ///
    /// # Arguments
    ///
    /// * `f` - The Formatter.
    ///
    /// # Example
    ///
    /// ```
    /// let permission = Permission {
    ///   id: String::from("id"),
    ///   name: String::from("Permission Name"),
    ///   description: Some(String::from("Permission Description")),
    ///   created_at: String::from("2021-01-01T00:00:00.000Z"),
    ///   updated_at: String::from("2021-01-01T00:00:00.000Z"),
    /// };
    ///
    /// println!("{}", permission);
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Permission: {{ id: {}, name: {}, description: {}, created_at: {}, updated_at: {} }}",
            self.id,
            self.name,
            self.description.as_ref().unwrap_or(&String::from("None")),
            self.created_at,
            self.updated_at
        )
    }
}
