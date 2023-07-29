use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
    pub secret: String,
    pub expiration: usize,
}

impl JwtService {
    /// # Summary
    ///
    /// Create a new JwtService.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret to use for signing and verifying JWTs.
    /// * `expiration` - The expiration time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let jwt_service = JwtService::new(String::from("secret"), 3600);
    /// ```
    ///
    /// # Returns
    ///
    /// * `JwtService` - The new JwtService.
    pub fn new(secret: String, expiration: usize) -> JwtService {
        JwtService { secret, expiration }
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
        let exp = now + chrono::Duration::seconds(self.expiration as i64);
        let iat = now;

        let claims = Claims::new(
            String::from(subject),
            exp.timestamp() as usize,
            iat.timestamp() as usize,
        );

        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret.as_bytes()),
        ) {
            Ok(t) => Some(t),
            Err(_) => None,
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
            &DecodingKey::from_secret(&self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        );

        match token_data {
            Ok(t) => Ok(t.claims.sub),
            Err(e) => Err(Error::InvalidToken(e.to_string())),
        }
    }
}
