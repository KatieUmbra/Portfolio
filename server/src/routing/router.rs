use axum::{
    routing::{get, post, put},
    Router,
};

use crate::util::state::AppState;

use super::routes::user::*;

/// Utility function that adds all routes to a router that will later be used in an axum app
pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/info", get(info))
        .route("/reqEmailVerify", get(req_email_verify))
        .route("/verify", put(verify))
        .with_state(state)
}
