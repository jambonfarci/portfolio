use sqlx::SqlitePool;
use validator::Validate;
use tracing::{info, warn, error};
use crate::{
    database::ContactRepository,
    models::{ContactMessage, CreateContactMessage},
    error::{ApiError, ApiResult},
};

/// Service for contact message-related business logic
pub struct ContactService {
    repository: ContactRepository,
}

impl ContactService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repository: ContactRepository::new(pool),
        }
    }

    /// Submit a new contact message
    pub async fn submit_message(&self, mut message_data: CreateContactMessage) -> ApiResult<ContactMessage> {
        info!("Submitting contact message from: {}", message_data.email);
        
        // Sanitize input data
        message_data.sanitize();
        
        // Validate input data
        if let Err(validation_errors) = message_data.validate() {
            warn!("Validation failed for contact message: {:?}", validation_errors);
            return Err(ApiError::from_validation_errors(validation_errors));
        }

        // Additional business logic validation
        if !message_data.is_valid_content() {
            return Err(ApiError::BadRequest("Message content appears to be invalid".to_string()));
        }

        // Rate limiting check (simple implementation)
        if let Ok(recent_messages) = self.repository.get_by_email(&message_data.email).await {
            let recent_count = recent_messages.iter()
                .filter(|msg| msg.is_recent())
                .count();
            
            if recent_count >= 3 {
                warn!("Rate limit exceeded for email: {}", message_data.email);
                return Err(ApiError::BadRequest("Too many messages sent recently. Please wait before sending another message.".to_string()));
            }
        }

        match self.repository.create(&message_data).await {
            Ok(message) => {
                info!("Successfully created contact message from {} (ID: {})", message.email, message.id);
                
                // Check for potential spam
                if message.is_likely_spam() {
                    warn!("Potential spam message detected from {}: {}", message.email, message.subject);
                }
                
                Ok(message)
            }
            Err(e) => {
                error!("Failed to create contact message from '{}': {}", message_data.email, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get all contact messages (admin only)
    pub async fn get_all_messages(&self) -> ApiResult<Vec<ContactMessage>> {
        info!("Fetching all contact messages");
        
        match self.repository.get_all().await {
            Ok(messages) => {
                info!("Successfully fetched {} contact messages", messages.len());
                Ok(messages)
            }
            Err(e) => {
                error!("Failed to fetch contact messages: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get contact message by ID (admin only)
    pub async fn get_message_by_id(&self, id: i32) -> ApiResult<ContactMessage> {
        info!("Fetching contact message with ID: {}", id);
        
        match self.repository.get_by_id(id).await {
            Ok(Some(message)) => {
                info!("Successfully fetched contact message from: {}", message.email);
                Ok(message)
            }
            Ok(None) => {
                warn!("Contact message with ID {} not found", id);
                Err(ApiError::NotFound(format!("Contact message with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to fetch contact message {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get messages with pagination (admin only)
    pub async fn get_messages_paginated(&self, page: u32, page_size: u32) -> ApiResult<(Vec<ContactMessage>, u64)> {
        info!("Fetching contact messages page {} with size {}", page, page_size);
        
        if page_size == 0 || page_size > 100 {
            return Err(ApiError::BadRequest("Page size must be between 1 and 100".to_string()));
        }

        let offset = (page.saturating_sub(1) * page_size) as i64;
        let limit = page_size as i64;

        match tokio::try_join!(
            self.repository.get_paginated(limit, offset),
            self.repository.count()
        ) {
            Ok((messages, total_count)) => {
                info!("Successfully fetched {} messages (page {}, total: {})", messages.len(), page, total_count);
                Ok((messages, total_count as u64))
            }
            Err(e) => {
                error!("Failed to fetch paginated messages: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Search messages (admin only)
    pub async fn search_messages(&self, query: &str) -> ApiResult<Vec<ContactMessage>> {
        info!("Searching contact messages with query: '{}'", query);
        
        if query.trim().is_empty() {
            return Err(ApiError::BadRequest("Search query cannot be empty".to_string()));
        }

        match self.repository.search(query).await {
            Ok(messages) => {
                info!("Found {} messages matching query '{}'", messages.len(), query);
                Ok(messages)
            }
            Err(e) => {
                error!("Failed to search messages with query '{}': {}", query, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get recent messages (admin only)
    pub async fn get_recent_messages(&self, days: u32) -> ApiResult<Vec<ContactMessage>> {
        info!("Fetching messages from last {} days", days);
        
        if days == 0 || days > 365 {
            return Err(ApiError::BadRequest("Days must be between 1 and 365".to_string()));
        }

        match self.repository.get_recent(days as i64).await {
            Ok(messages) => {
                info!("Successfully fetched {} recent messages", messages.len());
                Ok(messages)
            }
            Err(e) => {
                error!("Failed to fetch recent messages: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Delete a contact message (admin only)
    pub async fn delete_message(&self, id: i32) -> ApiResult<()> {
        info!("Deleting contact message with ID: {}", id);
        
        match self.repository.delete(id).await {
            Ok(true) => {
                info!("Successfully deleted contact message with ID: {}", id);
                Ok(())
            }
            Ok(false) => {
                warn!("Contact message with ID {} not found for deletion", id);
                Err(ApiError::NotFound(format!("Contact message with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to delete contact message {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get message statistics (admin only)
    pub async fn get_message_stats(&self) -> ApiResult<MessageStats> {
        info!("Fetching message statistics");
        
        match tokio::try_join!(
            self.repository.count(),
            self.repository.get_recent(7),
            self.repository.get_recent(30)
        ) {
            Ok((total_count, week_messages, month_messages)) => {
                let stats = MessageStats {
                    total_messages: total_count as u64,
                    messages_this_week: week_messages.len() as u64,
                    messages_this_month: month_messages.len() as u64,
                    spam_messages: month_messages.iter().filter(|m| m.is_likely_spam()).count() as u64,
                };
                
                info!("Successfully calculated message statistics");
                Ok(stats)
            }
            Err(e) => {
                error!("Failed to fetch message statistics: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Clean up old messages (admin only)
    pub async fn cleanup_old_messages(&self, days: u32) -> ApiResult<u64> {
        info!("Cleaning up messages older than {} days", days);
        
        if days < 30 {
            return Err(ApiError::BadRequest("Cannot delete messages newer than 30 days".to_string()));
        }

        match self.repository.delete_old(days as i64).await {
            Ok(deleted_count) => {
                info!("Successfully deleted {} old messages", deleted_count);
                Ok(deleted_count)
            }
            Err(e) => {
                error!("Failed to cleanup old messages: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }
}

/// Message statistics for admin dashboard
#[derive(Debug, serde::Serialize)]
pub struct MessageStats {
    pub total_messages: u64,
    pub messages_this_week: u64,
    pub messages_this_month: u64,
    pub spam_messages: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_service() -> ContactService {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create table
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

        ContactService::new(pool)
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
    async fn test_submit_message() {
        let service = create_test_service().await;
        let message_data = create_test_message();
        
        let submitted = service.submit_message(message_data).await.unwrap();
        assert_eq!(submitted.name, "John Doe");
        assert_eq!(submitted.email, "john.doe@example.com");
        assert_eq!(submitted.subject, "Test Subject");
    }

    #[tokio::test]
    async fn test_submit_message_validation_error() {
        let service = create_test_service().await;
        let mut message_data = create_test_message();
        message_data.email = "invalid-email".to_string();
        
        let result = service.submit_message(message_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::Validation(_) | ApiError::ValidationErrors(_) => {},
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_submit_message_invalid_content() {
        let service = create_test_service().await;
        let mut message_data = create_test_message();
        message_data.message = "123".to_string(); // Invalid content
        
        let result = service.submit_message(message_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::BadRequest(_) => {},
            _ => panic!("Expected bad request error"),
        }
    }

    #[tokio::test]
    async fn test_get_all_messages() {
        let service = create_test_service().await;
        let message_data = create_test_message();
        
        service.submit_message(message_data).await.unwrap();
        
        let messages = service.get_all_messages().await.unwrap();
        assert!(messages.len() >= 1);
    }

    #[tokio::test]
    async fn test_get_message_by_id() {
        let service = create_test_service().await;
        let message_data = create_test_message();
        
        let submitted = service.submit_message(message_data).await.unwrap();
        let retrieved = service.get_message_by_id(submitted.id).await.unwrap();
        
        assert_eq!(retrieved.id, submitted.id);
        assert_eq!(retrieved.email, submitted.email);
    }

    #[tokio::test]
    async fn test_search_messages() {
        let service = create_test_service().await;
        let message_data = create_test_message();
        
        service.submit_message(message_data).await.unwrap();
        
        let results = service.search_messages("John").await.unwrap();
        assert!(results.len() >= 1);
        assert!(results.iter().any(|m| m.name.contains("John")));
    }

    #[tokio::test]
    async fn test_get_message_stats() {
        let service = create_test_service().await;
        let message_data = create_test_message();
        
        service.submit_message(message_data).await.unwrap();
        
        let stats = service.get_message_stats().await.unwrap();
        assert!(stats.total_messages >= 1);
        assert!(stats.messages_this_week >= 1);
    }
}