#[derive(Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

impl ServerConfig {
    /// # Summary
    ///
    /// Create a new ServerConfig.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the ServerConfig.
    /// * `port` - The port of the ServerConfig.
    ///
    /// # Example
    ///
    /// ```
    /// let server_config = ServerConfig::new(String::from("address"), 8080);
    /// ```
    ///
    /// # Returns
    ///
    /// * `ServerConfig` - The new ServerConfig.
    pub fn new(address: String, port: u16) -> ServerConfig {
        ServerConfig { address, port }
    }
}
