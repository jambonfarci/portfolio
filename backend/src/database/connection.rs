use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::time::Duration;
use tracing::{info, error};

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub connection_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: "sqlite:data/portfolio.db".to_string(),
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
        }
    }
}

/// Initialize database connection pool
pub async fn create_pool(config: &DatabaseConfig) -> Result<SqlitePool, sqlx::Error> {
    info!("Creating database connection pool with URL: {}", config.database_url);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connection_timeout)
        .connect(&config.database_url)
        .await?;

    info!("Database connection pool created successfully");
    Ok(pool)
}

/// Test database connection
pub async fn test_connection(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Testing database connection...");
    
    let result = sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await;

    match result {
        Ok(_) => {
            info!("Database connection test successful");
            Ok(())
        }
        Err(e) => {
            error!("Database connection test failed: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_create_pool_success() {
        let config = DatabaseConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            connection_timeout: Duration::from_secs(10),
        };

        let pool = create_pool(&config).await;
        assert!(pool.is_ok());
    }

    #[tokio::test]
    async fn test_connection_test() {
        let config = DatabaseConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            connection_timeout: Duration::from_secs(10),
        };

        let pool = create_pool(&config).await.unwrap();
        let result = test_connection(&pool).await;
        assert!(result.is_ok());
    }
}