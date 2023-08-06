use crate::repository::audit::audit_model::Action::{Create, Delete, Read, Search, Update};
use crate::repository::audit::audit_model::{Audit, ResourceIdType, ResourceType};
use crate::repository::role::role_model::Role;
use crate::repository::role::role_repository::{Error, RoleRepository};
use crate::services::audit::audit_service::AuditService;
use crate::services::user::user_service::UserService;
use log::{error, info};
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
    /// Create a new Role.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role to be created.
    /// * `user_id` - The id of the User creating the Role.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let role = Role::new(String::from("role_name"));
    /// let user_id = "user_id";
    ///
    /// let role = role_service.create(role, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The created Role entity.
    /// * `Error` - The Error that occurred.
    pub async fn create(
        &self,
        role: Role,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Role, Error> {
        info!("Creating Role: {}", role);

        let new_audit = Audit::new(
            user_id,
            Create,
            &role.id,
            ResourceIdType::RoleId,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.create(role, db).await
    }

    /// # Summary
    ///
    /// Find all Role entities.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The id of the User finding the Roles.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let user_id = "user_id";
    ///
    /// let roles = role_service.find_all(user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Role>` - The Role entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(
        &self,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Vec<Role>, Error> {
        info!("Finding all roles");

        let new_audit = Audit::new(user_id, Read, "", ResourceIdType::None, ResourceType::Role);
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.find_all(db).await
    }

    /// # Summary
    ///
    /// Find a Role entity by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Role entity.
    /// * `user_id` - The id of the User finding the Role.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let id = "role_id";
    /// let user_id = "user_id";
    ///
    /// let role = role_service.find_by_id(id, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<Role>` - The optional Role entity.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id(
        &self,
        id: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Option<Role>, Error> {
        info!("Finding Role by ID: {}", id);

        let new_audit = Audit::new(
            user_id,
            Read,
            id,
            ResourceIdType::RoleId,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.find_by_id(id, db).await
    }

    /// # Summary
    ///
    /// Find a vector of Role entities by their ids.
    ///
    /// # Arguments
    ///
    /// * `id_vec` - The vector of ids of the Role entities.
    /// * `user_id` - The id of the User finding the Role entities.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let id_vec = vec!["role_id"];
    /// let user_id = "user_id";
    ///
    /// let roles = role_service.find_by_id_vec(id_vec, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Role>` - The vector of Role entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id_vec(
        &self,
        id_vec: Vec<String>,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Vec<Role>, Error> {
        info!("Finding roles by id vec: {:?}", id_vec);

        let new_audit = Audit::new(
            user_id,
            Read,
            &format!("{:?}", id_vec),
            ResourceIdType::RoleIdVec,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.find_by_id_vec(id_vec, db).await
    }

    /// # Summary
    ///
    /// Find a role by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name.
    /// * `user_id` - The id of the User finding the Role.
    /// * `db` - A reference to a Database instance.
    /// * `audit_service` - A reference to an AuditService instance.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let name = "role_name";
    /// let user_id = "user_id";
    ///
    /// let role = role_service.find_by_name(name, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// A Result with an Option of a Role instance or an Error.
    pub async fn find_by_name(
        &self,
        name: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Option<Role>, Error> {
        info!("Finding Role by name: {}", name);

        let new_audit = Audit::new(
            user_id,
            Read,
            name,
            ResourceIdType::RoleName,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.find_by_name(name, db).await
    }

    /// # Summary
    ///
    /// Update a Role entity.
    ///
    /// # Arguments
    ///
    /// * `role` - The Role entity.
    /// * `user_id` - The id of the User updating the Role entity.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let role = Role::new(String::from("role_name"), vec!["permission_id"]);
    /// let user_id = "user_id";
    ///
    /// let updated_role = role_service.update(role, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Role` - The updated Role entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(
        &self,
        role: Role,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Role, Error> {
        info!("Updating Role: {}", role);

        let new_audit = Audit::new(
            user_id,
            Update,
            &role.id,
            ResourceIdType::RoleId,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.update(role, db).await
    }

    /// # Summary
    ///
    /// Delete a Role entity by its id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the Role entity.
    /// * `user_id` - The id of the User deleting the Role entity.
    /// * `db` - The Database to be used.
    /// * `user_service` - The UserService to be used.
    /// * `audit_service` - The AuditService to be used.
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
        user_id: &str,
        db: &Database,
        user_service: &UserService,
        audit_service: &AuditService,
    ) -> Result<(), Error> {
        info!("Deleting Role by ID: {}", id);

        let new_audit = Audit::new(
            user_id,
            Delete,
            &id,
            ResourceIdType::RoleId,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

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

    /// # Summary
    ///
    /// Search for Role entities by text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `user_id` - The id of the User searching for the Role entities.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let role_repository = RoleRepository::new(String::from("roles"));
    /// let role_service = RoleService::new(role_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let text = "text";
    /// let user_id = "user_id";
    /// let result = role_service.search(text, user_id, &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<Role>` - The vector of Role entities.
    /// * `Error` - The Error that occurred.
    pub async fn search(
        &self,
        text: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Vec<Role>, Error> {
        info!("Searching for Role by text: {}", text);

        let new_audit = Audit::new(
            user_id,
            Search,
            "",
            ResourceIdType::RoleSearch,
            ResourceType::Role,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.role_repository.search(text, db).await
    }
}
