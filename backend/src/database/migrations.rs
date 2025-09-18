use sqlx::SqlitePool;
use std::fs;
use std::path::Path;
use tracing::{info, error, warn};

/// Migration error types
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Migration file not found: {0}")]
    FileNotFound(String),
}

/// Migration manager for handling database schema changes
pub struct MigrationManager {
    pool: SqlitePool,
    migrations_dir: String,
}

impl MigrationManager {
    pub fn new(pool: SqlitePool, migrations_dir: String) -> Self {
        Self {
            pool,
            migrations_dir,
        }
    }

    /// Run all pending migrations
    pub async fn run_migrations(&self) -> Result<(), MigrationError> {
        info!("Starting database migrations...");

        // Create migrations table if it doesn't exist
        self.create_migrations_table().await?;

        // Get list of migration files
        let migration_files = self.get_migration_files()?;
        
        if migration_files.is_empty() {
            warn!("No migration files found in {}", self.migrations_dir);
            return Ok(());
        }

        // Run each migration
        for file_path in migration_files {
            let migration_name = Path::new(&file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            if self.is_migration_applied(migration_name).await? {
                info!("Migration {} already applied, skipping", migration_name);
                continue;
            }

            info!("Running migration: {}", migration_name);
            self.run_migration(&file_path, migration_name).await?;
            info!("Migration {} completed successfully", migration_name);
        }

        info!("All migrations completed successfully");
        Ok(())
    }

    /// Create the migrations tracking table
    async fn create_migrations_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get list of migration files sorted by name
    fn get_migration_files(&self) -> Result<Vec<String>, std::io::Error> {
        let migrations_path = Path::new(&self.migrations_dir);
        
        if !migrations_path.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        
        for entry in fs::read_dir(migrations_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }

        files.sort();
        Ok(files)
    }

    /// Check if a migration has already been applied
    async fn is_migration_applied(&self, migration_name: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM _migrations WHERE name = ?",
        )
        .bind(migration_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(result > 0)
    }

    /// Run a single migration file
    async fn run_migration(&self, file_path: &str, migration_name: &str) -> Result<(), MigrationError> {
        // Read migration file
        let sql_content = fs::read_to_string(file_path)?;

        // Execute migration in a transaction
        let mut tx = self.pool.begin().await?;

        // Split SQL content by semicolons and execute each statement
        for statement in sql_content.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                sqlx::query(statement).execute(&mut *tx).await?;
            }
        }

        // Record migration as applied
        sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
            .bind(migration_name)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}

/// Initialize database with migrations
pub async fn initialize_database(pool: SqlitePool) -> Result<(), MigrationError> {
    let migration_manager = MigrationManager::new(pool, "migrations".to_string());
    migration_manager.run_migrations().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn create_test_pool() -> SqlitePool {
        sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_create_migrations_table() {
        let pool = create_test_pool().await;
        let manager = MigrationManager::new(pool.clone(), "test_migrations".to_string());
        
        let result = manager.create_migrations_table().await;
        assert!(result.is_ok());

        // Verify table was created
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='_migrations'"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_migration_applied_check() {
        let pool = create_test_pool().await;
        let manager = MigrationManager::new(pool.clone(), "test_migrations".to_string());
        
        manager.create_migrations_table().await.unwrap();
        
        // Initially should not be applied
        let applied = manager.is_migration_applied("test_migration").await.unwrap();
        assert!(!applied);

        // Insert migration record
        sqlx::query("INSERT INTO _migrations (name) VALUES (?)")
            .bind("test_migration")
            .execute(&pool)
            .await
            .unwrap();

        // Now should be applied
        let applied = manager.is_migration_applied("test_migration").await.unwrap();
        assert!(applied);
    }
}