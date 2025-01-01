use axum::http::StatusCode;

use crate::util::{
    error::{ApiError, ApiErrorCode},
    jwt::Claims,
};

use super::structs::ApiResult;

pub async fn post(claims: Claims) -> ApiResult {
    if claims.rank != 0 {
        return Err(ApiError {
            message: "You're not allowed to post to this site.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::BlogUnauthorized,
        });
    }
    Ok(())
}

pub async fn comment(claims: Claims) -> ApiResult {
    if claims.rank > 1 {
        return Err(ApiError {
            message: "You need to verify your account to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::AccountUnverified,
        });
    }
    Ok(())
}

pub async fn like(claims: Claims) -> ApiResult {
    if claims.rank > 1 {
        return Err(ApiError {
            message: "You need to verify your account to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::AccountUnverified,
        });
    }
    Ok(())
}

pub async fn delete(claims: Claims) -> ApiResult {
    if claims.rank > 1 {
        return Err(ApiError {
            message: "You need to verify your account to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::AccountUnverified,
        });
    }
    Ok(())
}
