use crate::repository::audit::audit_repository::Error as AuditError;
use crate::repository::user::user_model::User;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson};
use mongodb::error::Error as MongoError;
use mongodb::Database;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Clone)]
pub struct UserRepository {
    pub collection: String,
    pub email_regex: Regex,
}

#[derive(Clone, Debug)]
pub enum Error {
    InvalidId(String),
    EmptyId,
    EmptyUsername,
    EmptyCollection,
    EmptyEmail,
    EmptyPassword,
    EmptyTextSearch,
    UserNotFound(String),
    UsernameAlreadyTaken,
    EmailAlreadyTaken,
    InvalidEmail(String),
    MongoDb(MongoError),
    Audit(AuditError),
}

impl Display for Error {
    /// # Summary
    ///
    /// Display the Error.
    ///
    /// # Arguments
    ///
    /// * `f` - The Formatter.
    ///
    /// # Example
    ///
    /// ```
    /// let error = Error::InvalidEmail(String::from("email"));
    /// println!("{}", error);
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::InvalidId(id) => write!(f, "Invalid User ID: {}", id),
            Error::EmptyId => write!(f, "Empty User ID"),
            Error::EmptyUsername => write!(f, "Empty username"),
            Error::EmptyCollection => write!(f, "Empty collection"),
            Error::EmptyEmail => write!(f, "Empty email"),
            Error::EmptyPassword => write!(f, "Empty password"),
            Error::EmptyTextSearch => write!(f, "Empty text search"),
            Error::UserNotFound(id) => write!(f, "User not found: {}", id),
            Error::UsernameAlreadyTaken => write!(f, "Username already taken"),
            Error::EmailAlreadyTaken => write!(f, "Email already taken"),
            Error::InvalidEmail(email) => write!(f, "Invalid email address: {}", email),
            Error::MongoDb(e) => write!(f, "MongoDB error: {}", e),
            Error::Audit(e) => write!(f, "Audit error: {}", e),
        }
    }
}

impl UserRepository {
    /// # Summary
    ///
    /// Create a new UserRepository.
    ///
    /// # Arguments
    ///
    /// * `collection` - The name of the collection.
    /// * `email_regex` - The email regex.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    /// use repository::user::user_repository::UserRepository;
    ///
    /// let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-
    /// zA-Z0-9-.]+$").unwrap();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<UserRepository, Error>` - The result of the operation.
    pub fn new(collection: String, email_regex: Regex) -> Result<UserRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(UserRepository {
            collection,
            email_regex,
        })
    }

    /// # Summary
    ///
    /// Create a new User entity.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let user = User {
    ///   id: String::from("id"),
    ///   username: String::from("username"),
    ///   email: String::from("email"),
    ///   password: String::from("password"),
    ///   created_at: Utc::now(),
    ///   updated_at: Utc::now(),
    /// };
    ///
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let user = user_repository.create(user, &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<User, Error>` - The result of the operation.
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

        let user_id = user.id.to_hex();

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

    /// # Summary
    ///
    /// Find all User entities.
    ///
    /// # Arguments
    ///
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let users = user_repository.find_all(&db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<User>, Error>` - The result of the operation.
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

    /// # Summary
    ///
    /// Find a User entity by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the User entity.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let user = user_repository.find_by_id(&String::from("id"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation.
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<User>, Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let target_object_id = match ObjectId::parse_str(id) {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InvalidId(e.to_string()));
            }
        };

        let filter = doc! {
            "_id": target_object_id,
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

    /// # Summary
    ///
    /// Find a User entity by its username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the User entity.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let user = user_repository.find_by_username(&String::from("username"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation.
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

    /// # Summary
    ///
    /// Find a User entity by its email.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the User entity.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let user = user_repository.find_by_email(&String::from("email"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation.
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

    /// # Summary
    ///
    /// Insert a User entity.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to insert.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let user = User::new(String::from("username"), String::from("email"), String::from("password"));
    ///
    /// user.first_name = String::from("first_name");
    ///
    /// let user = user_repository.update(user, &db);
    /// ```
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

        let user_id = user.id;
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
                    Err(Error::UserNotFound(user_id.to_hex()))
                }
            }
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Update the password of a User entity.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the User entity.
    /// * `password` - The new password of the User entity.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    ///
    /// user_repository.update_password(&String::from("id"), &String::from("password"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the operation.
    pub async fn update_password(
        &self,
        id: &str,
        password: &str,
        db: &Database,
    ) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        if password.is_empty() {
            return Err(Error::EmptyPassword);
        }

        let target_object_id = match ObjectId::parse_str(id) {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InvalidId(e.to_string()));
            }
        };

        let filter = doc! {
            "_id": target_object_id,
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

    /// # Summary
    ///
    /// Delete a User entity.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the User entity.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    ///
    /// user_repository.delete(&String::from("id"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the operation.
    pub async fn delete(&self, id: &str, db: &Database) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let target_object_id = match ObjectId::parse_str(id) {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InvalidId(e.to_string()));
            }
        };

        let filter = doc! {
            "_id": target_object_id,
        };

        let collection = db.collection::<User>(&self.collection);
        let result = collection.delete_one(filter, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Delete a role from all users.
    ///
    /// # Arguments
    ///
    /// * `role_id` - The id of the role.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    ///
    /// user_repository.delete_role_from_all_users(&String::from("role_id"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the operation.
    pub async fn delete_role_from_all_users(
        &self,
        role_id: &str,
        db: &Database,
    ) -> Result<(), Error> {
        if role_id.is_empty() {
            return Err(Error::EmptyId);
        }

        let target_object_id = match ObjectId::parse_str(role_id) {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InvalidId(e.to_string()));
            }
        };

        let filter = doc! {};

        let update = doc! {
            "$pull": {
                "roles": target_object_id,
            }
        };

        match db
            .collection::<User>(&self.collection)
            .update_many(filter, update, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Search for users.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    ///
    /// user_repository.search(&String::from("text"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<User>, Error>` - The result of the operation.
    pub async fn search(&self, text: &str, db: &Database) -> Result<Vec<User>, Error> {
        if text.is_empty() {
            return Err(Error::EmptyTextSearch);
        }

        let filter = doc! {
            "$text": {
                "$search": text,
            },
        };

        let cursor = match db
            .collection::<User>(&self.collection)
            .find(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
