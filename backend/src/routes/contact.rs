use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::{
    error::ApiError,
    models::{ContactMessage, CreateContactMessage},
    routes::projects::{ApiResponse, PaginationInfo},
    services::{ContactService, contact_service::MessageStats},
};

/// Query parameters for contact message listing (admin only)
#[derive(Debug, Deserialize)]
pub struct ContactQuery {
    pub search: Option<String>,
    pub days: Option<u32>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Create contact routes
pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", post(submit_contact_message))
        .route("/messages", get(get_contact_messages))
        .route("/messages/:id", get(get_contact_message_by_id).delete(delete_contact_message))
        .route("/stats", get(get_message_stats))
        .route("/cleanup", post(cleanup_old_messages))
        .with_state(pool)
}

/// POST /api/contact - Submit a contact message
async fn submit_contact_message(
    State(pool): State<SqlitePool>,
    Json(message_data): Json<CreateContactMessage>,
) -> Result<Json<ApiResponse<ContactSubmissionResponse>>, ApiError> {
    let service = ContactService::new(pool);
    let message = service.submit_message(message_data).await?;
    
    let response = ContactSubmissionResponse {
        id: message.id,
        submitted_at: message.created_at,
        message: "Thank you for your message! I'll get back to you soon.".to_string(),
    };
    
    Ok(Json(ApiResponse::success_with_message(
        response,
        "Message submitted successfully".to_string(),
    )))
}

/// GET /api/contact/messages - Get all contact messages (admin only)
async fn get_contact_messages(
    State(pool): State<SqlitePool>,
    Query(params): Query<ContactQuery>,
) -> Result<Json<ApiResponse<Vec<ContactMessage>>>, ApiError> {
    let service = ContactService::new(pool);

    // Handle pagination
    if let (Some(page), Some(page_size)) = (params.page, params.page_size) {
        let (messages, total_count) = service.get_messages_paginated(page, page_size).await?;
        let total_pages = (total_count as f64 / page_size as f64).ceil() as u64;
        
        let pagination = PaginationInfo {
            page,
            page_size,
            total_count,
            total_pages,
        };

        return Ok(Json(ApiResponse::success_with_pagination(messages, pagination)));
    }

    // Handle search
    if let Some(search_query) = params.search {
        let messages = service.search_messages(&search_query).await?;
        return Ok(Json(ApiResponse::success(messages)));
    }

    // Handle recent messages
    if let Some(days) = params.days {
        let messages = service.get_recent_messages(days).await?;
        return Ok(Json(ApiResponse::success(messages)));
    }

    // Default: get all messages
    let messages = service.get_all_messages().await?;
    Ok(Json(ApiResponse::success(messages)))
}

/// GET /api/contact/messages/:id - Get a specific contact message by ID (admin only)
async fn get_contact_message_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<ContactMessage>>, ApiError> {
    let service = ContactService::new(pool);
    let message = service.get_message_by_id(id).await?;
    Ok(Json(ApiResponse::success(message)))
}

