use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    database::schema::blog_data::{I32Wrapper, IdWrapper, Post, PostData, VecWrapper},
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
    Json(post_data): Json<PostData>,
) -> Result<(), ApiError> {
    if claims.rank != 0 {
        return Err(ApiError {
            message: "You're not allowed to post to this site.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::BlogUnauthorized,
        });
    }
    let mut post: Post = post_data.into();
    post.creator = claims.username.clone();
    post.create(&state.pool).await?;
    tracing::info!("POST /blog/post {}", claims.username);
    Ok(())
}

pub async fn get_post(
    State(state): State<AppState>,
    id: Query<IdWrapper>,
) -> Result<Json<Post>, ApiError> {
    tracing::info!("GET /blog/get {}", id.id);
    Ok(Json(Post::get(id.id, &state.pool).await?))
}

pub async fn get_latest(
    State(state): State<AppState>,
    it: Query<I32Wrapper>,
) -> Result<Json<VecWrapper<Post>>, ApiError> {
    tracing::info!("GET /blog/get_latest {}", it.amount);
    let latest = Post::get_latest(it.amount, &state.pool).await?;
    Ok(Json(VecWrapper { vec: latest }))
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
