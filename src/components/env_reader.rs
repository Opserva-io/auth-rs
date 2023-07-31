use crate::configuration::collection_config::CollectionConfig;
use crate::configuration::config::Config;
use crate::configuration::default_user_config::DefaultUserConfig;
use crate::configuration::jwt_config::JwtConfig;
use std::env;

pub struct EnvReader {}

impl EnvReader {
    /// # Summary
    ///
    /// Reads the configuration from the environment variables.
    ///
    /// # Returns
    ///
    /// A Config instance.
    pub async fn read_configuration() -> Config {
        let addr = match env::var("SERVER_ADDR") {
            Ok(d) => d,
            Err(_) => panic!("No address specified"),
        };

        let port = match env::var("SERVER_PORT") {
            Ok(d) => {
                let res: u16 = d.trim().parse().expect("PORT must be a number");
                res
            }
            Err(_) => panic!("No port specified"),
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
            Err(_) => panic!("No permission collection specified"),
        };

        let role_collection = match env::var("DB_ROLE_COLLECTION") {
            Ok(d) => d,
            Err(_) => panic!("No role collection specified"),
        };

        let user_collection = match env::var("DB_USER_COLLECTION") {
            Ok(d) => d,
            Err(_) => panic!("No user collection specified"),
        };

        let salt = match env::var("HASH_SALT") {
            Ok(d) => d,
            Err(_) => panic!("No salt specified"),
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
            Err(_) => panic!("No JWT expiration specified"),
        };

        let collection_config =
            CollectionConfig::new(permission_collection, role_collection, user_collection);

        let default_username = match env::var("DEFAULT_USER_USERNAME") {
            Ok(d) => d,
            Err(_) => panic!("No default username specified"),
        };

        let default_email = match env::var("DEFAULT_USER_EMAIL") {
            Ok(d) => d,
            Err(_) => panic!("No default email specified"),
        };

        let default_password = match env::var("DEFAULT_USER_PASSWORD") {
            Ok(d) => d,
            Err(_) => panic!("No default password specified"),
        };

        let default_user_enabled = match env::var("DEFAULT_USER_ENABLED") {
            Ok(d) => {
                let res: bool = d
                    .trim()
                    .parse()
                    .expect("DEFAULT_USER_ENABLED must be a boolean");
                res
            }
            Err(_) => panic!("No default user enabled specified"),
        };

        let generate_default_user = match env::var("GENERATE_DEFAULT_USER") {
            Ok(d) => {
                let res: bool = d
                    .trim()
                    .parse()
                    .expect("GENERATE_DEFAULT_USER must be a boolean");
                res
            }
            Err(_) => panic!("No generate default user specified"),
        };

        let default_user_config = DefaultUserConfig::new(
            default_username,
            default_email,
            default_password,
            default_user_enabled,
        );

        Config::new(
            addr,
            port,
            &conn_string,
            &database,
            collection_config,
            default_user_config,
            generate_default_user,
            salt,
            JwtConfig::new(jwt_secret, jwt_expiration),
        )
        .await
    }
}
