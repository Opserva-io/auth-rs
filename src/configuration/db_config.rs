use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DbConfig {
    pub connection_string: String,
    pub database_name: String,
    pub permission_collection: String,
    pub role_collection: String,
    pub user_collection: String,
    pub create_indexes: bool,
}

impl DbConfig {
    /// # Summary
    ///
    /// Creates a new DbConfig instance.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - A String that holds the connection string.
    /// * `database_name` - A String that holds the database name.
    /// * `permission_collection` - A String that holds the permission collection name.
    /// * `role_collection` - A String that holds the role collection name.
    /// * `user_collection` - A String that holds the user collection name.
    /// * `create_indexes` - A bool that indicates whether to create indexes or not.
    ///
    /// # Returns
    ///
    /// A DbConfig instance.
    pub fn new(
        connection_string: String,
        database_name: String,
        permission_collection: String,
        role_collection: String,
        user_collection: String,
        create_indexes: bool,
    ) -> DbConfig {
        DbConfig {
            connection_string,
            database_name,
            permission_collection,
            role_collection,
            user_collection,
            create_indexes,
        }
    }
}