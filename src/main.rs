mod config;
mod domain;
mod handlers;
mod middleware;
mod repositories;
mod routes;
mod services;
mod utils;

use anyhow::Result;
use config::AppConfig;
use repositories::user::UserRepository;
use services::user::UserService;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type AppState = Arc<UserService>;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = AppConfig::load()?;

    // Initialize tracing
    init_tracing(&config)?;

    // Initialize dependencies
    let user_repository = Arc::new(UserRepository::new());
    let user_service = Arc::new(UserService::new(user_repository));

    // Build the application
    let app = routes::create_routes(user_service);

    // Create listener
    let listener = tokio::net::TcpListener::bind(&config.server.address).await?;

    info!("🚀 Server starting on {}", config.server.address);
    info!("📚 Environment: {}", config.environment);
    info!("🔍 Log level: {}", config.logging.level);

    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing(config: &AppConfig) -> Result<()> {
    let level = match config.logging.level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}={}", env!("CARGO_PKG_NAME").replace('-', "_"), level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}