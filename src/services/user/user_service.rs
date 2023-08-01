use crate::repository::user::user::User;
use crate::repository::user::user_repository::{Error, UserRepository};
use log::info;
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
    /// * `username` - The username of the User entity.
    /// * `db` - The Database to be used.
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
    pub async fn create(&self, user: User, db: &Database) -> Result<User, Error> {
        info!("Creating User: {}", user);
        self.user_repository.create(user, db).await
    }

    /// # Summary
    ///
    /// Find all User entities.
    ///
    /// # Arguments
    ///
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let users = user_service.find_all(&db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<User>` - The found User entities.
    /// * `Error` - The Error that occurred.
    pub async fn find_all(&self, db: &Database) -> Result<Vec<User>, Error> {
        info!("Finding all users");
        self.user_repository.find_all(db).await
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
    /// * `db` - The Database.
    ///
    /// # Example
    ///
    /// ```
    /// let db = Database::new();
    /// let user_repository = UserRepository::new(String::from("users"), email_regex);
    /// let user = user_repository.find_by_username(&String::from("username"), &db);
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
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let user = user_service.update(user, &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `User` - The updated User entity.
    /// * `Error` - The Error that occurred.
    pub async fn update(&self, user: User, db: &Database) -> Result<User, Error> {
        info!("Updating User: {}", user);
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
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// user_service.update_password("id", "password", &db);
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
        db: &Database,
    ) -> Result<(), Error> {
        info!("Updating User password: {}", id);
        self.user_repository.update_password(id, password, db).await
    }

    /// # Summary
    ///
    /// Delete a User entity by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the User entity to be deleted.
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let user = user_service.delete("id", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `()` - The delete operation was successful.
    /// * `Error` - The Error that occurred.
    pub async fn delete(&self, id: &str, db: &Database) -> Result<(), Error> {
        info!("Deleting User: {}", id);
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
    /// * `db` - The Database to be used.
    ///
    /// # Example
    ///
    /// ```
    /// let user_repository = UserRepository::new(String::from("users"));
    /// let user_service = UserService::new(user_repository);
    /// let db = mongodb::Database::new();
    ///
    /// let users = user_service.search("text", &db);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<User>` - The Users that match the search criteria.
    /// * `Error` - The Error that occurred.
    pub async fn search(&self, text: &str, db: &Database) -> Result<Vec<User>, Error> {
        info!("Searching Users: {}", text);
        self.user_repository.search(text, db).await
    }
}
