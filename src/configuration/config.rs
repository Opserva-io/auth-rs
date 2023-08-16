use crate::configuration::db_config::DbConfig;
use crate::configuration::default_user_config::DefaultUserConfig;
use crate::configuration::jwt_config::JwtConfig;
use crate::configuration::server_config::ServerConfig;
use crate::repository::audit::audit_model::Audit;
use crate::repository::audit::audit_repository::AuditRepository;
use crate::repository::permission::permission_model::Permission;
use crate::repository::permission::permission_repository::PermissionRepository;
use crate::repository::role::role_model::Role;
use crate::repository::role::role_repository::RoleRepository;
use crate::repository::user::user_model::User;
use crate::repository::user::user_repository::UserRepository;
use crate::services::audit::audit_service::AuditService;
use crate::services::jwt::jwt_service::JwtService;
use crate::services::password::password_service::PasswordService;
use crate::services::permission::permission_service::PermissionService;
use crate::services::role::role_service::RoleService;
use crate::services::user::user_service::UserService;
use crate::services::Services;
use argon2::password_hash::SaltString;
use log::{error, info};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Database, IndexModel};
use regex::Regex;

#[derive(Clone)]
pub struct Config {
    pub server_config: ServerConfig,
    pub database: Database,
    pub services: Services,
    pub open_api: bool,
}

