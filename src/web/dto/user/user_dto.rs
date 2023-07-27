use crate::repository::user::user::User;
use crate::web::dto::role::role_dto::RoleDto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub roles: Option<Vec<RoleDto>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub enabled: bool,
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        UserDto {
            id: value.id,
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            roles: None,
            created_at: value.created_at,
            updated_at: value.updated_at,
            enabled: value.enabled,
        }
    }
}

impl From<&User> for UserDto {
    fn from(value: &User) -> Self {
        UserDto {
            id: value.id.clone(),
            username: value.username.clone(),
            email: value.email.clone(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            roles: None,
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            enabled: value.enabled,
        }
    }
}
