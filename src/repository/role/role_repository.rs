use crate::repository::audit::audit_repository::Error as AuditError;
use crate::repository::role::role_model::Role;
use crate::repository::user::user_repository::Error as UserError;
use crate::services::user::user_service::UserService;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use mongodb::error::Error as MongoError;
use mongodb::options::FindOptions;
use mongodb::Database;
use std::fmt;
use std::time::SystemTime;

#[derive(Clone)]
pub struct RoleRepository {
    pub collection: String,
}

#[derive(Clone, Debug)]
pub enum Error {
    InvalidId(String),
    EmptyCollection,
    EmptyId,
    EmptyName,
    EmptyTextSearch,
    NameAlreadyTaken,
    RoleNotFound(String),
    MongoDb(MongoError),
    User(UserError),
    Audit(AuditError),
}

impl fmt::Display for Error {
    /// # Summary
    ///
    /// Formats the value using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - A reference to a formatter.
    ///
    /// # Returns
    ///
    /// A fmt::Result.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::InvalidId(id) => write!(f, "Invalid Role ID: {}", id),
            Error::EmptyCollection => write!(f, "Empty collection"),
            Error::EmptyId => write!(f, "Empty Role ID"),
            Error::EmptyName => write!(f, "Empty Role name"),
            Error::EmptyTextSearch => write!(f, "Empty text search"),
            Error::NameAlreadyTaken => write!(f, "Role name already taken"),
            Error::RoleNotFound(id) => write!(f, "Role not found: {}", id),
            Error::MongoDb(e) => write!(f, "MongoDB error: {}", e),
            Error::User(e) => write!(f, "User error: {}", e),
            Error::Audit(e) => write!(f, "Audit error: {}", e),
        }
    }
}

