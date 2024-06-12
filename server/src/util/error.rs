use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub struct ApiError {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: Option<u8>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;

        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({
                "statusCode": self.status_code.as_u16(),
                "errorCode": self.error_code,
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
            error_code: None,
        }
    }
}
