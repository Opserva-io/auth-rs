use crate::repository::user::user::User;
use crate::repository::user::user_repository::{Error, UserRepository};
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
        self.user_repository.find_by_id(id, db).await
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
        self.user_repository.delete(id, db).await
    }
}
