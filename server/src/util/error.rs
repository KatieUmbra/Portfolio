use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Clone)]
pub enum ApiErrorCode {
    None = 0,
    RegisterEmailExists = 1,
    RegisterUsernameExists = 2,
    LoginUsernameNotFound = 11,
    LoginWrongPassword = 12,
    InternalError = 21,
    InternalErrorContactSupport = 22,
    InternalUnspecifiedError = 23,
    UtilMismatchString = 31,
}

#[derive(Clone)]
pub struct ApiError {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: ApiErrorCode,
}

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

impl From<StatusCode> for ApiError {
    fn from(code: StatusCode) -> Self {
        ApiError {
            message: "Unspecified Api Error".to_string(),
            status_code: code,
            error_code: ApiErrorCode::InternalUnspecifiedError,
        }
    }
}
