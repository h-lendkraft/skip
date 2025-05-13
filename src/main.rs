mod error;
mod speed;

use axum::Router;
use std::env;
use std::sync::Arc;
use tokio::signal;
use tokio::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug)]
pub struct AppState {
    pub speed: Arc<SpeedState>,
}
use crate::speed::SpeedState;
impl From<AppState> for Arc<SpeedState> {
    fn from(state: AppState) -> Self {
        state.speed
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .init();
    let server_port = env::var("PORT").unwrap();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::limited(10))
        .cookie_store(true)
        .build()
        .unwrap();

    // region start: --- SPEED
    let speed_user = std::env::var("SPEED_USER").unwrap();
    let speed_pass = std::env::var("SPEED_PASSWD").unwrap();
    let speed_url = std::env::var("SPEED_BASE_URL").unwrap();
    let speed_search_append = std::env::var("SPEED_SEARCH_APPEND").unwrap();

    let speed_state = SpeedState::new(
        client,
        speed_url,
        speed_search_append,
        speed_pass,
        speed_user,
    );
    // region end:   --- SPEED

    let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    // Build router with all routes
    let app = Router::new()
        .merge(speed::handlers::routes(speed_state))
        .layer(middleware)
        .route("/health", axum::routing::get(health_handler));

    let addr = format!("0.0.0.0:{}", server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    // Run the server with graceful shutdown
    if let Err(e) = axum::serve(listener, app)
        // .with_graceful_shutdown(shutdown_signal(pool, ts))
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        eprintln!("Server error: {}", e);
        std::process::exit(1); // Server failed - exit with error
    }

    // Clean shutdown completed
    tracing::info!("Server shutdown completed");
    std::process::exit(0); // Server shutdown successfully
}

use axum::Json;
use serde_json::Value;
use time::OffsetDateTime;
async fn health_handler() -> Json<Value> {
    let now = OffsetDateTime::now_utc();

    let health_info = serde_json::json!({
        "status": "up",
        "timestamp": now.to_offset(time::macros::offset!(+5:30)).to_string(),
    });

    Json(health_info)
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => tracing::info!("Shutting down via Ctrl+C"),
        _ = terminate => tracing::info!("Shutting down via SIGTERM"),
    }
    tracing::info!("Starting cleanup...");

    tracing::info!("Closing database connections...");

    tracing::info!("Cleanup completed");
}
