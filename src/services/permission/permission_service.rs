use crate::repository::permission::permission::Permission;
use crate::repository::permission::permission_repository::{Error, PermissionRepository};
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
    /// Find a Permission entity by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Permission entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let permission = permission_service.find_by_name(String::from("name"), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<Permission>` - The Permission entity.
    /// * `Error` - The Error that occurred.
    pub async fn create(&self, permission: Permission, db: &Database) -> Result<Permission, Error> {
        self.permission_repository.create(permission, db).await
    }

    /// # Summary
    ///
    /// Find all Permission entities.
    ///
    /// # Arguments
    ///
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let permissions = permission_service.find_all(&db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Permission>` - The Permission entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(&self, db: &Database) -> Result<Vec<Permission>, Error> {
        self.permission_repository.find_all(db).await
    }

    /// # Summary
    ///
    /// Find all Permission entities by id.
    ///
    /// # Arguments
    ///
    /// * `id_vec` - The id of the Permission entities.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let permissions = permission_service.find_by_id_vec(vec![String::from("id")], &db);
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
        self.permission_repository.find_by_id(id, db).await
    }

    /// # Summary
    ///
    /// Create a Permission entity.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to create.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let permission = permission_service.create(Permission::new(String::from("name")), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Permission` - The Permission entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(&self, permission: Permission, db: &Database) -> Result<Permission, Error> {
        self.permission_repository.update(permission, db).await
    }

    /// # Summary
    ///
    /// Update a Permission entity.
    ///
    /// # Arguments
    ///
    /// * `permission` - The Permission entity to update.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let permission_repository = PermissionRepository::new(String::from("permissions"));
    /// let permission_service = PermissionService::new(permission_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let permission = permission_service.update(Permission::new(String::from("name")), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete(&self, id: &str, db: &Database) -> Result<(), Error> {
        self.permission_repository.delete(id, db).await
    }
}
