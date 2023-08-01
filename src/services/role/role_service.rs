use crate::repository::role::role::Role;
use crate::repository::role::role_repository::{Error, RoleRepository};
use crate::services::user::user_service::UserService;
use log::info;
use mongodb::Database;

#[derive(Clone)]
pub struct RoleService {
    pub role_repository: RoleRepository,
}

impl RoleService {
    /// # Summary
    ///
    /// Create a new RoleService.
    ///
    /// # Arguments
    ///
    /// * `role_repository` - The RoleRepository to be used by the RoleService.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// ```
    ///
    /// # Returns
    ///
    /// * `RoleService` - The new RoleService.
    pub fn new(role_repository: RoleRepository) -> RoleService {
        RoleService { role_repository }
    }

    /// # Summary
    ///
    /// Find a Role entity by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Role entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let role = role_service.find_by_name("name", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The created Role entity.
    /// * `Error` - The Error that occurred.
    pub async fn create(&self, role: Role, db: &Database) -> Result<Role, Error> {
        info!("Creating Role: {}", role);
        self.role_repository.create(role, db).await
    }

    /// # Summary
    ///
    /// Find all Role entities.
    ///
    /// # Arguments
    ///
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let roles = role_service.find_all(&db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Role>` - The Role entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(&self, db: &Database) -> Result<Vec<Role>, Error> {
        info!("Finding all roles");
        self.role_repository.find_all(db).await
    }

    /// # Summary
    ///
    /// Find a Role entity by its id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Role entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let role = role_service.find_by_id("id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<Role>` - The optional Role entity.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<Role>, Error> {
        info!("Finding Role by ID: {}", id);
        self.role_repository.find_by_id(id, db).await
    }

    /// # Summary
    ///
    /// Find a vector of Role entities by their ids.
    ///
    /// # Arguments
    ///
    /// * `id_vec` - The vector of ids of the Role entities.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let roles = role_service.find_by_id_vec(vec!["id1".to_string(), "id2".to_string()], &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Role>` - The vector of Role entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id_vec(
        &self,
        id_vec: Vec<String>,
        db: &Database,
    ) -> Result<Vec<Role>, Error> {
        info!("Finding roles by id vec: {:?}", id_vec);
        self.role_repository.find_by_id_vec(id_vec, db).await
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
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let role = role_service.find_by_name("name", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with an Option of a Role instance or an Error.
    pub async fn find_by_name(&self, name: &str, db: &Database) -> Result<Option<Role>, Error> {
        info!("Finding Role by name: {}", name);
        self.role_repository.find_by_name(name, db).await
    }

    /// # Summary
    ///
    /// Update a Role entity.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let role = role_service.update(role, &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The updated Role entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(&self, role: Role, db: &Database) -> Result<Role, Error> {
        info!("Updating Role: {}", role);
        self.role_repository.update(role, db).await
    }

    /// # Summary
    ///
    /// Delete a Role entity by its id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Role entity.
    /// * `db` - The Database to be used.
    /// * `user_service` - The UserService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let user_service = UserService::new(user_repository);
    /// let id = "id";
    /// let result = role_service.delete(id, &db, &user_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete(
        &self,
        id: &str,
        db: &Database,
        user_service: &UserService,
    ) -> Result<(), Error> {
        info!("Deleting Role by ID: {}", id);
        self.role_repository.delete(id, db, user_service).await
    }

    /// # Summary
    ///
    /// Delete a Permission entity from all Role entities.
    ///
    /// # Arguments
    ///
    /// * `permission_id` - The id of the Permission entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let res = role_service.delete_permission_from_all_roles("id", &db).await;
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete_permission_from_all_roles(
        &self,
        permission_id: &str,
        db: &Database,
    ) -> Result<(), Error> {
        info!(
            "Deleting permission {} from all Role entities",
            permission_id
        );
        self.role_repository
            .delete_permission_from_all_roles(permission_id, db)
            .await
    }
}
