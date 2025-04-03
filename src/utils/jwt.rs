use std::env;

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::{dto::jwt_dto::Claims, error::AppError};

pub fn generate_tokken(id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let encoding_key = EncodingKey::from_secret(key.as_bytes());
    let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let obj = Claims {
        sub: id.to_string(),
        exp: expiration,
    };
    let token = encode(&Header::default(), &obj, &encoding_key);
    token
}

pub fn decode_token(token: &str) -> TokenData<Claims> {
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let dencoding_key = DecodingKey::from_secret(key.as_bytes());
    let token_data = decode(token, &dencoding_key, &Validation::default())
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()));
    token_data.unwrap()
}
