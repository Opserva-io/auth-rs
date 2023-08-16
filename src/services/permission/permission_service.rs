use crate::repository::audit::audit_model::Action::{Create, Delete, Update};
use crate::repository::audit::audit_model::ResourceType::Permission as PermissionResourceType;
use crate::repository::audit::audit_model::{Audit, ResourceIdType};
use crate::repository::audit::audit_repository::Error as AuditError;
use crate::repository::permission::permission_model::Permission;
use crate::repository::permission::permission_repository::{Error, PermissionRepository};
use crate::services::audit::audit_service::AuditService;
use crate::services::role::role_service::RoleService;
use log::{error, info};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;

#[derive(Clone)]
pub struct PermissionService {
    pub permission_repository: PermissionRepository,
}

impl PermissionService {
    /// # Summary
    ///
    /// Create a new PermissionService.
    ///
    /// # Arguments
    ///
    /// * `permission_repository` - The PermissionRepository to be used by the PermissionService.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// ```
    ///
    /// # Returns
    ///
    /// * `PermissionService` - The new PermissionService.
    pub fn new(permission_repository: PermissionRepository) -> PermissionService {
        PermissionService {
            permission_repository,
        }
    }

    /// # Summary
    ///
    /// Create a new Permission entity.
    ///
    /// # Arguments
    ///
    /// * `new_permission` - The Permission entity to create.
    /// * `user_id` - The ID of the User creating the Permission entity.
    /// * `db` - The Database to create the Permission entity in.
    /// * `audit` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(audit_repository);
    /// let user_id = ObjectId::parse_str("user_id").unwrap();
    /// let permission = Permission::new(String::from("name"), String::from("description"));
    ///
    /// let new_permission = permission_service.create(permission, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<Permission>` - The Permission entity.
    /// * `Error` - The Error that occurred.
    pub async fn create(
        &self,
        new_permission: Permission,
        user_id: Option<ObjectId>,
        db: &Database,
        audit: &AuditService,
    ) -> Result<Permission, Error> {
        info!("Creating Permission: {}", new_permission);

        if user_id.is_some() {
            let new_audit = Audit::new(
                user_id.unwrap(),
                Create,
                new_permission.id,
                ResourceIdType::PermissionId,
                PermissionResourceType,
            );
            match audit.create(new_audit, db).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to create Audit: {}", e);
                    return Err(Error::Audit(e));
                }
            }
        }

        self.permission_repository.create(new_permission, db).await
    }

    /// # Summary
    ///
    /// Find all Permission entities.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit of Permission entities to find.
    /// * `page` - The page of Permission entities to find.
    /// * `db` - The Database to find the Permission entities in.
    /// * `audit` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let permissions = permission_service.find_all(limit, page, &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Permission>` - The Permission entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(
        &self,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Permission>, Error> {
        info!("Finding all permissions");
        self.permission_repository.find_all(limit, page, db).await
    }

    /// # Summary
    ///
    /// Find all Permission entities by id.
    ///
    /// # Arguments
    ///
    /// * `id_vec` - The Vector of IDs of the Permission entities.
    /// * `user_id` - The ID of the User finding the Permission entities.
    /// * `db` - The Database to find the Permission entities in.
    /// * `audit` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let id_vec = vec![String::from("id")];
    /// let permissions = permission_service.find_by_id_vec(id_vec, &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Permission>` - The Permission entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id_vec(
        &self,
        id_vec: Vec<String>,
        db: &Database,
    ) -> Result<Vec<Permission>, Error> {
        info!("Finding permissions by id_vec: {:?}", id_vec);
        self.permission_repository.find_by_id_vec(id_vec, db).await
    }

    /// # Summary
    ///
    /// Find a Permission entity by id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Permission entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let permission = permission_service.find_by_id(String::from("id"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<Permission>` - The Permission entity.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<Permission>, Error> {
        info!("Finding Permission by ID: {}", id);
        self.permission_repository.find_by_id(id, db).await
    }

    /// # Summary
    ///
    /// Find a Permission by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Permission to find.
    /// * `user_id` - The ID of the User finding the Permission.
    /// * `db` - The database to use.
    /// * `audit` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let name = String::from("name");
    /// let permission = permission_service.find_by_name(name, &db);
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
        info!("Finding Permission by name: {}", name);
        self.permission_repository.find_by_name(name, db).await
    }

    /// # Summary
    ///
    /// Update a Permission entity.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to create.
    /// * `user_id` - The ID of the User updating the Permission.
    /// * `db` - The Database to be used.
    /// * `audit` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(audit_repository);
    /// let user_id = ObjectId::parse_str("user_id").unwrap();
    /// let permission = Permission::new(String::from("name"), String::from("description"));
    /// let updated_permission = permission_service.update(permission, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Permission` - The Permission entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(
        &self,
        permission: Permission,
        user_id: Option<ObjectId>,
        db: &Database,
        audit: &AuditService,
    ) -> Result<Permission, Error> {
        info!("Updating Permission: {}", permission);

        if user_id.is_some() {
            let new_audit = Audit::new(
                user_id.unwrap(),
                Update,
                permission.id,
                ResourceIdType::PermissionId,
                PermissionResourceType,
            );
            match audit.create(new_audit, db).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to create Audit: {}", e);
                    return Err(Error::Audit(e));
                }
            }
        }

        self.permission_repository.update(permission, db).await
    }

    /// # Summary
    ///
    /// Delete a Permission entity.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Permission entity to delete.
    /// * `user_id` - The ID of the User deleting the Permission.
    /// * `db` - The Database to be used.
    /// * `role_service` - The RoleService to be used.
    /// * `audit` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(audit_repository);
    /// let role_service = RoleService::new(role_repository);
    /// let user_id = ObjectId::parse_str("user_id").unwrap();
    /// let id = String::from("id");
    ///
    /// permission_service.delete(id, user_id, &db, &role_service, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete(
        &self,
        id: &str,
        user_id: Option<ObjectId>,
        db: &Database,
        role_service: &RoleService,
        audit: &AuditService,
    ) -> Result<(), Error> {
        info!("Deleting Permission by ID: {}", id);

        if user_id.is_some() {
            let oid = match ObjectId::parse_str(id) {
                Ok(oid) => oid,
                Err(e) => {
                    return Err(Error::Audit(AuditError::ObjectId(e.to_string())));
                }
            };

            let new_audit = Audit::new(
                user_id.unwrap(),
                Delete,
                oid,
                ResourceIdType::PermissionId,
                PermissionResourceType,
            );
            match audit.create(new_audit, db).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to create Audit: {}", e);
                    return Err(Error::Audit(e));
                }
            }
        }

        self.permission_repository
            .delete(id, db, role_service)
            .await
    }

    /// # Summary
    ///
    /// Search for Permission entities by text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `limit` - The limit of Permission entities to find.
    /// * `page` - The page of Permission entities to find.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    /// let text = String::from("text");
    /// let permissions = permission_service.search(text, limit, page, &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Permission>` - The Permission entities.
    /// * `Error` - The Error that occurred.
    pub async fn search(
        &self,
        text: &str,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Permission>, Error> {
        info!("Searching for Permission by text: {}", text);
        self.permission_repository
            .search(text, limit, page, db)
            .await
    }
}
