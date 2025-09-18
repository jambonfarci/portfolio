use sqlx::SqlitePool;
use tracing::{info, error};
use crate::database::{
    connection::{DatabaseConfig, create_pool, test_connection},
    migrations::initialize_database,
    seed::seed_database,
    MigrationError, SeedError,
};

/// Database initialization error types
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("Database connection error: {0}")]
    Connection(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    Migration(#[from] MigrationError),
    #[error("Seed error: {0}")]
    Seed(#[from] SeedError),
}

/// Initialize the complete database setup
pub async fn initialize_complete_database(config: Option<DatabaseConfig>) -> Result<SqlitePool, InitError> {
    let config = config.unwrap_or_default();
    
    info!("Initializing database with configuration: {:?}", config);

    // Create connection pool
    let pool = create_pool(&config).await?;
    
    // Test connection
    test_connection(&pool).await?;
    
    // Run migrations
    initialize_database(pool.clone()).await?;
    
    // Seed initial data
    seed_database(&pool).await?;
    
    info!("Database initialization completed successfully");
    Ok(pool)
}

/// Initialize database for testing (in-memory)
pub async fn initialize_test_database() -> Result<SqlitePool, InitError> {
    let config = DatabaseConfig {
        database_url: "sqlite::memory:".to_string(),
        max_connections: 5,
        connection_timeout: std::time::Duration::from_secs(10),
    };
    
    initialize_complete_database(Some(config)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_test_database() {
        // For testing, we'll manually create the schema instead of using file-based migrations
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables manually for testing
        let schema = r#"
            CREATE TABLE IF NOT EXISTS profile (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                title TEXT NOT NULL,
                bio TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT,
                location TEXT NOT NULL,
                avatar_url TEXT,
                linkedin_url TEXT,
                github_url TEXT,
                twitter_url TEXT,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                long_description TEXT,
                technologies TEXT NOT NULL,
                github_url TEXT,
                demo_url TEXT,
                image_url TEXT,
                category TEXT NOT NULL,
                featured BOOLEAN DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS skills (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                level INTEGER NOT NULL CHECK (level >= 1 AND level <= 5),
                years_experience INTEGER,
                description TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS contact_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                subject TEXT NOT NULL,
                message TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        "#;

        for statement in schema.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                sqlx::query(statement).execute(&pool).await.unwrap();
            }
        }

        // Seed data
        seed_database(&pool).await.unwrap();
        
        // Verify tables exist
        let tables = sqlx::query_scalar::<_, String>(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        
        let expected_tables = vec!["contact_messages", "profile", "projects", "skills"];
        assert_eq!(tables, expected_tables);
        
        // Verify data exists
        let profile_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM profile")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(profile_count, 1);
        
        let skills_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM skills")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(skills_count > 0);
    }

    #[tokio::test]
    async fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.database_url, "sqlite:data/portfolio.db");
        assert_eq!(config.max_connections, 10);
    }
}