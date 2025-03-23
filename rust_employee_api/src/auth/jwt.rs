use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID or username
    pub exp: usize, // Expiration time (Unix timestamp)
}

const SECRET: &[u8] = b"your_secret_key"; // Replace with your actual secret

pub fn create_jwt(user_id: String) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60)) // Token expires in 1 hour
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let header = Header::default();
    encode(&header, &claims, &EncodingKey::from_secret(SECRET))
}

pub fn validate_jwt(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    ).map(|c| c.claims)
}