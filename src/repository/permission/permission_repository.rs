use crate::repository::permission::permission_model::Permission;
use crate::repository::role::role_repository::Error as RoleError;
use crate::services::role::role_service::RoleService;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::Bson;
use mongodb::{error::Error as MongoError, Database};
use std::fmt;
use std::fmt::Debug;
use std::time::SystemTime;

#[derive(Clone)]
pub struct PermissionRepository {
    pub collection: String,
}

#[derive(Clone, Debug)]
pub enum Error {
    EmptyCollection,
    EmptyId,
    EmptyName,
    EmptyTextSearch,
    NameAlreadyTaken,
    PermissionNotFound(String),
    MongoDb(MongoError),
    Role(RoleError),
}

impl fmt::Display for Error {
    /// # Summary
    ///
    /// Display the error message.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Example
    ///
    /// ```
    /// let error = Error::EmptyCollection;
    /// println!("{}", error);
    /// ```
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - The result of the display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::EmptyCollection => write!(f, "Empty collection"),
            Error::EmptyId => write!(f, "Empty Permission ID"),
            Error::EmptyName => write!(f, "Empty Permission name"),
            Error::EmptyTextSearch => write!(f, "Empty text search"),
            Error::NameAlreadyTaken => write!(f, "Permission name already taken"),
            Error::PermissionNotFound(id) => write!(f, "Permission not found: {}", id),
            Error::MongoDb(e) => write!(f, "MongoDB error: {}", e),
            Error::Role(e) => write!(f, "Role error: {}", e),
        }
    }
}