impl Config {
    /// # Summary
    ///
    /// Creates a new Config instance.
    ///
    /// # Arguments
    ///
    /// * `server_config` - A ServerConfig instance.
    /// * `db_config` - A DbConfig instance.
    /// * `default_user_config` - A DefaultUserConfig instance.
    /// * `generate_default_user` - A bool that indicates whether to generate a default user or not.
    /// * `salt` - A String that holds the salt.
    /// * `jwt_config` - A JwtConfig instance.
    /// * `open_api` - A bool that indicates whether to enable OpenAPI or not.
    ///
    /// # Returns
    ///
    /// A Config instance.
    pub async fn new(
        server_config: ServerConfig,
        db_config: DbConfig,
        default_user_config: DefaultUserConfig,
        generate_default_user: bool,
        salt: String,
        jwt_config: JwtConfig,
        open_api: bool,
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
        let audit_repository = match AuditRepository::new(db_config.audit_collection.clone()) {
            Ok(d) => d,
            Err(e) => panic!("Failed to initialize Audit repository: {:?}", e),
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
        let audit_service = AuditService::new(audit_repository, db_config.audit_enabled);
        let jwt_service = JwtService::new(jwt_config);

        let salt = match SaltString::from_b64(&salt) {
            Ok(s) => s,
            Err(e) => {
                panic!("Failed to generate salt: {}", e);
            }
        };
        let password_service = PasswordService::new(salt);

        let services = Services::new(
            permission_service,
            role_service,
            user_service,
            jwt_service,
            audit_service,
            password_service,
        );

        let cfg = Config {
            server_config,
            database: db,
            services,
            open_api,
        };

        if generate_default_user {
            cfg.initialize_database(default_user_config, email_regex)
                .await;
        }

        if db_config.create_indexes {
            cfg.create_permission_indexes(&db_config.permission_collection)
                .await;
            cfg.create_role_indexes(&db_config.role_collection).await;
            cfg.create_user_indexes(&db_config.user_collection).await;
            cfg.create_audit_indexes(&db_config.audit_collection).await;
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
                        .create(p, None, &self.database, &self.services.audit_service)
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
                    let permission_oid_vec: Option<Vec<ObjectId>> = match permissions {
                        None => None,
                        Some(p_vec) => {
                            let mut oid_vec: Vec<ObjectId> = vec![];
                            for p in p_vec {
                                match ObjectId::parse_str(p) {
                                    Ok(oid) => {
                                        oid_vec.push(oid);
                                    }
                                    Err(e) => {
                                        error!("Invalid ObjectId: {}", e);
                                    }
                                }
                            }
                            Some(oid_vec)
                        }
                    };

                    let new_role = Role::new(name.to_string(), description, permission_oid_vec);
                    match self
                        .services
                        .role_service
                        .create(new_role, None, &self.database, &self.services.audit_service)
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
    /// * `email_regex` - A Regex instance that holds the email regex.
    ///
    /// # Panics
    ///
    /// This method will panic if the email address is invalid or if the user could not be found or created.
    async fn find_or_create_user(
        &self,
        default_user_config: DefaultUserConfig,
        roles: Option<Vec<String>>,
        email_regex: Regex,
    ) {
        if default_user_config.email.is_some()
            && !email_regex.is_match(&default_user_config.email.clone().unwrap())
        {
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
            .find_by_username(&default_user_config.username, &self.database)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    let password_hash = match self
                        .services
                        .password_service
                        .hash_password(default_user_config.password)
                    {
                        Ok(e) => e,
                        Err(e) => {
                            panic!("Failed to hash password: {}", e);
                        }
                    };

                    let user = User::new(
                        default_user_config.username,
                        default_user_config.email,
                        "".to_string(),
                        "".to_string(),
                        password_hash,
                        roles,
                        default_user_config.enabled,
                    );
                    match self
                        .services
                        .user_service
                        .create(user, None, &self.database, &self.services.audit_service)
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
    /// # Arguments
    ///
    /// * `permission_collection` - A string slice that holds the name of the Permission collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_permission_indexes(&self, permission_collection: &str) {
        info!("Creating indexes for the Permission collection");
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": 1u32 })
            .options(options)
            .build();

        self.database
            .collection::<Permission>(permission_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": "text" })
            .options(options)
            .build();

        self.database
            .collection::<Permission>(permission_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");
    }

    /// # Summary
    ///
    /// Create default indexes for the Role collection.
    ///
    /// # Arguments
    ///
    /// * `role_collection` - A string slice that holds the name of the Role collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_role_indexes(&self, role_collection: &str) {
        info!("Creating indexes for the Role collection");
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": 1u32 })
            .options(options)
            .build();

        self.database
            .collection::<Role>(role_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "name": "text" })
            .options(options)
            .build();

        self.database
            .collection::<Role>(role_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");
    }

    /// # Summary
    ///
    /// Create default indexes for the User collection.
    ///
    /// # Arguments
    ///
    /// * `user_collection` - A string slice that holds the name of the User collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_user_indexes(&self, user_collection: &str) {
        info!("Creating indexes for the User collection");
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<User>(user_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "email": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<User>(user_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "username": "text", "email": "text", "firstName": "text", "lastName": "text"})
            .options(options)
            .build();

        self.database
            .collection::<User>(user_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");
    }

    /// # Summary
    ///
    /// Create default indexes for the Audit collection.
    ///
    /// # Arguments
    ///
    /// * `audit_collection` - A string slice that holds the name of the Audit collection.
    ///
    /// # Panics
    ///
    /// This method will panic if the indexes could not be created.
    pub async fn create_audit_indexes(&self, audit_collection: &str) {
        info!("Creating indexes for the Audit collection");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "action": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<Audit>(audit_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "resourceIdType": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<Audit>(audit_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "resourceType": 1u32})
            .options(options)
            .build();

        self.database
            .collection::<Audit>(audit_collection)
            .create_index(model, None)
            .await
            .expect("Creating an index should succeed");

        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "action": "text", "resourceIdType": "text", "resourceType": "text"})
            .options(options)
            .build();

        self.database
            .collection::<Audit>(audit_collection)
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

        let read_audit = self
            .find_or_create_permission(
                "CAN_READ_AUDIT",
                Some("The ability to read audits".to_string()),
            )
            .await;

        let can_update_self = self
            .find_or_create_permission(
                "CAN_UPDATE_SELF",
                Some("The ability to update your own user".to_string()),
            )
            .await;

        let can_delete_self = self
            .find_or_create_permission(
                "CAN_DELETE_SELF",
                Some("The ability to delete your own user".to_string()),
            )
            .await;

        let admin_role = self
            .find_or_create_role(
                "ADMIN",
                Some("The administrator role".to_string()),
                Some(vec![
                    create_permission.id.to_hex(),
                    read_permission.id.to_hex(),
                    update_permission.id.to_hex(),
                    delete_permission.id.to_hex(),
                    create_role.id.to_hex(),
                    read_role.id.to_hex(),
                    update_role.id.to_hex(),
                    delete_delete.id.to_hex(),
                    create_user.id.to_hex(),
                    read_user.id.to_hex(),
                    update_user.id.to_hex(),
                    delete_user.id.to_hex(),
                    read_audit.id.to_hex(),
                ]),
            )
            .await;

        let default_role = self
            .find_or_create_role(
                "DEFAULT",
                Some("The default role".to_string()),
                Some(vec![
                    can_update_self.id.to_hex(),
                    can_delete_self.id.to_hex(),
                ]),
            )
            .await;

        self.find_or_create_user(
            default_user_config,
            Some(vec![admin_role.id.to_hex(), default_role.id.to_hex()]),
            email_regex,
        )
        .await;
    }
}
