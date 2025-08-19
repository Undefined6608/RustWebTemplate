use hello_rust::{config::Config, db::create_pool, routes::create_routes};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hello_rust=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting server with config: {:#?}", config);

    // Create database pool
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Create application
    let app = create_routes(pool, config.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    tracing::info!("Server listening on {}", config.server_address());

    axum::serve(listener, app).await?;

    Ok(())
}
