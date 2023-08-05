use crate::configuration::db_config::DbConfig;
use crate::configuration::default_user_config::DefaultUserConfig;
use crate::configuration::jwt_config::JwtConfig;
use crate::repository::permission::permission::Permission;
use crate::repository::permission::permission_repository::PermissionRepository;
use crate::repository::role::role::Role;
use crate::repository::role::role_repository::RoleRepository;
use crate::repository::user::user::User;
use crate::repository::user::user_repository::UserRepository;
use crate::services::jwt::jwt_service::JwtService;
use crate::services::permission::permission_service::PermissionService;
use crate::services::role::role_service::RoleService;
use crate::services::user::user_service::UserService;
use crate::services::Services;
use log::info;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database, IndexModel};
use regex::Regex;

#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub database: Database,
    pub services: Services,
    pub salt: String,
    pub permission_collection: String,
    pub role_collection: String,
    pub user_collection: String,
}

impl Config {
    /// # Summary
    ///
    /// Creates a new Config instance.
    ///
    /// # Arguments
    ///
    /// * `address` - A String that holds the server address.
    /// * `port` - A u16 that holds the server port.
    /// * `db_config` - A DbConfig instance.
    /// * `default_user_config` - A DefaultUserConfig instance.
    /// * `generate_default_user` - A bool that indicates whether to generate a default user or not.
    /// * `salt` - A String that holds the salt.
    /// * `jwt_config` - A JwtConfig instance.
    ///
    /// # Returns
    ///
    /// A Config instance.
    pub async fn new(
        address: String,
        port: u16,
        db_config: DbConfig,
        default_user_config: DefaultUserConfig,
        generate_default_user: bool,
        salt: String,
        jwt_config: JwtConfig,
    ) -> Config {
        let mut client_options = match ClientOptions::parse(&db_config.connection_string).await {
            Ok(d) => d,
            Err(e) => {
                panic!("Failed to parse options: {:?}", e);
            }
        };

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).expect("Failed to initialize client");
        let db = client.database(&db_config.database_name);

        let permission_repository =
            match PermissionRepository::new(db_config.permission_collection.clone()) {
                Ok(d) => d,
                Err(e) => panic!("Failed to initialize Permission repository: {:?}", e),
            };
        let role_repository = match RoleRepository::new(db_config.role_collection.clone()) {
            Ok(d) => d,
            Err(e) => panic!("Failed to initialize Role repository: {:?}", e),
        };

        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        let user_repository =
            match UserRepository::new(db_config.user_collection.clone(), email_regex.clone()) {
                Ok(d) => d,
                Err(e) => panic!("Failed to initialize User repository: {:?}", e),
            };

        let permission_service = PermissionService::new(permission_repository);
        let role_service = RoleService::new(role_repository);
        let user_service = UserService::new(user_repository);
        let jwt_service = JwtService::new(jwt_config);

        let services = Services::new(permission_service, role_service, user_service, jwt_service);

        let cfg = Config {
            address,
            port,
            database: db,
            services,
            salt,
            permission_collection: db_config.permission_collection,
            role_collection: db_config.role_collection,
            user_collection: db_config.user_collection,
        };

        if generate_default_user {
            cfg.initialize_database(default_user_config, email_regex)
                .await;
        }

        if db_config.create_indexes {
            cfg.create_permission_indexes().await;
            cfg.create_role_indexes().await;
            cfg.create_user_indexes().await;
        }

