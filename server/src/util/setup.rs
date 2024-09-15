use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routing::router::init_router;

use super::state::AppState;

/// Initializes the tracing library for logging in debug mode
pub fn init_tracing() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}

/// Calls all intermediary functions to start the application
pub async fn init_app() -> anyhow::Result<()> {
    init_tracing();
    let state = AppState::init().await?;
    let (host, port) = (state.settings.host.clone(), state.settings.port.clone());
    let bind_address = host + ":" + &port;
    let router = init_router(state);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(bind_address).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
