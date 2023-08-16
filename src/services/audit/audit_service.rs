use crate::repository::audit::audit_model::Audit;
use crate::repository::audit::audit_repository::{AuditRepository, Error};
use log::info;
use mongodb::Database;

#[derive(Clone)]
pub struct AuditService {
    pub audit_repository: AuditRepository,
    pub enabled: bool,
}

impl AuditService {
    /// # Summary
    ///
    /// Create a new AuditService.
    ///
    /// # Arguments
    ///
    /// * `audit_repository` - The AuditRepository.
    /// * `enabled` - Whether or not the AuditService is enabled.
    ///
    /// # Returns
    ///
    /// * `AuditService` - The AuditService.
    pub fn new(audit_repository: AuditRepository, enabled: bool) -> AuditService {
        AuditService {
            audit_repository,
            enabled,
        }
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
        if !self.enabled {
            return Ok(());
        }

        info!("Creating audit: {}", audit);
        self.audit_repository.create(audit, db).await
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
        info!("Finding audit by ID: {}", id);
        self.audit_repository.find_by_id(id, db).await
    }

    /// # Summary
    ///
    /// Find all Audits.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit of Audits to find.
    /// * `page` - The page of Audits to find.
    /// * `db` - The Database to find the Audits in.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Audit>, Error>` - The result of the operation.
    pub async fn find_all(
        &self,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Audit>, Error> {
        info!("Finding all audits");
        self.audit_repository.find_all(limit, page, &db).await
    }

    /// # Summary
    ///
    /// Search for Audit entities.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `limit` - The limit of Audits to find.
    /// * `page` - The page of Audits to find.
    /// * `db` - The database to use.
    ///
    /// # Example
    ///
    /// ```
    /// let audit_repository = AuditRepository::new("audit".to_string()).unwrap();
    /// let audit_service = AuditService::new(audit_repository, true);
    /// let db = mongodb::Client::with_uri_str("mongodb://localhost:27017")
    ///    .unwrap()
    ///    .database("test");
    /// let res = audit_service.search("test", 1, 10, &db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Audit>, Error>` - The result of the operation.
    pub async fn search(
        &self,
        text: &str,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Audit>, Error> {
        info!("Searching for audits: {}", text);
        self.audit_repository.search(text, limit, page, db).await
    }
}
