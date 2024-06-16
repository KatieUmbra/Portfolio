use crate::LoginData;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use zeroize::Zeroize;

use super::error::{ApiError, ApiErrorCode};

pub fn hash_str(str: &mut String) -> Result<String, ApiError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed = argon2
        .hash_password(&str.as_bytes(), &salt)
        .map_err(|_| ApiError {
            message: "An internal error has ocurred, please contact support".into(),
            error_code: ApiErrorCode::InternalErrorContactSupport,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    str.zeroize();
    Ok(hashed.to_string())
}

pub fn verify_str(hashed: &String, user: &mut LoginData) -> Result<(), ApiError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hashed).map_err(|_| ApiError {
        message: "An internal error has occurred, please contact support".into(),
        error_code: ApiErrorCode::InternalErrorContactSupport,
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    let _ = argon2
        .verify_password(&user.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError {
            message: "The provided strings do not match".into(),
            error_code: ApiErrorCode::UtilMismatchString,
            status_code: StatusCode::BAD_REQUEST,
        })?;
    user.password.zeroize();
    Ok(())
}
