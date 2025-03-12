use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::util::state::AppState;

use super::routes::{
    blog::{edit, get_latest, get_md, get_post, post_to_blog},
    comment::{delete_comment, edit_comment, get_comment, get_latest_comments, post_comment},
    user::*,
};

/// Utility function that adds all routes to a router that will later be used in an axum app
pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/info", get(info))
        .route("/reqEmailVerify", get(req_email_verify))
        .route("/verify", put(verify))
        .route("/updateJwt", get(update_jwt))
        .route("/refreshJwt", post(refresh_jwt))
        // Blog stuff
        .route("/blog/post", post(post_to_blog))
        .route("/blog/get", get(get_post))
        .route("/blog/edit", put(edit))
        .route("/blog/get_md", get(get_md))
        .route("/blog/get_latest", get(get_latest))
        .route("/blog/delete", delete(super::routes::blog::delete))
        // Comment stuff
        .route("/blog/comment/post", post(post_comment))
        .route("/blog/comment/get", get(get_comment))
        .route("/blog/comment/edit", put(edit_comment))
        .route("/blog/comment/get_latest", get(get_latest_comments))
        .route("/blog/comment/delete", delete(delete_comment))
        .with_state(state)
}
