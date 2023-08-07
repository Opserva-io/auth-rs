use crate::repository::audit::audit_model::{Action, Audit, ResourceIdType, ResourceType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ActionDto {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ResourceIdTypeDto {
    #[serde(rename = "permissionId")]
    PermissionId,
    #[serde(rename = "permissionIdVec")]
    PermissionIdVec,
    #[serde(rename = "permissionName")]
    PermissionName,
    #[serde(rename = "permissionSearch")]
    PermissionSearch,
    #[serde(rename = "roleId")]
    RoleId,
    #[serde(rename = "roleIdVec")]
    RoleIdVec,
    #[serde(rename = "roleName")]
    RoleName,
    #[serde(rename = "roleSearch")]
    RoleSearch,
    #[serde(rename = "userId")]
    UserId,
    #[serde(rename = "userName")]
    UserName,
    #[serde(rename = "userSearch")]
    UserSearch,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ResourceTypeDto {
    #[serde(rename = "permission")]
    Permission,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "user")]
    User,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuditDto {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub action: ActionDto,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "resourceIdType")]
    pub resource_id_type: ResourceIdTypeDto,
    #[serde(rename = "resourceType")]
    pub resource_type: ResourceTypeDto,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl From<Audit> for AuditDto {
    /// # Summary
    ///
    /// Convert an Audit to an AuditDto.
    ///
    /// # Arguments
    ///
    /// * `value` - An Audit.
    ///
    /// # Returns
    ///
    /// An AuditDto.
    fn from(value: Audit) -> Self {
        let action = ActionDto::from(value.action);
        let resource_id_type = ResourceIdTypeDto::from(value.resource_id_type);
        let resource_type = ResourceTypeDto::from(value.resource_type);

        AuditDto {
            id: value.id.to_hex(),
            user_id: value.user_id,
            action,
            resource_id: value.resource_id,
            resource_id_type,
            resource_type,
            created_at: value.created_at,
        }
    }
}

impl From<ResourceType> for ResourceTypeDto {
    /// # Summary
    ///
    /// Convert a ResourceType to a ResourceTypeDto.
    ///
    /// # Arguments
    ///
    /// * `value` - A ResourceType.
    ///
    /// # Returns
    ///
    /// A ResourceTypeDto.
    fn from(value: ResourceType) -> Self {
        match value {
            ResourceType::Permission => ResourceTypeDto::Permission,
            ResourceType::Role => ResourceTypeDto::Role,
            ResourceType::User => ResourceTypeDto::User,
        }
    }
}

impl From<ResourceIdType> for ResourceIdTypeDto {
    /// # Summary
    ///
    /// Convert a ResourceIdType to a ResourceIdTypeDto.
    ///
    /// # Arguments
    ///
    /// * `value` - A ResourceIdType.
    ///
    /// # Returns
    ///
    /// A ResourceIdTypeDto.
    fn from(value: ResourceIdType) -> Self {
        match value {
            ResourceIdType::PermissionId => ResourceIdTypeDto::PermissionId,
            ResourceIdType::PermissionIdVec => ResourceIdTypeDto::PermissionIdVec,
            ResourceIdType::PermissionName => ResourceIdTypeDto::PermissionName,
            ResourceIdType::PermissionSearch => ResourceIdTypeDto::PermissionSearch,
            ResourceIdType::RoleId => ResourceIdTypeDto::RoleId,
            ResourceIdType::RoleIdVec => ResourceIdTypeDto::RoleIdVec,
            ResourceIdType::RoleName => ResourceIdTypeDto::RoleName,
            ResourceIdType::RoleSearch => ResourceIdTypeDto::RoleSearch,
            ResourceIdType::UserId => ResourceIdTypeDto::UserId,
            ResourceIdType::UserName => ResourceIdTypeDto::UserName,
            ResourceIdType::UserSearch => ResourceIdTypeDto::UserSearch,
            ResourceIdType::None => ResourceIdTypeDto::None,
        }
    }
}

impl From<Action> for ActionDto {
    /// # Summary
    ///
    /// Convert an Action to an ActionDto.
    ///
    /// # Arguments
    ///
    /// * `value` - An Action.
    ///
    /// # Returns
    ///
    /// An ActionDto.
    fn from(value: Action) -> Self {
        match value {
            Action::Create => ActionDto::Create,
            Action::Update => ActionDto::Update,
            Action::Delete => ActionDto::Delete,
        }
    }
}
