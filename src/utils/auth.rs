use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub exp: i64,     // expiration
    pub iat: i64,     // issued at
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn generate_jwt(user_id: Uuid, secret: &str) -> Result<String> {
    let claims = Claims::new(user_id);
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    
    encode(&header, &claims, &encoding_key)
        .map_err(AppError::Jwt)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    
    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(AppError::Jwt)
}
