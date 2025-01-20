use crate::database::schema::login_data::LoginData;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use zeroize::Zeroize;
use zxcvbn::zxcvbn;

use super::error::{generic_error, ApiError, ApiErrorCode};

/// Hashes the string utilizing the argon algorithm
/// ## Notes
/// the input [&mut String] will be turned into zeroes when it returns
pub fn hash_str(str: &mut String) -> Result<String, ApiError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed = argon2
        .hash_password(&str.as_bytes(), &salt)
        .map_err(|_| generic_error(ApiErrorCode::InternalPwdhshError))?;

    str.zeroize();
    Ok(hashed.to_string())
}

/// Verifies that some string is a valid password for the provided user [LoginData]
/// ## Notes
/// the input [&mut LoginData] will have it's [LoginData::password] turned into zeroes.
pub fn verify_str(hashed: &String, user: &mut LoginData) -> Result<(), ApiError> {
    let argon2 = Argon2::default();
    let parsed_hash =
        PasswordHash::new(&hashed).map_err(|_| generic_error(ApiErrorCode::InternalPwdhshError))?;
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

/// Verifies that an unhashed password string implements the following requirements:
/// - is at least 8 characters long
/// - is less than 40 characters long
/// - contains at least 1 special character (!@#$%^&*()-=_+)
/// - contains at least 1 lowercase letter
/// - contains at least 1 uppercase letter
pub fn verify_password_requirements(password: &String) -> Result<(), ApiError> {
    let estimate = zxcvbn(password, &[]);
    let score: u8 = estimate.score().into();
    if score < 3 {
        return Err(ApiError {
            message: "Password is too insecure!".into(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: ApiErrorCode::RegisterInsecurePassword,
        });
    }
    Ok(())
}
