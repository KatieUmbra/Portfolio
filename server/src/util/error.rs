use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

/// Enum containing serializable versions of internal error codes
#[derive(Serialize, Clone)]
pub enum ApiErrorCode {
    None = 0,
    RegisterEmailExists = 1,
    RegisterUsernameExists = 2,
    RegisterInvalidEmailToken = 3,
    RegisterInsecurePassword = 4,
    LoginUsernameNotFound = 11,
    LoginWrongPassword = 12,
    InternalError = 21,
    InternalErrorContactSupport = 22,
    InternalUnspecifiedError = 23,
    UtilMismatchString = 31,
}

/// Struct used for api errors
#[derive(Clone)]
pub struct ApiError {
    /// An ideally short and simple message that describes the error
    pub message: String,
    /// The http status code
    pub status_code: StatusCode,
    /// The internal error code, utilized for more detailed error handling by the user
    pub error_code: ApiErrorCode,
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
