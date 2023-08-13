use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

#[derive(Clone)]
pub struct PasswordService {
    pub salt: SaltString,
}

impl PasswordService {
    /// # Summary
    ///
    /// Create a new instance of PasswordService.
    ///
    /// # Arguments
    ///
    /// * `salt` - The salt to use for hashing.
    ///
    /// # Returns
    ///
    /// A new instance of PasswordService.
    pub fn new(salt: SaltString) -> PasswordService {
        PasswordService { salt }
    }

    /// # Summary
    ///
    /// Hash a password.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to hash.
    ///
    /// # Returns
    ///
    /// A Result containing the hashed password or an error.
    pub fn hash_password(&self, password: String) -> Result<String, String> {
        let password = &password.as_bytes();
        let argon2 = Argon2::default();
        match argon2.hash_password(password, &self.salt) {
            Ok(e) => Ok(e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}
