use std::env;

use chrono;
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn sign_jwt(id: Uuid) -> JwtResult<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("Valid Timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    )
}

pub fn decode_jwt(token: String) -> JwtResult<TokenData<Claims>> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::default(),
    )
}

pub fn get_secret() -> String {
    return env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}
