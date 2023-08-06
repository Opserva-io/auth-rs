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
    /// * `db` - The Database to find the Audits in.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Audit>, Error>` - The result of the operation.
    pub async fn find_all(&self, db: &Database) -> Result<Vec<Audit>, Error> {
        info!("Finding all audits");
        self.audit_repository.find_all(db).await
    }
}
