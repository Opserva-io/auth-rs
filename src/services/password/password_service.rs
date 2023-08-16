use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone)]
pub struct PasswordService {}

impl PasswordService {
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
    pub fn hash_password(password: String) -> Result<String, String> {
        let password = &password.as_bytes();
        let argon2 = Argon2::default();

        let salt = SaltString::generate(&mut OsRng);

        match argon2.hash_password(password, &salt) {
            Ok(e) => Ok(e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// # Summary
    ///
    /// Verify a password.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to verify.
    /// * `hash` - The hash to verify against.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the password is valid.
    pub fn verify_password(password: &str, hash: &PasswordHash) -> bool {
        Argon2::default()
            .verify_password(&password.as_bytes(), &hash)
            .is_ok()
    }
}
