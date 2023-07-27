use crate::repository::permission::permission::Permission;
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
    NameAlreadyTaken,
    PermissionNotFound(String),
    MongoDbError(MongoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::EmptyCollection => write!(f, "Empty collection"),
            Error::EmptyId => write!(f, "Empty Permission ID"),
            Error::EmptyName => write!(f, "Empty Permission name"),
            Error::NameAlreadyTaken => write!(f, "Permission name already taken"),
            Error::PermissionNotFound(id) => write!(f, "Permission not found: {}", id),
            Error::MongoDbError(e) => write!(f, "MongoDB error: {}", e),
        }
    }
}

impl PermissionRepository {
    pub fn new(collection: String) -> Result<PermissionRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(PermissionRepository { collection })
    }

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
            Err(e) => return Err(Error::MongoDbError(e)),
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

    pub async fn find_all(&self, db: &Database) -> Result<Vec<Permission>, Error> {
        let cursor = match db
            .collection::<Permission>(&self.collection)
            .find(None, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

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
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

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
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(permission)
    }

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
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(permission)
    }

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
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        if permission.is_none() {
            return Err(Error::PermissionNotFound(permission_id));
        }

        Ok(permission.unwrap())
    }

    pub async fn delete(&self, id: &str, db: &Database) -> Result<(), Error> {
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
            Ok(_) => {}
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(())
    }
}
