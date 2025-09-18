pub mod projects;
pub mod skills;
pub mod profile;
pub mod contact;

use axum::Router;
use sqlx::SqlitePool;

/// Create the main API router with all routes
pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .nest("/api/projects", projects::create_routes(pool.clone()))
        .nest("/api/skills", skills::create_routes(pool.clone()))
        .nest("/api/profile", profile::create_routes(pool.clone()))
        .nest("/api/contact", contact::create_routes(pool))
}