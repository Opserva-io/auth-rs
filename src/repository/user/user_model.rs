use crate::web::dto::authentication::register_request::RegisterRequest;
use crate::web::dto::user::create_user::CreateUser;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub password: String,
    pub roles: Option<Vec<ObjectId>>,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
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
    ///
    /// # Example
    ///
    /// ```
    /// let user = User::new(
    ///   String::from("username"),
    ///   Some(String::from("email")),
    ///   String::from("first_name"),
    ///   String::from("last_name"),
    ///   String::from("password"),
    ///   Some(vec![String::from("role")]),
    ///   true,
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The new User.
    pub fn new(
        username: String,
        email: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        password: String,
        roles: Option<Vec<String>>,
        enabled: bool,
    ) -> User {
        let now: DateTime<Utc> = SystemTime::now().into();

        let roles: Option<Vec<ObjectId>> = match roles {
            None => None,
            Some(r) => {
                let mut oid_vec: Vec<ObjectId> = vec![];
                for role in r {
                    match ObjectId::parse_str(&role) {
                        Ok(oid) => oid_vec.push(oid),
                        Err(_) => continue,
                    }
                }
                Some(oid_vec)
            }
        };

        User {
            id: ObjectId::new(),
            username,
            email,
            first_name,
            last_name,
            password,
            roles,
            created_at: now,
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
    ///   email: Some(String::from("email")),
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

        let roles: Option<Vec<ObjectId>> = match value.roles {
            None => None,
            Some(r) => {
                let mut oid_vec: Vec<ObjectId> = vec![];
                for role in r {
                    match ObjectId::parse_str(&role) {
                        Ok(oid) => oid_vec.push(oid),
                        Err(_) => continue,
                    }
                }
                Some(oid_vec)
            }
        };

        User {
            id: ObjectId::new(),
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.password,
            roles,
            created_at: now,
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
    ///  email: Some(String::from("email")),
    ///  first_name: String::from("first_name"),
    ///  last_name: String::from("last_name"),
    ///  password: String::from("password"),
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

        User {
            id: ObjectId::new(),
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.password,
            roles: None,
            created_at: now,
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
    ///   email: Some(String::from("email")),
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
        write!(
            f,
            "User: [id: {}, username: {}, email: {}, first_name: {}, last_name: {}, password: {}, roles: {:?}, created_at: {}, updated_at: {}, enabled: {}]",
            self.id.to_hex(),
            self.username,
            match &self.email {
                None => String::from("None"),
                Some(e) => e.to_string(),
            },
            self.first_name.clone().unwrap_or(String::from("")),
            self.last_name.clone().unwrap_or(String::from("")),
            self.password,
            match &self.roles {
                None => String::from("None"),
                Some(r) => format!("{:?}", r)
            },
            self.created_at,
            self.updated_at,
            self.enabled,
        )
    }
}
