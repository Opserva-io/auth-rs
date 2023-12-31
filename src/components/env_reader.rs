use crate::configuration::config::Config;
use crate::configuration::db_config::DbConfig;
use crate::configuration::default_user_config::DefaultUserConfig;
use crate::configuration::jwt_config::JwtConfig;
use crate::configuration::server_config::ServerConfig;
use log::info;
use std::env;

pub struct EnvReader {}

impl EnvReader {
    /// # Summary
    ///
    /// Reads the configuration from the environment variables.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::components::env_reader::EnvReader;
    /// use crate::configuration::config::Config;
    ///
    /// let config = EnvReader::read_configuration().await;
    /// ```
    ///
    /// # Returns
    ///
    /// A Config instance.
    pub async fn read_configuration() -> Config {
        info!("Reading configuration from environment variables");

        let addr = match env::var("SERVER_ADDR") {
            Ok(d) => d,
            Err(_) => String::from("0.0.0.0"),
        };

        let port = match env::var("SERVER_PORT") {
            Ok(d) => {
                let res: u16 = d.trim().parse().expect("PORT must be a number");
                res
            }
            Err(_) => 8080,
        };

        let max_limit = match env::var("MAX_FETCH_LIMIT") {
            Ok(d) => {
                let res: i64 = d.trim().parse().expect("MAX_FETCH_LIMIT must be a number");
                res
            }
            Err(_) => 100,
        };

        let workers = match env::var("SERVER_WORKERS") {
            Ok(d) => {
                let res: usize = d
                    .trim()
                    .parse()
                    .expect("SERVER_WORKERS must be a valid usize");
                res
            }
            Err(_) => 0,
        };

        let conn_string = match env::var("DB_CONNECTION_STRING") {
            Ok(d) => d,
            Err(_) => panic!("No connection string specified"),
        };

        let database = match env::var("DB_DATABASE") {
            Ok(d) => d,
            Err(_) => panic!("No database specified"),
        };

        let permission_collection = match env::var("DB_PERMISSION_COLLECTION") {
            Ok(d) => d,
            Err(_) => String::from("permissions"),
        };

        let role_collection = match env::var("DB_ROLE_COLLECTION") {
            Ok(d) => d,
            Err(_) => String::from("roles"),
        };

        let user_collection = match env::var("DB_USER_COLLECTION") {
            Ok(d) => d,
            Err(_) => String::from("users"),
        };

        let audit_collection = match env::var("DB_AUDIT_COLLECTION") {
            Ok(d) => d,
            Err(_) => String::from("audits"),
        };

        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(d) => d,
            Err(_) => panic!("No JWT secret specified"),
        };

        let jwt_expiration = match env::var("JWT_EXPIRATION") {
            Ok(d) => {
                let res: usize = d.trim().parse().expect("JWT_EXPIRATION must be a number");
                res
            }
            Err(_) => 3600,
        };

        let generate_default_user = match env::var("GENERATE_DEFAULT_USER") {
            Ok(d) => {
                let res: bool = d
                    .trim()
                    .parse()
                    .expect("GENERATE_DEFAULT_USER must be a boolean");
                res
            }
            Err(_) => true,
        };

        let mut default_username = String::new();
        let mut default_email = Some(String::new());
        let mut default_password = String::new();
        let mut default_user_enabled = false;

        if generate_default_user {
            default_username = match env::var("DEFAULT_USER_USERNAME") {
                Ok(d) => d,
                Err(_) => panic!("No default username specified"),
            };

            default_email = match env::var("DEFAULT_USER_EMAIL") {
                Ok(d) => Some(d),
                Err(_) => None,
            };

            default_password = match env::var("DEFAULT_USER_PASSWORD") {
                Ok(d) => d,
                Err(_) => panic!("No default password specified"),
            };

            default_user_enabled = match env::var("DEFAULT_USER_ENABLED") {
                Ok(d) => {
                    let res: bool = d
                        .trim()
                        .parse()
                        .expect("DEFAULT_USER_ENABLED must be a boolean");
                    res
                }
                Err(_) => panic!("No default user enabled specified"),
            };
        }

        let audit_enabled = match env::var("DB_AUDIT_ENABLED") {
            Ok(d) => {
                let res: bool = d
                    .trim()
                    .parse()
                    .expect("DB_AUDIT_ENABLED must be a boolean");
                res
            }
            Err(_) => false,
        };

        let audit_ttl = match env::var("DB_AUDIT_TTL") {
            Ok(d) => {
                let res: u64 = d.trim().parse().expect("DB_AUDIT_TTL must be a number");
                res
            }
            Err(_) => 0,
        };

        let create_indexes = match env::var("DB_CREATE_INDEXES") {
            Ok(d) => {
                let res: bool = d
                    .trim()
                    .parse()
                    .expect("DB_CREATE_INDEXES must be a boolean");
                res
            }
            Err(_) => true,
        };

        let enable_openapi = match env::var("ENABLE_OPENAPI") {
            Ok(d) => {
                let res: bool = d.trim().parse().expect("ENABLE_OPENAPI must be a boolean");
                res
            }
            Err(_) => true,
        };

        let default_user_config = DefaultUserConfig::new(
            default_username,
            default_email,
            default_password,
            default_user_enabled,
        );

        let db_config = DbConfig::new(
            conn_string,
            database,
            permission_collection,
            role_collection,
            user_collection,
            audit_collection,
            create_indexes,
            audit_enabled,
            audit_ttl,
        );

        let server_config = ServerConfig::new(addr, port, max_limit, workers);

        Config::new(
            server_config,
            db_config,
            default_user_config,
            generate_default_user,
            JwtConfig::new(jwt_secret, jwt_expiration),
            enable_openapi,
        )
        .await
    }
}
