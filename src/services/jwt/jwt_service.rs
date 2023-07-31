use crate::configuration::jwt_config::JwtConfig;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use log::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: String,
}

impl Claims {
    /// # Summary
    ///
    /// Create a new Claims.
    ///
    /// # Arguments
    ///
    /// * `sub` - The subject of the Claims.
    /// * `exp` - The expiration time of the Claims.
    /// * `iat` - The issued at time of the Claims.
    pub fn new(sub: String, exp: usize, iat: usize) -> Claims {
        Claims { sub, exp, iat }
    }
}

pub enum Error {
    InvalidToken(String),
}

impl Display for Error {
    /// # Summary
    ///
    /// Display the Error.
    ///
    /// # Arguments
    ///
    /// * `f` - The Formatter.
    ///
    /// # Example
    ///
    /// ```
    /// let error = Error::InvalidToken(String::from("message"));
    /// println!("{}", error);
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::fmt::Result` - The result of the operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidToken(message) => write!(f, "Invalid token: {}", message),
        }
    }
}

#[derive(Clone)]
pub struct JwtService {
    pub jwt_config: JwtConfig,
}

impl JwtService {
    /// # Summary
    ///
    /// Create a new JwtService.
    ///
    /// # Arguments
    ///
    /// * `jwt_config` - The configuration for the JwtService.
    ///
    /// # Example
    ///
    /// ```
    /// let jwt_service = JwtService::new(jwt_config);
    /// ```
    ///
    /// # Returns
    ///
    /// * `JwtService` - The new JwtService.
    pub fn new(jwt_config: JwtConfig) -> JwtService {
        JwtService { jwt_config }
    }

    /// # Summary
    ///
    /// Generate a JWT token.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject of the JWT token.
    ///
    /// # Example
    ///
    /// ```
    /// let token = jwt_service.generate_jwt_token("subject");
    /// ```
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The JWT token.
    pub fn generate_jwt_token(&self, subject: &str) -> Option<String> {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::seconds(self.jwt_config.jwt_expiration as i64);
        let iat = now;

        let claims = Claims::new(
            String::from(subject),
            exp.timestamp() as usize,
            iat.timestamp() as usize,
        );

        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.jwt_secret.as_bytes()),
        ) {
            Ok(t) => Some(t),
            Err(e) => {
                error!("Error generating JWT token: {}", e.to_string());
                None
            },
        }
    }

    /// # Summary
    ///
    /// Verify a JWT token.
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT token to verify.
    ///
    /// # Example
    ///
    /// ```
    /// let sub = jwt_service.verify_jwt_token("token");
    /// ```
    ///
    /// # Returns
    ///
    /// * `Result<String, Error>` - The result of the operation.
    pub fn verify_jwt_token(&self, token: &str) -> Result<String, Error> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        );

        match token_data {
            Ok(t) => Ok(t.claims.sub),
            Err(e) => {
                error!("Error verifying JWT token: {}", e.to_string());
                Err(Error::InvalidToken(e.to_string()))
            },
        }
    }
}
