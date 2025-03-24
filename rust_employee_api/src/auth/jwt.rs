use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, Validation, DecodingKey, errors::Error, errors::ErrorKind};
use std::env; // Import the env module

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Username
    pub exp: usize, // Expiration time (Unix timestamp)
}

fn get_secret_key() -> Vec<u8> {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set").into_bytes()
}

pub fn validate_jwt(token: &str) -> Result<Claims, Error> {
    let validation = Validation::default();
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(&get_secret_key()),
        &validation,
    ) {
        Ok(c) => Ok(c.claims),
        Err(e) => {
            match e.kind() {
                ErrorKind::ExpiredSignature => Err(Error::from(ErrorKind::ExpiredSignature)),
                _ => Err(e),
            }
        }
    }
}