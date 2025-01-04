use axum::{extract::State, http::StatusCode, Json};

use crate::{
    database::schema::blog_data::{IdWrapper, Post},
    util::{
        error::{ApiError, ApiErrorCode},
        jwt::Claims,
        state::AppState,
    },
};

use super::structs::ApiResult;

pub async fn post_to_blog(
    State(state): State<AppState>,
    claims: Claims,
    Json(mut post_data): Json<Post>,
) -> Result<(), ApiError> {
    if claims.rank != 0 {
        return Err(ApiError {
            message: "You're not allowed to post to this site.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::BlogUnauthorized,
        });
    }
    post_data.creator = claims.username.clone();
    post_data.create(&state.pool).await?;
    tracing::info!("POST /blog/post {}", claims.username);
    Ok(())
}

pub async fn get_post(
    State(state): State<AppState>,
    Json(id): Json<IdWrapper>,
) -> Result<Json<Post>, ApiError> {
    Ok(Json(Post::get(id.id, &state.pool).await?))
}

pub async fn comment(claims: Claims) -> ApiResult {
    if claims.rank > 1 {
        return Err(ApiError {
            message: "You need to verify your account to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::AccountUnverified,
        });
    }
    tracing::info!("POST /blog/comment user: {}, post:", claims.username);
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
    tracing::info!(
        "POST /blog/like user: {}, post: , parent: ",
        claims.username
    );
    Ok(())
}

pub async fn delete(
    State(state): State<AppState>,
    claims: Claims,
    Json(id): Json<IdWrapper>,
) -> ApiResult {
    if claims.rank > 1 {
        return Err(ApiError {
            message: "You need to verify your account to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::AccountUnverified,
        });
    }
    Post::delete(id.id, &state.pool).await?;
    tracing::info!("POST /blog/delete user: {}", claims.username);
    Ok(())
}