impl PermissionRepository {
    /// # Summary
    ///
    /// Create a new PermissionRepository.
    ///
    /// # Arguments
    ///
    /// * `collection` - The name of the collection.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// ```
    ///
    /// # Returns
    ///
    /// * `PermissionRepository` - The new PermissionRepository.
    pub fn new(collection: String) -> Result<PermissionRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(PermissionRepository { collection })
    }

    /// # Summary
    ///
    /// Create a new Permission.
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission to create.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission = Permission::from(CreatePermission {
    ///    name: String::from("Permission Name"),
    ///    description: Some(String::from("Permission Description")),
    /// });
    ///
    /// let permission = permission_repository.create(permission, &db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Permission, Error>` - The result of the creation.
    pub async fn create(&self, permission: Permission, db: &Database) -> Result<Permission, Error> {
        match self.find_by_name(&permission.name.to_lowercase(), db).await {
            Ok(p) => {
                if p.is_some() {
                    return Err(Error::NameAlreadyTaken);
                }
            }
            Err(e) => return Err(e),
        };

        let permission_id = permission.id.clone();

        match db
            .collection::<Permission>(&self.collection)
            .insert_one(permission, None)
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        let r = self.find_by_id(&permission_id, db).await;
        match r {
            Ok(r) => {
                if r.is_some() {
                    Ok(r.unwrap())
                } else {
                    Err(Error::PermissionNotFound(permission_id))
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Summary
    ///
    /// Find all Permission entities
    ///
    /// # Arguments
    ///
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permissions = permission_repository.find_all(&db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Permission>, Error>` - The result of the operation.
    pub async fn find_all(&self, db: &Database) -> Result<Vec<Permission>, Error> {
        let cursor = match db
            .collection::<Permission>(&self.collection)
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
    /// Find a vector of Permissions by their ID.
    ///
    /// # Arguments
    ///
    /// * `id_vec` - The vector of IDs to find.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permissions = permission_repository.find_by_id_vec(vec![String::from("permission_id")], &db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Permission>, Error>` - The result of the operation.
    pub async fn find_by_id_vec(
        &self,
        id_vec: Vec<String>,
        db: &Database,
    ) -> Result<Vec<Permission>, Error> {
        let filter = doc! {
            "_id": {
                "$in": id_vec,
            },
        };

        let cursor = match db
            .collection::<Permission>(&self.collection)
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
    /// Find a Permission by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the Permission to find.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission = permission_repository.find_by_id(String::from("permission_id"), &db).await;
    ///
    /// match permission {
    ///    Ok(p) => println!("Permission: {:?}", p),
    ///    Err(e) => println!("Error: {:?}", e),
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<Permission>, Error>` - The result of the operation.
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<Permission>, Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let filter = doc! {
            "_id": id,
        };

        let permission = match db
            .collection::<Permission>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(permission)
    }

    /// # Summary
    ///
    /// Find a Permission by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Permission to find.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission = permission_repository.find_by_name(String::from("permission_name"), &db).await;
    ///
    /// match permission {
    ///   Ok(p) => println!("Permission: {:?}", p),
    ///   Err(e) => println!("Error: {:?}", e),
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<Permission>, Error>` - The result of the operation.
    pub async fn find_by_name(
        &self,
        name: &str,
        db: &Database,
    ) -> Result<Option<Permission>, Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }

        let regex_pattern = format!("^{}$", regex::escape(name));
        let re = mongodb::bson::Regex {
            pattern: regex_pattern,
            options: String::from("i"),
        };

        let filter = doc! { "name": { "$regex": Bson::RegularExpression(re) } };

        let permission = match db
            .collection::<Permission>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(permission)
    }

    /// # Summary
    ///
    /// Update a Permission.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission to update.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission = permission_repository.find_by_id(String::from("permission_id"), &db).await;
    ///
    /// match permission {
    ///  Ok(p) => {
    ///   let mut permission = p;
    ///   permission.name = String::from("new_permission_name");
    ///   permission_repository.update(permission, &db).await;
    ///  },
    ///  Err(e) => println!("Error: {:?}", e),
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Permission, Error>` - The result of the operation.
    pub async fn update(&self, permission: Permission, db: &Database) -> Result<Permission, Error> {
        // Check if the name is already taken
        match self.find_by_name(&permission.name.to_lowercase(), db).await {
            Ok(p) => {
                if let Some(p) = p {
                    if p.id != permission.id {
                        return Err(Error::NameAlreadyTaken);
                    }
                }
            }
            Err(e) => return Err(e),
        };

        let permission_id = permission.id.clone();
        let filter = doc! {
            "_id": &permission_id,
        };

        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        let update = doc! {
            "$set": {
                "name": permission.name,
                "description": permission.description,
                "updated_at": now,
            }
        };

        let permission = match db
            .collection::<Permission>(&self.collection)
            .find_one_and_update(filter, update, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        if permission.is_none() {
            return Err(Error::PermissionNotFound(permission_id));
        }

        Ok(permission.unwrap())
    }

    /// # Summary
    ///
    /// Delete a Permission.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the Permission to delete.
    /// * `db` - The database to use.
    /// * `role_service` - The reference RoleService to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// permission_repository.delete(String::from("permission_id"), &db, role_repository).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the operation.
    pub async fn delete(
        &self,
        id: &str,
        db: &Database,
        role_service: &RoleService,
    ) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let filter = doc! {
            "_id": id,
        };

        match db
            .collection::<Permission>(&self.collection)
            .delete_one(filter, None)
            .await
        {
            Ok(_) => {
                match role_service.delete_permission_from_all_roles(id, db).await {
                    Ok(_) => (),
                    Err(e) => return Err(Error::Role(e)),
                };
            }
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(())
    }

    /// # Summary
    ///
    /// Search for Permissions.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permissions = permission_repository.search(String::from("permission_name"), &db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Permission>, Error>` - The result of the operation.
    pub async fn search(&self, text: &str, db: &Database) -> Result<Vec<Permission>, Error> {
        if text.is_empty() {
            return Err(Error::EmptyTextSearch);
        }

        let filter = doc! {
            "$text": {
                "$search": text,
            },
        };

        let cursor = match db
            .collection::<Permission>(&self.collection)
            .find(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
