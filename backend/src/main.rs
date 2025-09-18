use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use portfolio_backend::{database, routes};
use sqlx::SqlitePool;
use std::env;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:data/portfolio.db".to_string());

    // Create database connection pool
    let pool = SqlitePool::connect(&database_url).await?;

    // Run database migrations
    database::migrations::initialize_database(pool.clone()).await?;

    // Seed database if needed
    if env::var("SEED_DATABASE").unwrap_or_default() == "true" {
        database::seed::seed_database(&pool).await?;
    }

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>()?,
            "http://localhost:5173".parse::<HeaderValue>()?,
            "http://localhost:5174".parse::<HeaderValue>()?,
            "http://localhost:5175".parse::<HeaderValue>()?,
            "http://localhost:5176".parse::<HeaderValue>()?,
            "http://localhost:5177".parse::<HeaderValue>()?,
            "http://127.0.0.1:3000".parse::<HeaderValue>()?,
            "http://127.0.0.1:5173".parse::<HeaderValue>()?,
            "http://127.0.0.1:5174".parse::<HeaderValue>()?,
            "http://127.0.0.1:5175".parse::<HeaderValue>()?,
            "http://127.0.0.1:5176".parse::<HeaderValue>()?,
            "http://127.0.0.1:5177".parse::<HeaderValue>()?,
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(false);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Portfolio Backend API v1.0" }))
        .route("/health", get(health_check))
        .merge(routes::create_router(pool))
        .layer(cors);

    // Get port from environment or use default
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Run the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🚀 Portfolio Backend API running on http://{}", addr);
    println!("📊 Health check available at http://{}/health", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}