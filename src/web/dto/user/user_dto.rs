use crate::repository::user::user_model::User;
use crate::web::dto::role::role_dto::{RoleDto, SimpleRoleDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
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
    /// # Summary
    ///
    /// Convert a User entity into a UserDto.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new(
    ///   String::from("id"),
    ///   String::from("username"),
    ///   String::from("email"),
    ///   String::from("first_name"),
    ///   String::from("last_name"),
    ///   String::from("password"),
    ///   None,
    /// );
    ///
    /// let user_dto = UserDto::from(user);
    /// ```
    ///
    /// # Returns
    ///
    /// * `UserDto` - The new UserDto.
    fn from(value: User) -> Self {
        UserDto {
            id: value.id.to_hex(),
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
    /// # Summary
    ///
    /// Convert a reference User entity into a UserDto.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new(
    ///   String::from("id"),
    ///   String::from("username"),
    ///   String::from("email"),
    ///   String::from("first_name"),
    ///   String::from("last_name"),
    ///   String::from("password"),
    ///   None,
    /// );
    ///
    /// let user_dto = UserDto::from(&user);
    /// ```
    ///
    /// # Returns
    ///
    /// * `UserDto` - The new UserDto.
    fn from(value: &User) -> Self {
        UserDto {
            id: value.id.to_hex(),
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SimpleUserDto {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub roles: Option<Vec<SimpleRoleDto>>,
}

impl From<User> for SimpleUserDto {
    /// # Summary
    ///
    /// Convert a User entity into a SimpleUserDto.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new(
    ///   String::from("id"),
    ///   String::from("username"),
    ///   String::from("email"),
    ///   String::from("first_name"),
    ///   String::from("last_name"),
    ///   String::from("password"),
    ///   None,
    /// );
    ///
    /// let simple_user_dto = SimpleUserDto::from(user);
    /// ```
    ///
    /// # Returns
    ///
    /// * `SimpleUserDto` - The new SimpleUserDto.
    fn from(value: User) -> Self {
        SimpleUserDto {
            id: value.id.to_hex(),
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            roles: None,
        }
    }
}

impl From<&User> for SimpleUserDto {
    /// # Summary
    ///
    /// Convert a reference User entity into a SimpleUserDto.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new(
    ///   String::from("id"),
    ///   String::from("username"),
    ///   String::from("email"),
    ///   String::from("first_name"),
    ///   String::from("last_name"),
    ///   String::from("password"),
    ///   None,
    /// );
    ///
    /// let simple_user_dto = SimpleUserDto::from(&user);
    /// ```
    ///
    /// # Returns
    ///
    /// * `SimpleUserDto` - The new SimpleUserDto.
    fn from(value: &User) -> Self {
        SimpleUserDto {
            id: value.id.to_hex(),
            username: value.username.clone(),
            email: value.email.clone(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            roles: None,
        }
    }
}