        cfg
    }

    /// # Summary
    ///
    /// Find or create a permission.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the permission name.
    /// * `description` - An optional string slice that holds the permission description.
    ///
    /// # Returns
    ///
    /// A Permission instance.
    async fn find_or_create_permission(
        &self,
        name: &str,
        description: Option<String>,
    ) -> Permission {
        match self
            .services
            .permission_service
            .find_by_name(name, &self.database)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    let p = Permission::new(name.to_string(), description);
                    match self
                        .services
                        .permission_service
                        .create(p, &self.database)
                        .await
                    {
                        Ok(p) => return p,
                        Err(e) => panic!("Failed to create permission: {:?}", e),
                    }
                }
                d.unwrap()
            }
            Err(e) => panic!("Failed to find permission: {:?}", e),
        }
    }

    /// # Summary
    ///
    /// Find or create a role.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the role name.
    /// * `description` - An optional string slice that holds the role description.
    /// * `permissions` - An optional vector of string slices that holds the permissions.
    ///
    /// # Returns
    ///
    /// A Role instance.
    async fn find_or_create_role(
        &self,
        name: &str,
        description: Option<String>,
        permissions: Option<Vec<String>>,
    ) -> Role {
        match self
            .services
            .role_service
            .find_by_name(name, &self.database)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    let new_role = Role::new(name.to_string(), description, permissions.clone());
                    match self
                        .services
                        .role_service
                        .create(new_role, &self.database)
                        .await
                    {
                        Ok(d) => d,
                        Err(e) => panic!("Failed to create role: {:?}", e),
                    }
                } else {
                    d.unwrap()
                }
            }
            Err(e) => panic!("Failed to find role: {:?}", e),
        }
    }

    /// # Summary
    ///
    /// Find or create a user.
    ///
    /// # Arguments
    ///
    /// * `default_user_config` - A DefaultUserConfig instance.
    /// * `roles` - An optional vector of string slices that holds the roles.
    /// * `salt` - A string slice that holds the salt to hash the password.
    /// * `email_regex` - A Regex instance that holds the email regex.
    ///
    /// # Panics
    ///
    /// This method will panic if the email address is invalid or if the user could not be found or created.
    async fn find_or_create_user(
        &self,
        default_user_config: DefaultUserConfig,
        roles: Option<Vec<String>>,
        salt: &str,
        email_regex: Regex,
    ) {
        if !email_regex.is_match(&default_user_config.email) {
            panic!("Invalid email address");
        }

        match self
            .services
            .user_service
            .find_by_username(&default_user_config.username, &self.database)
            .await
        {
            Ok(user) => {
                if user.is_some() {
                    return;
                }
            }
            Err(e) => {
                panic!("Failed to find user: {:?}", e);
            }
        }

        match self
            .services
            .user_service
            .find_by_email(&default_user_config.email, &self.database)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    let user = User::new(
                        default_user_config.username,
                        default_user_config.email,
                        "".to_string(),
                        "".to_string(),
                        default_user_config.password,
                        roles,
                        default_user_config.enabled,
                        salt,
                    );
                    match self
                        .services
                        .user_service
                        .create(user, &self.database)
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => panic!("Failed to create user: {:?}", e),
                    }
                }
            }
            Err(e) => panic!("Failed to find user: {:?}", e),
        }
    }

    /// # Summary
    ///
    /// Create default indexes for the Permission collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_permission_indexes(&self) {
        info!("Creating indexes for the Permission collection");
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": 1u32 })
            .options(options)
            .build();

        self.database
            .collection::<Permission>(&self.permission_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": "text", "_id": "text" })
            .options(options)
            .build();

        self.database
            .collection::<Permission>(&self.permission_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");
    }

    /// # Summary
    ///
    /// Create default indexes for the Role collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_role_indexes(&self) {
        info!("Creating indexes for the Role collection");
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": 1u32 })
            .options(options)
            .build();

        self.database
            .collection::<Role>(&self.role_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": "text", "_id": "text" })
            .options(options)
            .build();

        self.database
            .collection::<Role>(&self.role_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");
    }

    /// # Summary
    ///
    /// Create default indexes for the User collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_user_indexes(&self) {
        info!("Creating indexes for the User collection");
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<Permission>(&self.user_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "email": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<Permission>(&self.user_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "_id": "text", "username": "text", "email": "text", "firstName": "text", "lastName": "text"})
            .options(options)
            .build();

        self.database
            .collection::<Permission>(&self.user_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");
    }

    /// # Summary
    ///
    /// Initialize the database.
    ///
    /// # Arguments
    ///
    /// * `default_user_config` - A DefaultUserConfig instance that holds the default user configuration.
    /// * `email_regex` - A Regex instance that holds the email regex.
    ///
    /// # Panics
    ///
    /// This method will panic if the email address is invalid, if the database connection is invalid or if permissions, roles or users could not be created.
    pub async fn initialize_database(
        &self,
        default_user_config: DefaultUserConfig,
        email_regex: Regex,
    ) {
        let create_permission = self
            .find_or_create_permission(
                "CAN_CREATE_PERMISSION",
                Some("The ability to create permissions".to_string()),
            )
            .await;
        let read_permission = self
            .find_or_create_permission(
                "CAN_READ_PERMISSION",
                Some("The ability to read permissions".to_string()),
            )
            .await;
        let update_permission = self
            .find_or_create_permission(
                "CAN_UPDATE_PERMISSION",
                Some("The ability to update permissions".to_string()),
            )
            .await;
        let delete_permission = self
            .find_or_create_permission(
                "CAN_DELETE_PERMISSION",
                Some("The ability to delete permissions".to_string()),
            )
            .await;

        let create_role = self
            .find_or_create_permission(
                "CAN_CREATE_ROLE",
                Some("The ability to create roles".to_string()),
            )
            .await;
        let read_role = self
            .find_or_create_permission(
                "CAN_READ_ROLE",
                Some("The ability to read roles".to_string()),
            )
            .await;
        let update_role = self
            .find_or_create_permission(
                "CAN_UPDATE_ROLE",
                Some("The ability to update roles".to_string()),
            )
            .await;
        let delete_delete = self
            .find_or_create_permission(
                "CAN_DELETE_ROLE",
                Some("The ability to delete roles".to_string()),
            )
            .await;

        let create_user = self
            .find_or_create_permission(
                "CAN_CREATE_USER",
                Some("The ability to create users".to_string()),
            )
            .await;
        let read_user = self
            .find_or_create_permission(
                "CAN_READ_USER",
                Some("The ability to read users".to_string()),
            )
            .await;
        let update_user = self
            .find_or_create_permission(
                "CAN_UPDATE_USER",
                Some("The ability to update users".to_string()),
            )
            .await;
        let delete_user = self
            .find_or_create_permission(
                "CAN_DELETE_USER",
                Some("The ability to delete users".to_string()),
            )
            .await;

        let can_update_self = self
            .find_or_create_permission(
                "CAN_UPDATE_SELF",
                Some("The ability to update your own user".to_string()),
            )
            .await;

        let admin_role = self
            .find_or_create_role(
                "ADMIN",
                Some("The administrator role".to_string()),
                Some(vec![
                    create_permission.id.to_string(),
                    read_permission.id.to_string(),
                    update_permission.id.to_string(),
                    delete_permission.id.to_string(),
                    create_role.id.to_string(),
                    read_role.id.to_string(),
                    update_role.id.to_string(),
                    delete_delete.id.to_string(),
                    create_user.id.to_string(),
                    read_user.id.to_string(),
                    update_user.id.to_string(),
                    delete_user.id.to_string(),
                    can_update_self.id.to_string(),
                ]),
            )
            .await;

        let default_role = self
            .find_or_create_role(
                "DEFAULT",
                Some("The default role".to_string()),
                Some(vec![can_update_self.id.to_string()]),
            )
            .await;

        self.find_or_create_user(
            default_user_config,
            Some(vec![admin_role.id.to_string(), default_role.id.to_string()]),
            &self.salt,
            email_regex,
        )
        .await;
    }
}
