use axum::{
    extract::State,
    response::Json,
    routing::{get, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    error::ApiError,
    models::{Profile, UpdateProfile},
    routes::projects::ApiResponse,
    services::{ProfileService, profile_service::ProfileSummary},
};

/// Create profile routes
pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(get_profile).put(update_profile))
        .route("/summary", get(get_profile_summary))
        .route("/exists", get(check_profile_exists))
        .with_state(pool)
}

/// GET /api/profile - Get the profile
async fn get_profile(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<Profile>>, ApiError> {
    let service = ProfileService::new(pool);
    let profile = service.get_profile().await?;
    Ok(Json(ApiResponse::success(profile)))
}

/// PUT /api/profile - Update the profile
async fn update_profile(
    State(pool): State<SqlitePool>,
    Json(profile_data): Json<UpdateProfile>,
) -> Result<Json<ApiResponse<Profile>>, ApiError> {
    let service = ProfileService::new(pool);
    let profile = service.update_profile(profile_data).await?;
    Ok(Json(ApiResponse::success_with_message(
        profile,
        "Profile updated successfully".to_string(),
    )))
}

/// GET /api/profile/summary - Get profile summary (public info only)
async fn get_profile_summary(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<ProfileSummary>>, ApiError> {
    let service = ProfileService::new(pool);
    let summary = service.get_profile_summary().await?;
    Ok(Json(ApiResponse::success(summary)))
}

/// GET /api/profile/exists - Check if profile exists
async fn check_profile_exists(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<ProfileExistsResponse>>, ApiError> {
    let service = ProfileService::new(pool);
    let exists = service.profile_exists().await?;
    Ok(Json(ApiResponse::success(ProfileExistsResponse { exists })))
}

/// Response for profile exists endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileExistsResponse {
    pub exists: bool,
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
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        // Insert test profile
        sqlx::query(
            "INSERT INTO profile (id, name, title, bio, email, location) VALUES (1, 'Test User', 'Test Developer', 'Test bio', 'test@example.com', 'Test Location')"
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = create_routes(pool.clone());
        (app, pool)
    }

    #[tokio::test]
    async fn test_get_profile() {
        let (app, _pool) = create_test_app().await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Profile> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let profile = response_json.data.unwrap();
        assert_eq!(profile.name, "Test User");
        assert_eq!(profile.title, "Test Developer");
        assert_eq!(profile.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_update_profile() {
        let (app, _pool) = create_test_app().await;

        let update_data = json!({
            "name": "Updated User",
            "title": "Senior Developer",
            "bio": "Updated bio"
        });

        let request = Request::builder()
            .method(Method::PUT)
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(update_data.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Profile> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let profile = response_json.data.unwrap();
        assert_eq!(profile.name, "Updated User");
        assert_eq!(profile.title, "Senior Developer");
        assert_eq!(profile.bio, "Updated bio");
    }

    #[tokio::test]
    async fn test_get_profile_summary() {
        let (app, _pool) = create_test_app().await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/summary")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<ProfileSummary> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let summary = response_json.data.unwrap();
        assert_eq!(summary.name, "Test User");
        assert_eq!(summary.title, "Test Developer");
        assert_eq!(summary.location, "Test Location");
    }

    #[tokio::test]
    async fn test_check_profile_exists() {
        let (app, _pool) = create_test_app().await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/exists")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<ProfileExistsResponse> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let exists_response = response_json.data.unwrap();
        assert!(exists_response.exists);
    }

    #[tokio::test]
    async fn test_update_profile_validation_error() {
        let (app, _pool) = create_test_app().await;

        let update_data = json!({
            "email": "invalid-email"
        });

        let request = Request::builder()
            .method(Method::PUT)
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(update_data.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}