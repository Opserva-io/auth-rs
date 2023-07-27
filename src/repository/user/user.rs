use crate::web::dto::user::create_user::CreateUser;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

impl From<CreateUser> for User {
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
