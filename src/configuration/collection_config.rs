#[derive(Clone)]
pub struct CollectionConfig {
    pub permission_collection: String,
    pub role_collection: String,
    pub user_collection: String,
}

impl CollectionConfig {
    /// # Summary
    ///
    /// Creates a new CollectionConfig instance.
    ///
    /// # Arguments
    ///
    /// * `permission_collection` - A String that holds the permission collection name.
    /// * `role_collection` - A String that holds the role collection name.
    /// * `user_collection` - A String that holds the user collection name.
    ///
    /// # Returns
    ///
    /// A CollectionConfig instance.
    pub fn new(
        permission_collection: String,
        role_collection: String,
        user_collection: String,
    ) -> CollectionConfig {
        CollectionConfig {
            permission_collection,
            role_collection,
            user_collection,
        }
    }
}
