use crate::repository::audit::audit_model::Action::{Create, Delete, Read, Search, Update};
use crate::repository::audit::audit_model::{Audit, ResourceIdType, ResourceType};
use crate::repository::user::user_model::User;
use crate::repository::user::user_repository::{Error, UserRepository};
use crate::services::audit::audit_service::AuditService;
use log::{error, info};
use mongodb::Database;

#[derive(Clone)]
pub struct UserService {
    pub user_repository: UserRepository,
}

impl UserService {
    /// # Summary
    ///
    /// Create a new UserService.
    ///
    /// # Arguments
    ///
    /// * `user_repository` - The UserRepository to be used by the UserService.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// ```
    ///
    /// # Returns
    ///
    /// * `UserService` - The new UserService.
    pub fn new(user_repository: UserRepository) -> UserService {
        UserService { user_repository }
    }

    /// # Summary
    ///
    /// Create a User entity.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to be created.
    /// * `user_id` - The ID of the User entity that is creating the new User.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let user = user_service.find_by_username("username", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The created User entity.
    /// * `Error` - The Error that occurred.
    pub async fn create(
        &self,
        user: User,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<User, Error> {
        info!("Creating User: {}", user);

        let new_audit = Audit::new(
            user_id,
            Create,
            &user.id,
            ResourceIdType::UserId,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.create(user, db).await
    }

    /// # Summary
    ///
    /// Find all User entities.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the User entity that is finding all Users.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let user = user_service.find_by_username("username", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<User>` - The found User entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(
        &self,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Vec<User>, Error> {
        info!("Finding all users");

        let new_audit = Audit::new(user_id, Read, "", ResourceIdType::None, ResourceType::User);
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.find_all(db).await
    }

    /// # Summary
    ///
    /// Find a User entity by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the User entity.
    /// * `user_id` - The ID of the User entity that is finding the User.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let user = user_service.find_by_id("id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<User>` - The created User entity.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_id(
        &self,
        id: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Option<User>, Error> {
        info!("Finding User by ID: {}", id);

        let new_audit = Audit::new(
            user_id,
            Read,
            id,
            ResourceIdType::UserId,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.find_by_id(id, db).await
    }

    /// # Summary
    ///
    /// Find a User entity by email.
    ///
    /// # Arguments
    ///
    /// * `email` - The email of the User entity.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let user = user_service.find_by_email("email", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<User>` - The found User entity.
    /// * `Error` - The Error that occurred.
    pub async fn find_by_email(&self, email: &str, db: &Database) -> Result<Option<User>, Error> {
        info!("Finding User by email: {}", email);
        self.user_repository.find_by_email(email, db).await
    }

    /// # Summary
    ///
    /// Find a User entity by its username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the User entity.
    /// * `user_id` - The ID of the User entity that is finding the User.
    /// * `db` - The Database.
    /// * `audit_service` - The AuditService.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let user = user_service.find_by_username("username", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation.
    pub async fn find_by_username(
        &self,
        username: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Option<User>, Error> {
        info!("Finding User by username: {}", username);

        let new_audit = Audit::new(
            user_id,
            Read,
            username,
            ResourceIdType::UserName,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.find_by_username(username, db).await
    }

    /// # Summary
    ///
    /// Update a user entity.
    ///
    /// # Arguments
    ///
    /// * `user` - The User entity to be updated including its updated values.
    /// * `user_id` - The ID of the User entity that is updating the User.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let user = user_service.update(User::new(), "id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The updated User entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(
        &self,
        user: User,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<User, Error> {
        info!("Updating User: {}", user);

        let new_audit = Audit::new(
            user_id,
            Update,
            &user.id,
            ResourceIdType::UserId,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.update(user, db).await
    }

    /// # Summary
    ///
    /// Update a User entity's password.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the User entity to be updated.
    /// * `password` - The new password of the User entity.
    /// * `user_id` - The ID of the User entity that is updating the User.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let user = user_service.update_password("id", "password", "user_id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The update operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn update_password(
        &self,
        id: &str,
        password: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<(), Error> {
        info!("Updating User password: {}", id);

        let new_audit = Audit::new(
            user_id,
            Update,
            id,
            ResourceIdType::UserId,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.update_password(id, password, db).await
    }

    /// # Summary
    ///
    /// Delete a User entity by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the User entity to be deleted.
    /// * `user_id` - The ID of the User entity that is deleting the User.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// user_service.delete("id", "user_id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The delete operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete(
        &self,
        id: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<(), Error> {
        info!("Deleting User: {}", id);

        let new_audit = Audit::new(
            user_id,
            Delete,
            id,
            ResourceIdType::UserId,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.delete(id, db).await
    }

    /// # Summary
    ///
    /// Delete a Role from all Users.
    ///
    /// # Arguments
    ///
    /// * `role_id` - The ID of the Role entity to be deleted from all Users.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// user_service.delete_role_from_all_users("role_id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The delete operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete_role_from_all_users(
        &self,
        role_id: &str,
        db: &Database,
    ) -> Result<(), Error> {
        info!("Deleting Role from all Users: {}", role_id);
        self.user_repository
            .delete_role_from_all_users(role_id, db)
            .await
    }

    /// # Summary
    ///
    /// Search for Users.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search for.
    /// * `user_id` - The ID of the User entity that is searching for Users.
    /// * `db` - The Database to be used.
    /// * `audit_service` - The AuditService to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let users = user_service.search("text", "user_id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<User>` - The Users that match the search criteria.
    /// * `Error` - The Error that occurred.
    pub async fn search(
        &self,
        text: &str,
        user_id: &str,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<Vec<User>, Error> {
        info!("Searching Users: {}", text);

        let new_audit = Audit::new(
            user_id,
            Search,
            "",
            ResourceIdType::UserSearch,
            ResourceType::User,
        );
        match audit_service.create(new_audit, db).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to create Audit: {}", e);
                return Err(Error::Audit(e));
            }
        }

        self.user_repository.search(text, db).await
    }
}
