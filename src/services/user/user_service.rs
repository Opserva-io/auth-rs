use crate::repository::audit::audit_model::Action::{Create, Delete, Update};
use crate::repository::audit::audit_model::{Audit, ResourceIdType, ResourceType};
use crate::repository::audit::audit_repository::Error as AuditError;
use crate::repository::user::user_model::User;
use crate::repository::user::user_repository::{Error, UserRepository};
use crate::services::audit::audit_service::AuditService;
use log::{error, info};
use mongodb::bson::oid::ObjectId;
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
    /// let audit_service = AuditService::new(AuditRepository::new(String::from("audits")));
    /// let user = User::new("username", "password");
    ///
    /// let user = user_service.create(user, ObjectId::parse_str("user_id").unwrap(), &db, &audit_service);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The created User entity.
    /// * `Error` - The Error that occurred.
    pub async fn create(
        &self,
        user: User,
        user_id: Option<ObjectId>,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<User, Error> {
        info!("Creating User: {}", user);

        if user_id.is_some() {
            let new_audit = Audit::new(
                user_id.unwrap(),
                Create,
                user.id,
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
        }

        self.user_repository.create(user, db).await
    }

    /// # Summary
    ///
    /// Find all User entities.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of Users to return.
    /// * `page` - The page of Users to return.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let users = user_service.find_all(Some(10), Some(1), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<User>` - The found User entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(
        &self,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<User>, Error> {
        info!("Finding all users");
        self.user_repository.find_all(limit, page, db).await
    }

    /// # Summary
    ///
    /// Find a User entity by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the User entity.
    /// * `db` - The Database to be used.
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
    pub async fn find_by_id(&self, id: &str, db: &Database) -> Result<Option<User>, Error> {
        info!("Finding User by ID: {}", id);
        self.user_repository.find_by_id(id, db).await
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
    /// let user = user_service.find_by_username("username", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation.
    pub async fn find_by_username(
        &self,
        username: &str,
        db: &Database,
    ) -> Result<Option<User>, Error> {
        info!("Finding User by username: {}", username);
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
    ///
    /// let user = user_service.update(User::new(), ObjectId::parse_str("id").unwrap(), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The updated User entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(
        &self,
        user: User,
        user_id: Option<ObjectId>,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<User, Error> {
        info!("Updating User: {}", user);

        if user_id.is_some() {
            let new_audit = Audit::new(
                user_id.unwrap(),
                Update,
                user.id,
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
    ///
    /// let user = user_service.update_password("id", "password", ObjectId::parse_str("user_id").unwrap(), &db);
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
        user_id: Option<ObjectId>,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<(), Error> {
        info!("Updating User password: {}", id);

        if user_id.is_some() {
            let oid = match ObjectId::parse_str(id) {
                Ok(oid) => oid,
                Err(e) => {
                    return Err(Error::Audit(AuditError::ObjectId(e.to_string())));
                }
            };

            let new_audit = Audit::new(
                user_id.unwrap(),
                Update,
                oid,
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
    ///
    /// user_service.delete("id", ObjectId::parse_str("user_id").unwrap(), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The delete operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete(
        &self,
        id: &str,
        user_id: Option<ObjectId>,
        db: &Database,
        audit_service: &AuditService,
    ) -> Result<(), Error> {
        info!("Deleting User: {}", id);

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
    /// * `limit` - The maximum number of Users to return.
    /// * `page` - The page of Users to return.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    /// let users = user_service.search("text", Some(10), Some(1), &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<User>` - The Users that match the search criteria.
    /// * `Error` - The Error that occurred.
    pub async fn search(
        &self,
        text: &str,
        limit: Option<i64>,
        page: Option<i64>,
        db: &Database,
    ) -> Result<Vec<User>, Error> {
        info!("Searching Users: {}", text);
        self.user_repository.search(text, limit, page, db).await
    }
}
