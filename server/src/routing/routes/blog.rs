use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    database::schema::blog_data::{IdWrapper, Post, PostData, RangeParams, VecWrapper},
    util::{
        error::{generic_error, ApiError, ApiErrorCode},
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
    if claims.rank > 1 {
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

pub async fn get_md(
    State(state): State<AppState>,
    id: Query<IdWrapper>,
) -> Result<Json<Post>, ApiError> {
    tracing::info!("GET /blog/get_md {}", id.id);
    Ok(Json(Post::get_md(id.id, &state.pool).await?))
}

pub async fn edit(
    State(state): State<AppState>,
    id: Query<IdWrapper>,
    claims: Claims,
    Json(post_data): Json<PostData>,
) -> ApiResult {
    if claims.rank > 1 {
        return Err(ApiError {
            message: "You're not allowed to post to this site.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::BlogUnauthorized,
        });
    }
    let mut post: Post = post_data.into();
    post.creator = claims.username.clone();
    post.update(id.id, &state.pool).await?;
    tracing::info!("PUT /blog/edit?id={} {}", id.id, claims.username);
    Ok(())
}

pub async fn get_latest(
    State(state): State<AppState>,
    params: Query<RangeParams>,
) -> Result<Json<VecWrapper<Post>>, ApiError> {
    let page = params.page;
    let amount = params.amount;
    tracing::info!("GET /blog/get_latest page: {} amount: {}", page, amount);
    let latest = Post::get_latest(((page - 1) * amount, amount), &state.pool).await?;
    Ok(Json(VecWrapper { vec: latest }))
}

pub async fn like(claims: Claims) -> ApiResult {
    if claims.rank > 2 {
        return Err(generic_error(ApiErrorCode::AccountUnverified));
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
    id: Query<IdWrapper>,
) -> ApiResult {
    tracing::info!("DELETE /blog/delete user: {}", claims.username);
    if claims.rank > 2 {
        Post::delete(id.id, &state.pool, Some(claims.username)).await?;
        return Err(ApiError {
            message: "You are not allowed to do that.".into(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::BlogUnauthorized,
        });
    }
    Post::delete(id.id, &state.pool, None).await?;
    Ok(())
}
