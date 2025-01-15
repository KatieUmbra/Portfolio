use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::util::state::AppState;

use super::routes::{
    blog::{get_latest, get_post, post_to_blog},
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
        .route("/blog/post", post(post_to_blog))
        .route("/blog/get", get(get_post))
        .route("/blog/get_latest", get(get_latest))
        .route("/blog/delete", delete(super::routes::blog::delete))
        .with_state(state)
}
