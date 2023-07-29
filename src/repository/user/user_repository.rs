use crate::repository::user::user::User;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::{doc, Bson};
use mongodb::error::Error as MongoError;
use mongodb::Database;
use std::fmt::{Display, Formatter};
use std::time::SystemTime;
use regex::Regex;

#[derive(Clone)]
pub struct UserRepository {
    pub collection: String,
    pub email_regex: Regex,
}

#[derive(Clone, Debug)]
pub enum Error {
    EmptyId,
    EmptyUsername,
    EmptyCollection,
    EmptyEmail,
    EmptyPassword,
    UserNotFound(String),
    UsernameAlreadyTaken,
    EmailAlreadyTaken,
    InvalidEmail(String),
    MongoDb(MongoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::EmptyId => write!(f, "Empty User ID"),
            Error::EmptyUsername => write!(f, "Empty username"),
            Error::EmptyCollection => write!(f, "Empty collection"),
            Error::EmptyEmail => write!(f, "Empty email"),
            Error::EmptyPassword => write!(f, "Empty password"),
            Error::UserNotFound(id) => write!(f, "User not found: {}", id),
            Error::UsernameAlreadyTaken => write!(f, "Username already taken"),
            Error::EmailAlreadyTaken => write!(f, "Email already taken"),
            Error::InvalidEmail(email) => write!(f, "Invalid email address: {}", email),
            Error::MongoDb(e) => write!(f, "MongoDB error: {}", e),
        }
    }
}

impl UserRepository {
    pub fn new(collection: String, email_regex: Regex) -> Result<UserRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(UserRepository { collection, email_regex })
    }

    pub async fn create(&self, user: User, db: &Database) -> Result<User, Error> {
        if !&self.email_regex.is_match(&user.email) {
            return Err(Error::InvalidEmail(user.email));
        }

        match self.find_by_username(&user.username, db).await {
            Ok(user) => {
                if user.is_some() {
                    return Err(Error::UsernameAlreadyTaken);
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        match self.find_by_email(&user.email.to_lowercase(), db).await {
            Ok(user) => {
                if user.is_some() {
                    return Err(Error::EmailAlreadyTaken);
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        let user_id = user.id.clone();

        let collection = db.collection::<User>(&self.collection);
        let result = collection.insert_one(user, None).await;

        match result {
            Ok(_) => {}
            Err(e) => return Err(Error::MongoDb(e)),
        };

        match self.find_by_id(&user_id, db).await {
            Ok(user) => match user {
                Some(u) => Ok(u),
                None => Err(Error::UserNotFound(user_id)),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn find_all(&self, db: &Database) -> Result<Vec<User>, Error> {
        let cursor = match db
            .collection::<User>(&self.collection)
            .find(None, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<User>, Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let filter = doc! {
            "_id": id,
        };

        match db
            .collection::<User>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => Ok(d),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    pub async fn find_by_username(
        &self,
        username: &str,
        db: &Database,
    ) -> Result<Option<User>, Error> {
        if username.is_empty() {
            return Err(Error::EmptyUsername);
        }

        let regex_pattern = format!("^{}$", regex::escape(username));
        let re = mongodb::bson::Regex {
            pattern: regex_pattern,
            options: String::from("i"),
        };

        let filter = doc! { "username": { "$regex": Bson::RegularExpression(re) } };

        let user = match db
            .collection::<User>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str, db: &Database) -> Result<Option<User>, Error> {
        if email.is_empty() {
            return Err(Error::EmptyEmail);
        }

        let filter = doc! {
            "email": email,
        };

        let user = match db
            .collection::<User>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(user)
    }

    pub async fn update(&self, user: User, db: &Database) -> Result<User, Error> {
        if !self.email_regex.is_match(&user.email) {
            return Err(Error::InvalidEmail(user.email));
        }

        match self
            .find_by_username(&user.username.to_lowercase(), db)
            .await
        {
            Ok(u) => {
                if let Some(p) = u {
                    if p.id != user.id {
                        return Err(Error::UsernameAlreadyTaken);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        match self.find_by_email(&user.email.to_lowercase(), db).await {
            Ok(u) => {
                if let Some(p) = u {
                    if p.id != user.id {
                        return Err(Error::EmailAlreadyTaken);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        let user_id = user.id.clone();
        let filter = doc! {
            "_id": &user_id,
        };

        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        let update = doc! {
            "$set": {
                "username": &user.username,
                "email": &user.email,
                "firstName": &user.first_name,
                "lastName": &user.last_name,
                "roles": &user.roles,
                "updated_at": now,
                "enabled": user.enabled,
            },
        };

        let collection = db.collection::<User>(&self.collection);
        let result = collection.find_one_and_update(filter, update, None).await;

        match result {
            Ok(user) => {
                if let Some(u) = user {
                    Ok(u)
                } else {
                    Err(Error::UserNotFound(user_id))
                }
            }
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    pub async fn update_password(&self, id: &str, password: &str, db: &Database) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        if password.is_empty() {
            return Err(Error::EmptyPassword);
        }

        let filter = doc! {
            "_id": id,
        };

        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        let update = doc! {
            "$set": {
                "password": password,
                "updated_at": now,
            },
        };

        let collection = db.collection::<User>(&self.collection);
        let result = collection.update_one(filter, update, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    pub async fn delete(&self, id: &str, db: &Database) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let filter = doc! {
            "_id": id,
        };

        let collection = db.collection::<User>(&self.collection);
        let result = collection.delete_one(filter, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }
}
