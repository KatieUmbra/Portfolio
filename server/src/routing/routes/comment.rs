use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    database::schema::blog_data::{Comment, CommentData, IdWrapper, PostCommentFilter, VecWrapper},
    util::{
        error::{generic_error, ApiError, ApiErrorCode},
        jwt::Claims,
        state::AppState,
    },
};

use super::structs::ApiResult;

pub async fn post_comment(
    State(state): State<AppState>,
    claims: Claims,
    Json(comment_data): Json<CommentData>,
) -> ApiResult {
    tracing::info!("TEST");
    tracing::info!("POST /blog/comment/post data: {:#?}", comment_data);
    if claims.rank > 2 {
        return Err(generic_error(ApiErrorCode::AccountUnverified));
    }
    let mut comment: Comment = comment_data.into();
    comment.creator = claims.username.clone();
    comment.create(&state.pool).await?;
    Ok(())
}

pub async fn get_comment(
    State(state): State<AppState>,
    id: Query<IdWrapper>,
) -> Result<Json<Comment>, ApiError> {
    let id = id.id;
    tracing::info!("GET /blog/comment/get {}", id);
    let comment = Comment::get(id, &state.pool).await?;
    Ok(Json(comment))
}
pub async fn get_latest_comments(
    State(state): State<AppState>,
    params: Query<PostCommentFilter>,
) -> Result<Json<VecWrapper<Comment>>, ApiError> {
    let post = params.post_id;
    let page = params.page;
    let amount = params.amount;
    tracing::info!(
        "GET /blog/comment/get_latest page: {} amount: {}",
        page,
        amount
    );
    let latest = Comment::get_latest(post, ((page - 1) * amount, amount), &state.pool).await?;
    Ok(Json(VecWrapper { vec: latest }))
}
pub async fn edit_comment(
    State(state): State<AppState>,
    id: Query<IdWrapper>,
    claims: Claims,
    Json(comment_data): Json<CommentData>,
) -> ApiResult {
    if claims.rank > 2 {
        return Err(generic_error(ApiErrorCode::AccountUnverified));
    }
    tracing::info!("PUT /blog/comment/edit data: {:#?}", comment_data);
    let mut comment: Comment = comment_data.into();
    comment.creator = claims.username.clone();
    comment.id = id.id;
    comment.update(&state.pool).await?;
    Ok(())
}
pub async fn like_comment(claims: Claims) -> ApiResult {
    if claims.rank > 2 {
        return Err(generic_error(ApiErrorCode::AccountUnverified));
    }
    tracing::info!(
        "POST /blog/comment/like user: {}, post: , parent: ",
        claims.username
    );
    Ok(())
}
pub async fn delete_comment(
    State(state): State<AppState>,
    claims: Claims,
    id: Query<IdWrapper>,
) -> ApiResult {
    let id = id.id;
    if claims.rank > 2 {
        Comment::delete(id, &state.pool, Some(claims.username)).await?;
        return Err(generic_error(ApiErrorCode::AccountUnverified));
    }
    tracing::info!("DELETE /blog/comment/delete id: {}", id);
    Comment::delete(id, &state.pool, None).await?;
    Ok(())
}
