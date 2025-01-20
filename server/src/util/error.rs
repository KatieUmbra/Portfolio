use std::{collections::HashMap, sync::LazyLock};

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

/// Enum containing serializable versions of internal error codes
#[derive(Eq, Hash, PartialEq, Serialize, Clone, Debug, Default)]
pub enum ApiErrorCode {
    #[default]
    // None (ideally don't use)
    None = 000,
    // Register
    RegisterEmailExists = 001,
    RegisterUsernameExists = 002,
    RegisterInvalidEmailToken = 003,
    RegisterInsecurePassword = 004,
    // Login
    LoginUsernameNotFound = 101,
    LoginWrongPassword = 102,
    // Internal
    InternalError = 201,
    InternalErrorContactSupport = 202,
    InternalUnspecifiedError = 203,
    InternalNotFound = 204,
    InternalSqlxError = 205,
    InternalFsError = 206,
    InternalMarkdownError = 207,
    InternalJwtError = 208,
    InternalEmailError = 209,
    InternalPwdhshError = 210,
    //Util
    UtilMismatchString = 301,
    //Blog
    BlogUnauthorized = 401,
    BlogNotFound = 402,
    //Account
    AccountUnverified = 501,
}

/// Struct used for api errors
#[derive(Clone, Debug, Default)]
pub struct ApiError {
    /// An ideally short and simple message that describes the error
    pub message: String,
    /// The http status code
    pub status_code: StatusCode,
    /// The internal error code, utilized for more detailed error handling by the user
    pub error_code: ApiErrorCode,
}

pub static GENERIC_ERRORS: LazyLock<HashMap<ApiErrorCode, ApiError>> = LazyLock::new(|| {
    let mut errors: HashMap<ApiErrorCode, ApiError> = HashMap::new();

    errors.insert(
        ApiErrorCode::None,
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ApiErrorCode::InternalUnspecifiedError,
            message: "An internal error has occurred (please contact support)".into(),
        },
    );
    errors.insert(
        ApiErrorCode::BlogUnauthorized,
        ApiError {
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::BlogUnauthorized,
            message: "You are not the creator of this post!".into(),
        },
    );
    errors.insert(
        ApiErrorCode::InternalFsError,
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ApiErrorCode::InternalFsError,
            message: "An internal error has occurred (fs error)".into(),
        },
    );
    errors.insert(
        ApiErrorCode::AccountUnverified,
        ApiError {
            message: "You need to verify your account to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::AccountUnverified,
        },
    );
    errors.insert(
        ApiErrorCode::InternalSqlxError,
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ApiErrorCode::InternalSqlxError,
            message: "An internal error has occurred (db error)".into(),
        },
    );
    errors.insert(
        ApiErrorCode::InternalJwtError,
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ApiErrorCode::InternalJwtError,
            message: "An internal error has occurred (jwt error)".into(),
        },
    );

    errors
});

pub fn generic_error(code: ApiErrorCode) -> ApiError {
    (&*GENERIC_ERRORS)
        .get(&code)
        .unwrap_or_else(|| {
            panic!("GENERIC ERROR {:?} HAS TO BE IMPLEMENTED", code);
        })
        .clone()
}

/// Implementation of [IntoResponse] for [ApiError], it simply converts the struct into a json
/// string and appends the `"application/json"` header
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;

        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({
                "statusCode": self.status_code.as_u16(),
                "errorCode": self.error_code as u8,
                "message": self.message
            })),
        )
            .into_response()
    }
}

/// Convinience implementation to quickly turn an axum [StatusCode] into an [ApiError]
impl From<StatusCode> for ApiError {
    fn from(code: StatusCode) -> Self {
        ApiError {
            message: "Unspecified Api Error".to_string(),
            status_code: code,
            error_code: ApiErrorCode::InternalUnspecifiedError,
        }
    }
}
