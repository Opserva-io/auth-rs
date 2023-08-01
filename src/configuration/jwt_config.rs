#[derive(Clone)]
pub struct JwtConfig {
    pub jwt_secret: String,
    pub jwt_expiration: usize,
}

impl JwtConfig {
    /// # Summary
    ///
    /// Create a new JwtConfig.
    ///
    /// # Arguments
    ///
    /// * `jwt_secret` - The secret to use for signing and verifying JWTs.
    /// * `jwt_expiration` - The expiration time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let jwt_config = JwtConfig::new(String::from("secret"), 3600);
    /// ```
    ///
    /// # Returns
    ///
    /// * `JwtConfig` - The new JwtConfig.
    pub fn new(jwt_secret: String, jwt_expiration: usize) -> JwtConfig {
        JwtConfig {
            jwt_secret,
            jwt_expiration,
        }
    }
}
