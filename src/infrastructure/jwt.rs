// # JWT logic
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}