/// DELETE /api/contact/messages/:id - Delete a contact message (admin only)
async fn delete_contact_message(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Value>>, ApiError> {
    let service = ContactService::new(pool);
    service.delete_message(id).await?;
    Ok(Json(ApiResponse::success_with_message(
        json!({}),
        "Message deleted successfully".to_string(),
    )))
}

/// GET /api/contact/stats - Get message statistics (admin only)
async fn get_message_stats(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<MessageStats>>, ApiError> {
    let service = ContactService::new(pool);
    let stats = service.get_message_stats().await?;
    Ok(Json(ApiResponse::success(stats)))
}

/// POST /api/contact/cleanup - Clean up old messages (admin only)
async fn cleanup_old_messages(
    State(pool): State<SqlitePool>,
    Json(cleanup_request): Json<CleanupRequest>,
) -> Result<Json<ApiResponse<CleanupResponse>>, ApiError> {
    let service = ContactService::new(pool);
    let deleted_count = service.cleanup_old_messages(cleanup_request.days).await?;
    
    let response = CleanupResponse {
        deleted_count,
        message: format!("Successfully deleted {} old messages", deleted_count),
    };
    
    Ok(Json(ApiResponse::success(response)))
}

/// Response for contact form submission
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactSubmissionResponse {
    pub id: i32,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub message: String,
}

/// Request for cleanup operation
#[derive(Debug, Deserialize)]
pub struct CleanupRequest {
    pub days: u32,
}

/// Response for cleanup operation
#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupResponse {
    pub deleted_count: u64,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use serde_json::json;
    use sqlx::SqlitePool;
    use tower::ServiceExt;

    async fn create_test_app() -> (Router, SqlitePool) {
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

        let app = create_routes(pool.clone());
        (app, pool)
    }

    fn create_test_contact_json() -> serde_json::Value {
        json!({
            "name": "John Doe",
            "email": "john.doe@example.com",
            "subject": "Inquiry about services",
            "message": "Hello, I'm interested in your web development services. Could you please provide more information?"
        })
    }

    #[tokio::test]
    async fn test_submit_contact_message() {
        let (app, _pool) = create_test_app().await;
        
        let request = Request::builder()
            .method(Method::POST)
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(create_test_contact_json().to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<ContactSubmissionResponse> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let submission = response_json.data.unwrap();
        assert!(submission.id > 0);
        assert!(submission.message.contains("Thank you"));
    }

    #[tokio::test]
    async fn test_submit_contact_message_validation_error() {
        let (app, _pool) = create_test_app().await;
        
        let invalid_message = json!({
            "name": "",
            "email": "invalid-email",
            "subject": "Test",
            "message": "Test message"
        });
        
        let request = Request::builder()
            .method(Method::POST)
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(invalid_message.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_contact_messages() {
        let (app, pool) = create_test_app().await;
        
        // First submit a message
        let service = ContactService::new(pool);
        let message_data = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message with sufficient content for testing purposes.".to_string(),
        };
        service.submit_message(message_data).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/messages")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<ContactMessage>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let messages = response_json.data.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].name, "John Doe");
    }

    #[tokio::test]
    async fn test_get_contact_message_by_id() {
        let (app, pool) = create_test_app().await;
        
        // First submit a message
        let service = ContactService::new(pool);
        let message_data = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message with sufficient content for testing purposes.".to_string(),
        };
        let submitted_message = service.submit_message(message_data).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri(&format!("/messages/{}", submitted_message.id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<ContactMessage> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let message = response_json.data.unwrap();
        assert_eq!(message.id, submitted_message.id);
        assert_eq!(message.name, "John Doe");
    }

    #[tokio::test]
    async fn test_get_message_stats() {
        let (app, pool) = create_test_app().await;
        
        // First submit a message
        let service = ContactService::new(pool);
        let message_data = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message with sufficient content for testing purposes.".to_string(),
        };
        service.submit_message(message_data).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/stats")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<MessageStats> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let stats = response_json.data.unwrap();
        assert!(stats.total_messages >= 1);
    }

    #[tokio::test]
    async fn test_delete_contact_message() {
        let (app, pool) = create_test_app().await;
        
        // First submit a message
        let service = ContactService::new(pool);
        let message_data = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message with sufficient content for testing purposes.".to_string(),
        };
        let submitted_message = service.submit_message(message_data).await.unwrap();

        let request = Request::builder()
            .method(Method::DELETE)
            .uri(&format!("/messages/{}", submitted_message.id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.message.is_some());
    }

    #[tokio::test]
    async fn test_cleanup_old_messages() {
        let (app, _pool) = create_test_app().await;

        let cleanup_request = json!({
            "days": 365
        });

        let request = Request::builder()
            .method(Method::POST)
            .uri("/cleanup")
            .header("content-type", "application/json")
            .body(Body::from(cleanup_request.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<CleanupResponse> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let cleanup_response = response_json.data.unwrap();
        assert!(cleanup_response.message.contains("deleted"));
    }

    #[tokio::test]
    async fn test_get_messages_with_pagination() {
        let (app, pool) = create_test_app().await;
        
        // Create multiple messages
        let service = ContactService::new(pool);
        for i in 0..5 {
            let message_data = CreateContactMessage {
                name: format!("User {}", i),
                email: format!("user{}@example.com", i),
                subject: "Test Subject".to_string(),
                message: "This is a test message with sufficient content for testing purposes.".to_string(),
            };
            service.submit_message(message_data).await.unwrap();
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri("/messages?page=1&page_size=3")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<ContactMessage>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        assert!(response_json.pagination.is_some());
        
        let messages = response_json.data.unwrap();
        let pagination = response_json.pagination.unwrap();
        
        assert!(messages.len() <= 3);
        assert_eq!(pagination.total_count, 5);
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.page_size, 3);
    }
}