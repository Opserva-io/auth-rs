use crate::repository::audit::audit_model::Audit;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error as MongodbError;
use mongodb::Database;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct AuditRepository {
    pub collection: String,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidId(String),
    EmptyCollection,
    EmptyTextSearch,
    MongoDb(MongodbError),
}

impl Display for Error {
    /// # Summary
    ///
    /// Display the Error.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a Formatter.
    ///
    /// # Returns
    ///
    /// A std::fmt::Result.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidId(id) => write!(f, "Invalid Audit ID: {}", id),
            Error::EmptyCollection => write!(f, "Empty Audit collection"),
            Error::EmptyTextSearch => write!(f, "Empty text search"),
            Error::MongoDb(e) => write!(f, "MongoDb Error: {}", e),
        }
    }
}

impl AuditRepository {
    /// # Summary
    ///
    /// Create a new AuditRepository.
    ///
    /// # Arguments
    ///
    /// * `collection` - The collection name.
    ///
    /// # Returns
    ///
    /// * `Result<AuditRepository, Error>` - The result of the operation.
    pub fn new(collection: String) -> Result<AuditRepository, Error> {
        if collection.is_empty() {
            return Err(Error::EmptyCollection);
        }

        Ok(AuditRepository { collection })
    }

    /// # Summary
    ///
    /// Create a new Audit.
    ///
    /// # Arguments
    ///
    /// * `audit` - The Audit to create.
    /// * `db` - The Database to create the Audit in.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the operation.
    pub async fn create(&self, audit: Audit, db: &Database) -> Result<(), Error> {
        match db
            .collection::<Audit>(&self.collection)
            .insert_one(audit, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Find an Audit by id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Audit to find.
    /// * `db` - The Database to find the Audit in.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Audit>, Error>` - The result of the operation.
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<Audit>, Error> {
        let target_object_id = match ObjectId::parse_str(id) {
            Ok(res) => res,
            Err(e) => {
                return Err(Error::InvalidId(e.to_string()));
            }
        };

        match db
            .collection::<Audit>(&self.collection)
            .find_one(doc! {"_id": target_object_id}, None)
            .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Find all Audits.
    ///
    /// # Arguments
    ///
    /// * `db` - The Database to find the Audits in.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Audit>, Error>` - The result of the operation.
    pub async fn find_all(&self, db: &Database) -> Result<Vec<Audit>, Error> {
        match db
            .collection::<Audit>(&self.collection)
            .find(doc! {}, None)
            .await
        {
            Ok(r) => Ok(r.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(e) => Err(Error::MongoDb(e)),
        }
    }

    /// # Summary
    ///
    /// Search for Audit entities.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let audit_repository = AuditRepository::new("audit".to_string()).unwrap();
    /// let db = mongodb::Client::with_uri_str("mongodb://localhost:27017")
    ///    .await
    ///    .unwrap()
    ///    .database("test");
    ///
    /// let result = audit_repository.search("", &db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Audit>, Error>` - The result of the operation.
    pub async fn search(&self, text: &str, db: &Database) -> Result<Vec<Audit>, Error> {
        if text.is_empty() {
            return Err(Error::EmptyTextSearch);
        }

        let filter = doc! {
            "$text": {
                "$search": text,
            },
        };

        let cursor = match db
            .collection::<Audit>(&self.collection)
            .find(filter, None)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(Error::MongoDb(e)),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
