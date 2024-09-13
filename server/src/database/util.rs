use std::collections::HashMap;

use axum::http::StatusCode;

use crate::util::error::{ApiError, ApiErrorCode};

pub fn user_db_map_error(error: sqlx::Error) -> ApiError {
    let err1 = ApiError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: ApiErrorCode::InternalError,
        message: "Something went wrong, try again later".to_string(),
    };

    let db_constraint_error_map: HashMap<&str, ApiError> = HashMap::from([
        (
            "users_pkey",
            ApiError {
                message: "This username already exists!".to_string(),
                error_code: ApiErrorCode::RegisterUsernameExists,
                status_code: StatusCode::CONFLICT,
            },
        ),
        (
            "users_email_key",
            ApiError {
                message: "This email is already in use!".to_string(),
                error_code: ApiErrorCode::RegisterEmailExists,
                status_code: StatusCode::CONFLICT,
            },
        ),
    ]);

    let error = error.into_database_error();
    if let Some(res) = error {
        println!("{:?}", &res);
        if let Some(constraint) = res.constraint() {
            println!("constraint: {:?}", res);
            db_constraint_error_map.get(constraint).unwrap().clone()
        } else {
            err1
        }
    } else {
        err1
    }
}
