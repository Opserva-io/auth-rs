use crate::web::dto::role::create_role::CreateRole;
use crate::web::dto::role::role_dto::RoleDto;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Option<Vec<ObjectId>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Role {
    /// # Summary
    ///
    /// Create a new Role.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Role.
    /// * `description` - The description of the Role.
    /// * `permissions` - The Permissions of the Role.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role::new(String::from("Role Name"), Some(String::from("Role Description")), Some(vec![ObjectId::new()]));
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The new Role.
    pub fn new(
        name: String,
        description: Option<String>,
        permissions: Option<Vec<ObjectId>>,
    ) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        Role {
            id: ObjectId::new(),
            name,
            description,
            permissions,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl From<RoleDto> for Role {
    /// # Summary
    ///
    /// Convert a RoleDto into a Role without the Permission entities.
    ///
    /// # Arguments
    ///
    /// * `role_dto` - The RoleDto to convert.
    ///
    /// # Example
    ///
    /// ```
    /// let role_dto = RoleDto {
    ///   id: String::from("id"),
    ///   name: String::from("name"),
    ///   description: Some(String::from("description")),
    ///   permissions: Some(vec![String::from("permission")]),
    ///   created_at: String::from("created_at"),
    ///   updated_at: String::from("updated_at"),
    /// };
    ///
    /// let role = Role::from(role_dto);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The new Role.
    fn from(role_dto: RoleDto) -> Self {
        let id = ObjectId::parse_str(&role_dto.id).unwrap();

        Role {
            id,
            name: role_dto.name,
            description: role_dto.description,
            permissions: None,
            created_at: role_dto.created_at,
            updated_at: role_dto.updated_at,
        }
    }
}

impl From<CreateRole> for Role {
    /// # Summary
    ///
    /// Convert a CreateRole into a Role without the Permission entities.
    ///
    /// # Arguments
    ///
    /// * `create_role` - The CreateRole to convert.
    ///
    /// # Example
    ///
    /// ```
    /// let create_role = CreateRole {
    ///   name: String::from("name"),
    ///   description: Some(String::from("description")),
    ///   permissions: Some(vec![String::from("permission")]),
    /// };
    ///
    /// let role = Role::from(create_role);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The new Role.
    fn from(create_role: CreateRole) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        let permissions = match create_role.permissions {
            None => None,
            Some(p) => {
                let mut oid_vec: Vec<ObjectId> = vec![];
                for oid in p {
                    match ObjectId::parse_str(&oid) {
                        Ok(d) => oid_vec.push(d),
                        Err(_) => continue,
                    }
                }
                Some(oid_vec)
            }
        };

        Role {
            id: ObjectId::new(),
            name: create_role.name,
            description: create_role.description,
            permissions,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl Display for Role {
    /// # Summary
    ///
    /// Display the Role.
    ///
    /// # Arguments
    ///
    /// * `f` - The Formatter.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role {
    ///   id: String::from("id"),
    ///   name: String::from("name"),
    ///   description: Some(String::from("description")),
    ///   permissions: Some(vec![String::from("permission")]),
    ///   created_at: String::from("created_at"),
    ///   updated_at: String::from("updated_at"),
    /// };
    /// println!("{}", role);
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the display.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Role: {{ id: {}, name: {}, description: {}, permissions: {:?}, created_at: {}, updated_at: {} }}", self.id, self.name, self.description.as_ref().unwrap_or(&String::from("None")), self.permissions.as_ref().unwrap_or(&vec![]), self.created_at, self.updated_at)
    }
}
