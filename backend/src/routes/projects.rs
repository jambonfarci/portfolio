use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::{
    error::{ApiError, ApiResult},
    models::{CreateProject, Project, ProjectResponse, UpdateProject},
    services::ProjectService,
};

/// Query parameters for project listing
#[derive(Debug, Deserialize)]
pub struct ProjectQuery {
    pub category: Option<String>,
    pub featured: Option<bool>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Response wrapper for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub pagination: Option<PaginationInfo>,
}

/// Pagination information
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            pagination: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
            pagination: None,
        }
    }

    pub fn success_with_pagination(data: T, pagination: PaginationInfo) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            pagination: Some(pagination),
        }
    }
}

/// Create project routes
pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(get_projects).post(create_project))
        .route("/:id", get(get_project_by_id).put(update_project).delete(delete_project))
        .with_state(pool)
}

/// GET /api/projects - Get all projects with optional filtering and pagination
async fn get_projects(
    State(pool): State<SqlitePool>,
    Query(params): Query<ProjectQuery>,
) -> Result<Json<ApiResponse<Vec<ProjectResponse>>>, ApiError> {
    let service = ProjectService::new(pool);

    // Handle pagination
    if let (Some(page), Some(page_size)) = (params.page, params.page_size) {
        let (projects, total_count) = service.get_projects_paginated(page, page_size).await?;
        let total_pages = (total_count as f64 / page_size as f64).ceil() as u64;
        
        let pagination = PaginationInfo {
            page,
            page_size,
            total_count,
            total_pages,
        };

        let project_responses: Vec<ProjectResponse> = projects.into_iter().map(ProjectResponse::from).collect();
        return Ok(Json(ApiResponse::success_with_pagination(project_responses, pagination)));
    }

    // Handle search
    if let Some(search_query) = params.search {
        let projects = service.search_projects(&search_query).await?;
        let project_responses: Vec<ProjectResponse> = projects.into_iter().map(ProjectResponse::from).collect();
        return Ok(Json(ApiResponse::success(project_responses)));
    }

    // Handle category filtering
    if let Some(category) = params.category {
        let projects = service.get_projects_by_category(&category).await?;
        let project_responses: Vec<ProjectResponse> = projects.into_iter().map(ProjectResponse::from).collect();
        return Ok(Json(ApiResponse::success(project_responses)));
    }

    // Handle featured filtering
    if let Some(true) = params.featured {
        let projects = service.get_featured_projects().await?;
        let project_responses: Vec<ProjectResponse> = projects.into_iter().map(ProjectResponse::from).collect();
        return Ok(Json(ApiResponse::success(project_responses)));
    }

    // Default: get all projects
    let projects = service.get_all_projects().await?;
    let project_responses: Vec<ProjectResponse> = projects.into_iter().map(ProjectResponse::from).collect();
    Ok(Json(ApiResponse::success(project_responses)))
}

/// GET /api/projects/:id - Get a specific project by ID
async fn get_project_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<ProjectResponse>>, ApiError> {
    let service = ProjectService::new(pool);
    let project = service.get_project_by_id(id).await?;
    let project_response = ProjectResponse::from(project);
    Ok(Json(ApiResponse::success(project_response)))
}

/// POST /api/projects - Create a new project
async fn create_project(
    State(pool): State<SqlitePool>,
    Json(project_data): Json<CreateProject>,
) -> Result<Json<ApiResponse<ProjectResponse>>, ApiError> {
    let service = ProjectService::new(pool);
    let project = service.create_project(project_data).await?;
    let project_response = ProjectResponse::from(project);
    Ok(Json(ApiResponse::success_with_message(
        project_response,
        "Project created successfully".to_string(),
    )))
}

/// PUT /api/projects/:id - Update an existing project
async fn update_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(project_data): Json<UpdateProject>,
) -> Result<Json<ApiResponse<ProjectResponse>>, ApiError> {
    let service = ProjectService::new(pool);
    let project = service.update_project(id, project_data).await?;
    let project_response = ProjectResponse::from(project);
    Ok(Json(ApiResponse::success_with_message(
        project_response,
        "Project updated successfully".to_string(),
    )))
}

