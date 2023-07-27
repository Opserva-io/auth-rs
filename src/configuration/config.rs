use crate::repository::permission::permission_repository::PermissionRepository;
use crate::repository::role::role_repository::RoleRepository;
use crate::repository::user::user_repository::UserRepository;
use crate::services::permission::permission_service::PermissionService;
use crate::services::role::role_service::RoleService;
use crate::services::user::user_service::UserService;
use crate::services::Services;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};

#[derive(Clone)]
pub struct Config {
    pub database: Database,
    pub services: Services,
}

impl Config {
    /// # Summary
    ///
    /// Creates a new Config instance.
    ///
    /// # Arguments
    ///
    /// * `db_connection_string` - A string slice that holds the database connection string.
    /// * `database` - A string slice that holds the database name.
    /// * `permission_collection` - A String that holds the permission collection name.
    /// * `role_collection` - A String that holds the role collection name.
    /// * `user_collection` - A String that holds the user collection name.
    ///
    /// # Returns
    ///
    /// A Config instance.
    pub async fn new(
        db_connection_string: &str,
        database: &str,
        permission_collection: String,
        role_collection: String,
        user_collection: String,
    ) -> Config {
        let mut client_options = match ClientOptions::parse(db_connection_string).await {
            Ok(d) => d,
            Err(e) => {
                panic!("Failed to parse options: {:?}", e);
            }
        };

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).expect("Failed to initialize client");
        let db = client.database(&database);

        let permission_repository = match PermissionRepository::new(permission_collection) {
            Ok(d) => d,
            Err(e) => panic!("Failed to initialize Permission repository: {:?}", e),
        };
        let role_repository = match RoleRepository::new(role_collection) {
            Ok(d) => d,
            Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
        };

        let user_repository = match UserRepository::new(user_collection) {
            Ok(d) => d,
            Err(e) => panic!("Failed to initialize User repository: {:?}", e),
        };

        let permission_service = PermissionService::new(permission_repository);
        let role_service = RoleService::new(role_repository);
        let user_service = UserService::new(user_repository);

        let services = Services::new(permission_service, role_service, user_service);

        Config {
            database: db,
            services,
        }
    }
}
