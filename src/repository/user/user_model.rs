use crate::web::dto::authentication::register_request::RegisterRequest;
use crate::web::dto::user::create_user::CreateUser;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub password: String,
    pub roles: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub enabled: bool,
}

impl User {
    /// # Summary
    ///
    /// Create a new User.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the User.
    /// * `email` - The email of the User.
    /// * `first_name` - The first name of the User.
    /// * `last_name` - The last name of the User.
    /// * `password` - The password of the User.
    /// * `roles` - The roles of the User.
    /// * `enabled` - The enabled of the User.
    /// * `salt` - The salt of the User.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new(String::from("User Id"), String::from("User Username"), String::from("User Email"), String::from("User First Name"), String::from("User Last Name"), String::from("User Password"), Some(vec![String::from("Role Id")]), String::from("User Created At"), String::from("User Updated At"), true);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The new User.
    pub fn new(
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        password: String,
        roles: Option<Vec<String>>,
        enabled: bool,
    ) -> User {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        User {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            email,
            first_name,
            last_name,
            password,
            roles,
            created_at: now.clone(),
            updated_at: now,
            enabled,
        }
    }
}

impl From<CreateUser> for User {
    /// # Summary
    ///
    /// Convert a CreateUser into a User.
    ///
    /// # Arguments
    ///
    /// * `value` - The CreateUser to convert.
    ///
    /// # Example
    ///
    /// ```
    /// let create_user = CreateUser {
    ///   username: String::from("username"),
    ///   email: String::from("email"),
    ///   first_name: String::from("first_name"),
    ///   last_name: String::from("last_name"),
    ///   password: String::from("password"),
    ///   roles: Some(vec![String::from("role")]),
    /// };
    ///
    /// let user = User::from(create_user);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The new User.
    fn from(value: CreateUser) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        User {
            id: uuid::Uuid::new_v4().to_string(),
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.password,
            roles: value.roles,
            created_at: now.clone(),
            updated_at: now,
            enabled: true,
        }
    }
}

impl From<RegisterRequest> for User {
    /// # Summary
    ///
    /// Convert a RegisterRequest into a User.
    ///
    /// # Arguments
    ///
    /// * `value` - The RegisterRequest to convert.
    ///
    /// # Example
    ///
    /// ```
    /// let register_request = RegisterRequest {
    ///  username: String::from("username"),
    /// email: String::from("email"),
    /// first_name: String::from("first_name"),
    /// last_name: String::from("last_name"),
    /// password: String::from("password"),
    /// };
    ///
    /// let user = User::from(register_request);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The new User.
    fn from(value: RegisterRequest) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        User {
            id: uuid::Uuid::new_v4().to_string(),
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.password,
            roles: None,
            created_at: now.clone(),
            updated_at: now,
            enabled: true,
        }
    }
}

impl Display for User {
    /// # Summary
    ///
    /// Display the User.
    ///
    /// # Arguments
    ///
    /// * `f` - The Formatter.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User {
    ///   id: String::from("id"),
    ///   username: String::from("username"),
    ///   email: String::from("email"),
    ///   first_name: String::from("first_name"),
    ///   last_name: String::from("last_name"),
    ///   password: String::from("password"),
    ///   roles: Some(vec![String::from("role")]),
    ///   created_at: String::from("created_at"),
    ///   updated_at: String::from("updated_at"),
    ///   enabled: true,
    /// };
    ///
    /// println!("{}", user);
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {{ id: {}, username: {}, email: {}, first_name: {}, last_name: {}, roles: {:?}, created_at: {}, updated_at: {} }}", self.id, self.username, self.email, self.first_name, self.last_name, self.roles.as_ref().unwrap_or(&vec![]), self.created_at, self.updated_at)
    }
}