/// DELETE /api/projects/:id - Delete a project
async fn delete_project(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Value>>, ApiError> {
    let service = ProjectService::new(pool);
    service.delete_project(id).await?;
    Ok(Json(ApiResponse::success_with_message(
        json!({}),
        "Project deleted successfully".to_string(),
    )))
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
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = create_routes(pool.clone());
        (app, pool)
    }

    fn create_test_project_json() -> serde_json::Value {
        json!({
            "title": "Test Project",
            "description": "A test project description",
            "long_description": "A longer description",
            "technologies": ["Rust", "SQLite"],
            "github_url": "https://github.com/test/project",
            "demo_url": "https://demo.example.com",
            "image_url": "https://example.com/image.jpg",
            "category": "web",
            "featured": true
        })
    }

    #[tokio::test]
    async fn test_create_project() {
        let (app, _pool) = create_test_app().await;
        
        let request = Request::builder()
            .method(Method::POST)
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(create_test_project_json().to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Project> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let project = response_json.data.unwrap();
        assert_eq!(project.title, "Test Project");
        assert_eq!(project.category, "web");
        assert!(project.featured);
    }

    #[tokio::test]
    async fn test_get_projects() {
        let (app, pool) = create_test_app().await;
        
        // First create a project
        let service = ProjectService::new(pool);
        let project_data = CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: Some("A longer description".to_string()),
            technologies: vec!["Rust".to_string(), "SQLite".to_string()],
            github_url: Some("https://github.com/test/project".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            category: "web".to_string(),
            featured: Some(true),
        };
        service.create_project(project_data).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<Project>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let projects = response_json.data.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].title, "Test Project");
    }

    #[tokio::test]
    async fn test_get_project_by_id() {
        let (app, pool) = create_test_app().await;
        
        // First create a project
        let service = ProjectService::new(pool);
        let project_data = CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: Some("A longer description".to_string()),
            technologies: vec!["Rust".to_string(), "SQLite".to_string()],
            github_url: Some("https://github.com/test/project".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            category: "web".to_string(),
            featured: Some(true),
        };
        let created_project = service.create_project(project_data).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri(&format!("/{}", created_project.id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Project> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let project = response_json.data.unwrap();
        assert_eq!(project.id, created_project.id);
        assert_eq!(project.title, "Test Project");
    }

    #[tokio::test]
    async fn test_update_project() {
        let (app, pool) = create_test_app().await;
        
        // First create a project
        let service = ProjectService::new(pool);
        let project_data = CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: Some("A longer description".to_string()),
            technologies: vec!["Rust".to_string(), "SQLite".to_string()],
            github_url: Some("https://github.com/test/project".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            category: "web".to_string(),
            featured: Some(true),
        };
        let created_project = service.create_project(project_data).await.unwrap();

        let update_data = json!({
            "title": "Updated Project",
            "description": "Updated description"
        });

        let request = Request::builder()
            .method(Method::PUT)
            .uri(&format!("/{}", created_project.id))
            .header("content-type", "application/json")
            .body(Body::from(update_data.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Project> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let project = response_json.data.unwrap();
        assert_eq!(project.title, "Updated Project");
        assert_eq!(project.description, "Updated description");
    }

    #[tokio::test]
    async fn test_delete_project() {
        let (app, pool) = create_test_app().await;
        
        // First create a project
        let service = ProjectService::new(pool);
        let project_data = CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: Some("A longer description".to_string()),
            technologies: vec!["Rust".to_string(), "SQLite".to_string()],
            github_url: Some("https://github.com/test/project".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            category: "web".to_string(),
            featured: Some(true),
        };
        let created_project = service.create_project(project_data).await.unwrap();

        let request = Request::builder()
            .method(Method::DELETE)
            .uri(&format!("/{}", created_project.id))
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
    async fn test_get_projects_with_pagination() {
        let (app, pool) = create_test_app().await;
        
        // Create multiple projects
        let service = ProjectService::new(pool);
        for i in 0..5 {
            let project_data = CreateProject {
                title: format!("Test Project {}", i),
                description: "A test project description".to_string(),
                long_description: Some("A longer description".to_string()),
                technologies: vec!["Rust".to_string(), "SQLite".to_string()],
                github_url: Some("https://github.com/test/project".to_string()),
                demo_url: Some("https://demo.example.com".to_string()),
                image_url: Some("https://example.com/image.jpg".to_string()),
                category: "web".to_string(),
                featured: Some(false),
            };
            service.create_project(project_data).await.unwrap();
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri("/?page=1&page_size=3")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<Project>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        assert!(response_json.pagination.is_some());
        
        let projects = response_json.data.unwrap();
        let pagination = response_json.pagination.unwrap();
        
        assert!(projects.len() <= 3);
        assert_eq!(pagination.total_count, 5);
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.page_size, 3);
    }

    #[tokio::test]
    async fn test_get_projects_by_category() {
        let (app, pool) = create_test_app().await;
        
        // Create projects with different categories
        let service = ProjectService::new(pool);
        let web_project = CreateProject {
            title: "Web Project".to_string(),
            description: "A web project".to_string(),
            long_description: None,
            technologies: vec!["JavaScript".to_string()],
            github_url: None,
            demo_url: None,
            image_url: None,
            category: "web".to_string(),
            featured: Some(false),
        };
        let mobile_project = CreateProject {
            title: "Mobile Project".to_string(),
            description: "A mobile project".to_string(),
            long_description: None,
            technologies: vec!["React Native".to_string()],
            github_url: None,
            demo_url: None,
            image_url: None,
            category: "mobile".to_string(),
            featured: Some(false),
        };
        
        service.create_project(web_project).await.unwrap();
        service.create_project(mobile_project).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/?category=web")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<Project>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let projects = response_json.data.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].category, "web");
        assert_eq!(projects[0].title, "Web Project");
    }
}