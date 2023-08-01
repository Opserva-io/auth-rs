#[derive(Clone)]
pub struct DefaultUserConfig {
    pub username: String,
    pub email: String,
    pub password: String,
    pub enabled: bool,
}

impl DefaultUserConfig {
    /// # Summary
    ///
    /// Creates a new DefaultUserConfig instance.
    ///
    /// # Arguments
    ///
    /// * `username` - A String that holds the default username.
    /// * `email` - A String that holds the default email.
    /// * `password` - A String that holds the default password.
    /// * `enabled` - A bool that holds the default enabled value.
    ///
    /// # Returns
    ///
    /// A DefaultUserConfig instance.
    pub fn new(
        username: String,
        email: String,
        password: String,
        enabled: bool,
    ) -> DefaultUserConfig {
        DefaultUserConfig {
            username,
            email,
            password,
            enabled,
        }
    }
}
