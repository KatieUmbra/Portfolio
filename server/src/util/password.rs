use crate::{LoginData, UserData};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use zeroize::Zeroize;

use super::error::ApiError;

pub fn hash_pwd(user: &mut UserData) -> Result<(), ApiError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed = argon2.hash_password(&user.password.as_bytes(), &salt);

    user.password.zeroize();

    if let Ok(pwd) = hashed {
        user.password = pwd.to_string();
        Ok(())
    } else {
        Err(ApiError {
            message: "Something went wrong, try again later".into(),
            error_code: None,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

pub fn verify_pwd(hashed: &String, user: &mut LoginData) -> Result<(), ApiError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hashed).map_err(|_| ApiError {
        message: "An internal error has occurred, please contact support".into(),
        error_code: None,
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    let _ = argon2
        .verify_password(&user.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError {
            message: "The provided password is not correct".into(),
            error_code: None,
            status_code: StatusCode::UNAUTHORIZED,
        })?;
    user.password.zeroize();
    Ok(())
}
