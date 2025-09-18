use sqlx::SqlitePool;
use chrono::Utc;
use crate::models::{ContactMessage, CreateContactMessage};

/// Repository for contact message database operations
pub struct ContactRepository {
    pool: SqlitePool,
}

impl ContactRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get all contact messages
    pub async fn get_all(&self) -> Result<Vec<ContactMessage>, sqlx::Error> {
        sqlx::query_as::<_, ContactMessage>(
            "SELECT id, name, email, subject, message, created_at FROM contact_messages ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get contact message by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<ContactMessage>, sqlx::Error> {
        sqlx::query_as::<_, ContactMessage>(
            "SELECT id, name, email, subject, message, created_at FROM contact_messages WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Create a new contact message
    pub async fn create(&self, message: &CreateContactMessage) -> Result<ContactMessage, sqlx::Error> {
        let now = Utc::now();
        
        let result = sqlx::query(
            "INSERT INTO contact_messages (name, email, subject, message, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&message.name)
        .bind(&message.email)
        .bind(&message.subject)
        .bind(&message.message)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid() as i32;
        
        // Fetch the created message
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    /// Delete a contact message
    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM contact_messages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get messages with pagination
    pub async fn get_paginated(&self, limit: i64, offset: i64) -> Result<Vec<ContactMessage>, sqlx::Error> {
        sqlx::query_as::<_, ContactMessage>(
            "SELECT id, name, email, subject, message, created_at FROM contact_messages ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Count total messages
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM contact_messages")
            .fetch_one(&self.pool)
            .await
    }

    /// Get recent messages (within last N days)
    pub async fn get_recent(&self, days: i64) -> Result<Vec<ContactMessage>, sqlx::Error> {
        let cutoff_date = Utc::now() - chrono::Duration::days(days);
        
        sqlx::query_as::<_, ContactMessage>(
            "SELECT id, name, email, subject, message, created_at FROM contact_messages WHERE created_at >= ? ORDER BY created_at DESC"
        )
        .bind(cutoff_date)
        .fetch_all(&self.pool)
        .await
    }

    /// Search messages by email or name
    pub async fn search(&self, query: &str) -> Result<Vec<ContactMessage>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);
        
        sqlx::query_as::<_, ContactMessage>(
            "SELECT id, name, email, subject, message, created_at FROM contact_messages WHERE name LIKE ? OR email LIKE ? OR subject LIKE ? ORDER BY created_at DESC"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await
    }

    /// Get messages by email address
    pub async fn get_by_email(&self, email: &str) -> Result<Vec<ContactMessage>, sqlx::Error> {
        sqlx::query_as::<_, ContactMessage>(
            "SELECT id, name, email, subject, message, created_at FROM contact_messages WHERE email = ? ORDER BY created_at DESC"
        )
        .bind(email)
        .fetch_all(&self.pool)
        .await
    }

    /// Delete old messages (older than N days)
    pub async fn delete_old(&self, days: i64) -> Result<u64, sqlx::Error> {
        let cutoff_date = Utc::now() - chrono::Duration::days(days);
        
        let result = sqlx::query("DELETE FROM contact_messages WHERE created_at < ?")
            .bind(cutoff_date)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn create_test_repository() -> ContactRepository {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables manually for testing
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS contact_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                subject TEXT NOT NULL,
                message TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        ContactRepository::new(pool)
    }

    fn create_test_message() -> CreateContactMessage {
        CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message with sufficient content for testing purposes.".to_string(),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_message() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        let created = repo.create(&message_data).await.unwrap();
        assert_eq!(created.name, message_data.name);
        assert_eq!(created.email, message_data.email);
        assert_eq!(created.subject, message_data.subject);
        assert_eq!(created.message, message_data.message);

        let retrieved = repo.get_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, created.name);
    }

    #[tokio::test]
    async fn test_get_all_messages() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        repo.create(&message_data).await.unwrap();
        
        let messages = repo.get_all().await.unwrap();
        assert!(messages.len() >= 1);
    }

    #[tokio::test]
    async fn test_delete_message() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        let created = repo.create(&message_data).await.unwrap();
        
        let deleted = repo.delete(created.id).await.unwrap();
        assert!(deleted);
        
        let retrieved = repo.get_by_id(created.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_get_paginated() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        // Create multiple messages
        for i in 0..5 {
            let mut msg = message_data.clone();
            msg.subject = format!("Test Subject {}", i);
            repo.create(&msg).await.unwrap();
        }
        
        let messages = repo.get_paginated(3, 0).await.unwrap();
        assert!(messages.len() <= 3);
    }

    #[tokio::test]
    async fn test_count_messages() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        let initial_count = repo.count().await.unwrap();
        repo.create(&message_data).await.unwrap();
        let new_count = repo.count().await.unwrap();
        
        assert_eq!(new_count, initial_count + 1);
    }

    #[tokio::test]
    async fn test_search_messages() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        repo.create(&message_data).await.unwrap();
        
        let results = repo.search("John").await.unwrap();
        assert!(results.len() >= 1);
        assert!(results.iter().any(|m| m.name.contains("John")));
    }

    #[tokio::test]
    async fn test_get_by_email() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        repo.create(&message_data).await.unwrap();
        
        let messages = repo.get_by_email("john.doe@example.com").await.unwrap();
        assert!(messages.len() >= 1);
        assert!(messages.iter().all(|m| m.email == "john.doe@example.com"));
    }

    #[tokio::test]
    async fn test_get_recent() {
        let repo = create_test_repository().await;
        let message_data = create_test_message();
        
        repo.create(&message_data).await.unwrap();
        
        let recent = repo.get_recent(1).await.unwrap();
        assert!(recent.len() >= 1);
    }
}