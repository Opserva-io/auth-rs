#[derive(Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
    pub max_limit: i64,
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
    /// * `max_limit` - The maximum amount of entity records that can be retrieved in one call.
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
    pub fn new(address: String, port: u16, max_limit: i64) -> ServerConfig {
        ServerConfig {
            address,
            port,
            max_limit,
        }
    }
}
