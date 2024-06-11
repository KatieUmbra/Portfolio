use crate::AppState;

use super::routes::{info, login, register};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/info", get(info))
}
