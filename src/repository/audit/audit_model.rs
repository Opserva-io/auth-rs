use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub enum ResourceType {
    Permission,
    Role,
    User,
}

impl Display for ResourceType {
    /// # Summary
    ///
    /// Display the ResourceType.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a Formatter.
    ///
    /// # Returns
    ///
    /// A std::fmt::Result.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Permission => write!(f, "Permission"),
            ResourceType::Role => write!(f, "Role"),
            ResourceType::User => write!(f, "User"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ResourceIdType {
    PermissionId,
    PermissionIdVec,
    PermissionName,
    PermissionSearch,
    RoleId,
    RoleIdVec,
    RoleName,
    RoleSearch,
    UserId,
    UserName,
    UserSearch,
    None,
}

impl Display for ResourceIdType {
    /// # Summary
    ///
    /// Display the ResourceIdType.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a Formatter.
    ///
    /// # Returns
    ///
    /// A std::fmt::Result.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceIdType::PermissionId => write!(f, "PermissionId"),
            ResourceIdType::PermissionIdVec => write!(f, "PermissionIdVec"),
            ResourceIdType::PermissionName => write!(f, "PermissionName"),
            ResourceIdType::PermissionSearch => write!(f, "PermissionSearch"),
            ResourceIdType::RoleId => write!(f, "RoleId"),
            ResourceIdType::RoleIdVec => write!(f, "RoleIdVec"),
            ResourceIdType::RoleName => write!(f, "RoleName"),
            ResourceIdType::RoleSearch => write!(f, "RoleSearch"),
            ResourceIdType::UserId => write!(f, "UserId"),
            ResourceIdType::UserName => write!(f, "UserName"),
            ResourceIdType::UserSearch => write!(f, "UserSearch"),
            ResourceIdType::None => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Search,
}

impl Display for Action {
    /// # Summary
    ///
    /// Display the Action.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a Formatter.
    ///
    /// # Returns
    ///
    /// A std::fmt::Result.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Create => write!(f, "Create"),
            Action::Read => write!(f, "Read"),
            Action::Update => write!(f, "Update"),
            Action::Delete => write!(f, "Delete"),
            Action::Search => write!(f, "Search"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Audit {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub action: Action,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "resourceIdType")]
    pub resource_id_type: ResourceIdType,
    #[serde(rename = "resourceType")]
    pub resource_type: ResourceType,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl Audit {
    /// # Summary
    ///
    /// Create a new Audit.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user id of the Audit.
    /// * `action` - The action of the Audit.
    /// * `resource_id` - The resource id of the Audit.
    /// * `resource_id_type` - The resource id type of the Audit.
    /// * `resource_type` - The resource type of the Audit.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::repository::audit::audit_model::{Action, Audit, ResourceIdType, ResourceType};
    /// ```
    ///
    /// # Returns
    ///
    /// * `Audit` - The new Audit.
    pub fn new(
        user_id: &str,
        action: Action,
        resource_id: &str,
        resource_id_type: ResourceIdType,
        resource_type: ResourceType,
    ) -> Audit {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        Audit {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            action,
            resource_id: resource_id.to_string(),
            resource_id_type,
            resource_type,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

impl Display for Audit {
    /// # Summary
    ///
    /// Display the Audit.
    ///
    /// # Arguments
    ///
    /// * `f` - The Formatter.
    ///
    /// # Returns
    ///
    /// A std::fmt::Result.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Audit {{ id: {}, user_id: {}, action: {}, resource_id: {}, resource_type: {}, created_at: {}, updated_at: {} }}",
            self.id, self.user_id, self.action, self.resource_id, self.resource_type, self.created_at, self.updated_at
        )
    }
}
