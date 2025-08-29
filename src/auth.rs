use crate::{config::Config, errors::AppError, models::Claims};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn create_jwt(user_id: &str, email: &str, config: &Config) -> Result<String, AppError> {
    let now = Utc::now();
    let expire = now + Duration::hours(config.jwt_expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str, config: &Config) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn hash_password(password: &str, cost: u32) -> Result<String, AppError> {
    let hash = bcrypt::hash(password, cost)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let is_valid = bcrypt::verify(password, hash)?;
    Ok(is_valid)
}