impl RoleRepository {
    /// # Summary
    ///
    /// Creates a new RoleRepository instance.
    ///
    /// # Arguments
    ///
    /// * `collection` - A String that holds the collection name.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///    Ok(d) => d,
    ///    Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A RoleRepository instance.
    pub fn new(collection: String) -> Result<RoleRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(RoleRepository { collection })
    }

    /// # Summary
    ///
    /// Create a new role.
    ///
    /// # Arguments
    ///
    /// * `role` - A Role instance.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role = Role {
    ///    id: "unique id".to_string(),
    ///    name: "Administrator".to_string(),
    ///    description: "Administrator role".to_string(),
    ///    permissions: vec!["permission id".to_string()],
    ///    created_at: SystemTime::now(),
    ///    updated_at: SystemTime::now(),
    /// };
    ///
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// let role = match role_repository.create(role, &db).await {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to create Role: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with the created Role instance or an Error.
    pub async fn create(&self, role: Role, db: &Database) -> Result<Role, Error> {
        // Check if the name is already taken
        match self.find_by_name(&role.name.to_lowercase(), db).await {
            Ok(r) => {
                if r.is_some() {
                    return Err(Error::NameAlreadyTaken);
                }
            }
            Err(e) => return Err(e),
        }

        let role_id = role.id.to_hex();

        match db
            .collection::<Role>(&self.collection)
            .insert_one(role, None)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(Error::MongoDb(e)),
        };

        let r = self.find_by_id(&role_id, db).await;
        match r {
            Ok(r) => {
                if r.is_some() {
                    Ok(r.unwrap())
                } else {
                    Err(Error::RoleNotFound(role_id))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Summary
    ///
    /// Find all roles.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional i64 that holds the limit.
    /// * `page` - An optional i64 that holds the page.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// let roles = match role_repository.find_all(Some(10), Some(1), &db).await {
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with a vector of Role instances or an Error.
    pub async fn find_all(
        &self,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Role>, Error> {
        let mut skip: Option<u64> = None;

        if let Some(l) = limit {
            if l > 1 {
                if let Some(p) = page {
                    if p > 1 {
                        let res = u64::try_from((p - 1) * l).unwrap_or(0);
                        skip = Some(res);
                    }
                }
            }
        }

        let find_options = FindOptions::builder().limit(limit).skip(skip).build();

        let cursor = match db
            .collection::<Role>(&self.collection)
            .find(None, find_options)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    /// # Summary
    ///
    /// Find a vector of roles by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A vector of Strings that holds the IDs.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// let roles = match role_repository.find_by_id_vec(vec!["id".to_string()], &db).await {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to find Roles by ID: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with a vector of Role instances or an Error.
    pub async fn find_by_id_vec(
        &self,
        ids: Vec<String>,
        db: &Database,
    ) -> Result<Vec<Role>, Error> {
        let mut oid_vec: Vec<ObjectId> = vec![];
        for id in ids {
            match ObjectId::parse_str(id) {
                Ok(d) => oid_vec.push(d),
                Err(e) => {
                    return Err(Error::InvalidId(e.to_string()));
                }
            };
        }

        if oid_vec.is_empty() {
            return Ok(vec![]);
        }

        let filter = doc! {
            "_id": {
                "$in": oid_vec,
            },
        };

        let cursor = match db
            .collection::<Role>(&self.collection)
            .find(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    /// # Summary
    ///
    /// Find a role by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the ID.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// let role = match role_repository.find_by_id("id", &db).await {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to find Role by ID: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with an Option of a Role instance or an Error.
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<Role>, Error> {
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

        let role = match db
            .collection::<Role>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(role)
    }

    /// # Summary
    ///
    /// Find a role by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// let role = match role_repository.find_by_name("name", &db).await {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to find Role by name: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with an Option of a Role instance or an Error.
    pub async fn find_by_name(&self, name: &str, db: &Database) -> Result<Option<Role>, Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }

        let regex_pattern = format!("^{}$", regex::escape(name));
        let re = mongodb::bson::Regex {
            pattern: regex_pattern,
            options: String::from("i"),
        };

        let filter = doc! { "name": { "$regex": Bson::RegularExpression(re) } };

        match db
            .collection::<Role>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => Ok(d),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Update a Role.
    ///
    /// # Arguments
    ///
    /// * `role` - A Role instance.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// let role = match role_repository.update(role, &db).await {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to update Role: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with a Role instance or an Error.
    pub async fn update(&self, role: Role, db: &Database) -> Result<Role, Error> {
        // Check if the name is already taken
        match self.find_by_name(&role.name.to_lowercase(), db).await {
            Ok(r) => {
                if let Some(p) = r {
                    if p.id != role.id {
                        return Err(Error::NameAlreadyTaken);
                    }
                }
            }
            Err(e) => return Err(e),
        }

        let role_id = role.id;
        let filter = doc! {
            "_id": &role_id,
        };

        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        let update = doc! {
            "$set": {
                "name": role.name,
                "description": role.description,
                "permissions": role.permissions,
                "updated_at": now,
            }
        };

        let role = match db
            .collection::<Role>(&self.collection)
            .find_one_and_update(filter, update, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        if role.is_none() {
            return Err(Error::RoleNotFound(role_id.to_hex()));
        }

        Ok(role.unwrap())
    }

    /// # Summary
    ///
    /// Delete a Role.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the ID.
    /// * `db` - A reference to a Database instance.
    /// * `user_service` - A UserService instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// match role_repository.delete("id", &db).await {
    ///   Ok(_) => (),
    ///   Err(e) => panic!("Failed to delete Role: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with an empty return value or an Error.
    pub async fn delete(
        &self,
        id: &str,
        db: &Database,
        user_service: &UserService,
    ) -> Result<(), Error> {
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
            .collection::<Role>(&self.collection)
            .delete_one(filter, None)
            .await
        {
            Ok(_) => match user_service.delete_role_from_all_users(id, db).await {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::User(e)),
            },
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Delete a permission from all roles.
    ///
    /// # Arguments
    ///
    /// * `permission_id` - A string slice that holds the permission ID.
    /// * `db` - A reference to a Database instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = match RoleRepository::new("roles".to_string()) {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
    /// };
    ///
    /// match role_repository.delete_permission_from_all_roles("permission_id", &db).await {
    ///   Ok(_) => (),
    ///   Err(e) => panic!("Failed to delete permission from all roles: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with an empty return value or an Error.
    pub async fn delete_permission_from_all_roles(
        &self,
        permission_id: &str,
        db: &Database,
    ) -> Result<(), Error> {
        if permission_id.is_empty() {
            return Err(Error::EmptyId);
        }

        let target_object_id = match ObjectId::parse_str(permission_id) {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InvalidId(e.to_string()));
            }
        };

        let filter = doc! {};

        let update = doc! {
            "$pull": {
                "permissions": target_object_id,
            }
        };

        match db
            .collection::<Role>(&self.collection)
            .update_many(filter, update, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Search for Roles.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `limit` - The limit of Roles to find.
    /// * `page` - The page of Roles to find.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new("roles".to_string()).unwrap();
    /// let roles = match role_repository.search("text", Some(10), Some(1), &db).await {
    ///   Ok(d) => d,
    ///   Err(e) => panic!("Failed to search for Roles: {:?}", e),
    /// };
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Role>, Error>` - The result of the operation.
    pub async fn search(
        &self,
        text: &str,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Role>, Error> {
        if text.is_empty() {
            return Err(Error::EmptyTextSearch);
        }

        let mut skip: Option<u64> = None;

        if let Some(l) = limit {
            if l > 1 {
                if let Some(p) = page {
                    if p > 1 {
                        let res = u64::try_from((p - 1) * l).unwrap_or(0);
                        skip = Some(res);
                    }
                }
            }
        }

        let find_options = FindOptions::builder().limit(limit).skip(skip).build();

        let filter = doc! {
            "$text": {
                "$search": text,
            },
        };

        let cursor = match db
            .collection::<Role>(&self.collection)
            .find(filter, find_options)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
