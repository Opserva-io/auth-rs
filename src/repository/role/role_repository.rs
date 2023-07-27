use crate::repository::role::role::Role;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::Bson;
use mongodb::error::Error as MongoError;
use mongodb::Database;
use std::fmt;
use std::time::SystemTime;

#[derive(Clone)]
pub struct RoleRepository {
    pub collection: String,
}

#[derive(Clone, Debug)]
pub enum Error {
    EmptyCollection,
    EmptyId,
    EmptyName,
    NameAlreadyTaken,
    RoleNotFound(String),

    MongoDbError(MongoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::EmptyCollection => write!(f, "Empty collection"),
            Error::EmptyId => write!(f, "Empty Role ID"),
            Error::EmptyName => write!(f, "Empty Role name"),
            Error::NameAlreadyTaken => write!(f, "Role name already taken"),
            Error::RoleNotFound(id) => write!(f, "Role not found: {}", id),
            Error::MongoDbError(e) => write!(f, "MongoDB error: {}", e),
        }
    }
}

impl RoleRepository {
    pub fn new(collection: String) -> Result<RoleRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(RoleRepository { collection })
    }

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

        let role_id = role.id.clone();
        match db
            .collection::<Role>(&self.collection)
            .insert_one(role, None)
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(Error::MongoDbError(e)),
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

    pub async fn find_all(&self, db: &Database) -> Result<Vec<Role>, Error> {
        let cursor = match db
            .collection::<Role>(&self.collection)
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
        ids: Vec<String>,
        db: &Database,
    ) -> Result<Vec<Role>, Error> {
        let filter = doc! {
            "_id": {
                "$in": ids,
            },
        };

        let cursor = match db
            .collection::<Role>(&self.collection)
            .find(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<Role>, Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let filter = doc! {
            "_id": id,
        };

        let role = match db
            .collection::<Role>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(role)
    }

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

        let role = match db
            .collection::<Role>(&self.collection)
            .find_one(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        Ok(role)
    }

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

        let role_id = role.id.clone();
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
            Err(e) => return Err(Error::MongoDbError(e)),
        };

        if role.is_none() {
            return Err(Error::RoleNotFound(role_id));
        }

        Ok(role.unwrap())
    }

    pub async fn delete(&self, id: &str, db: &Database) -> Result<(), Error> {
        if id.is_empty() {
            return Err(Error::EmptyId);
        }

        let filter = doc! {
            "_id": id,
        };

        match db
            .collection::<Role>(&self.collection)
            .delete_one(filter, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDbError(e)),
        }
    }
}
