use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::{
    error::ApiError,
    models::{CreateSkill, Skill, UpdateSkill},
    routes::projects::ApiResponse,
    services::SkillService,
};

/// Query parameters for skill listing
#[derive(Debug, Deserialize)]
pub struct SkillQuery {
    pub category: Option<String>,
    pub min_level: Option<i32>,
}

/// Create skill routes
pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(get_skills).post(create_skill))
        .route("/:id", get(get_skill_by_id).put(update_skill).delete(delete_skill))
        .route("/categories", get(get_categories))
        .with_state(pool)
}

/// GET /api/skills - Get all skills with optional filtering
async fn get_skills(
    State(pool): State<SqlitePool>,
    Query(params): Query<SkillQuery>,
) -> Result<Json<ApiResponse<Vec<Skill>>>, ApiError> {
    let service = SkillService::new(pool);

    // Handle category filtering
    if let Some(category) = params.category {
        let skills = service.get_skills_by_category(&category).await?;
        return Ok(Json(ApiResponse::success(skills)));
    }

    // Handle minimum level filtering
    if let Some(min_level) = params.min_level {
        let skills = service.get_skills_by_min_level(min_level).await?;
        return Ok(Json(ApiResponse::success(skills)));
    }

    // Default: get all skills
    let skills = service.get_all_skills().await?;
    Ok(Json(ApiResponse::success(skills)))
}

/// GET /api/skills/:id - Get a specific skill by ID
async fn get_skill_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Skill>>, ApiError> {
    let service = SkillService::new(pool);
    let skill = service.get_skill_by_id(id).await?;
    Ok(Json(ApiResponse::success(skill)))
}

/// POST /api/skills - Create a new skill
async fn create_skill(
    State(pool): State<SqlitePool>,
    Json(skill_data): Json<CreateSkill>,
) -> Result<Json<ApiResponse<Skill>>, ApiError> {
    let service = SkillService::new(pool);
    let skill = service.create_skill(skill_data).await?;
    Ok(Json(ApiResponse::success_with_message(
        skill,
        "Skill created successfully".to_string(),
    )))
}

/// PUT /api/skills/:id - Update an existing skill
async fn update_skill(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
    Json(skill_data): Json<UpdateSkill>,
) -> Result<Json<ApiResponse<Skill>>, ApiError> {
    let service = SkillService::new(pool);
    let skill = service.update_skill(id, skill_data).await?;
    Ok(Json(ApiResponse::success_with_message(
        skill,
        "Skill updated successfully".to_string(),
    )))
}

/// DELETE /api/skills/:id - Delete a skill
async fn delete_skill(
    State(pool): State<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Value>>, ApiError> {
    let service = SkillService::new(pool);
    service.delete_skill(id).await?;
    Ok(Json(ApiResponse::success_with_message(
        json!({}),
        "Skill deleted successfully".to_string(),
    )))
}

/// GET /api/skills/categories - Get all available skill categories
async fn get_categories(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<SkillCategoriesResponse>>, ApiError> {
    let service = SkillService::new(pool);
    
    // Get both used categories and available categories
    let used_categories = service.get_categories().await?;
    let available_categories = service.get_available_categories();
    
    let response = SkillCategoriesResponse {
        used: used_categories,
        available: available_categories.into_iter().map(|s| s.to_string()).collect(),
    };
    
    Ok(Json(ApiResponse::success(response)))
}

/// Response for skill categories endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillCategoriesResponse {
    pub used: Vec<String>,
    pub available: Vec<String>,
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
            CREATE TABLE IF NOT EXISTS skills (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                level INTEGER NOT NULL CHECK (level >= 1 AND level <= 5),
                years_experience INTEGER,
                description TEXT,
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

    fn create_test_skill_json() -> serde_json::Value {
        json!({
            "name": "Rust",
            "category": "Backend",
            "level": 4,
            "years_experience": 3,
            "description": "Systems programming language"
        })
    }

    #[tokio::test]
    async fn test_create_skill() {
        let (app, _pool) = create_test_app().await;
        
        let request = Request::builder()
            .method(Method::POST)
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(create_test_skill_json().to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Skill> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let skill = response_json.data.unwrap();
        assert_eq!(skill.name, "Rust");
        assert_eq!(skill.category, "Backend");
        assert_eq!(skill.level, 4);
    }

    #[tokio::test]
    async fn test_get_skills() {
        let (app, pool) = create_test_app().await;
        
        // First create a skill
        let service = SkillService::new(pool);
        let skill_data = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: Some("Systems programming language".to_string()),
        };
        service.create_skill(skill_data).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<Skill>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let skills = response_json.data.unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].name, "Rust");
    }

    #[tokio::test]
    async fn test_get_skills_by_category() {
        let (app, pool) = create_test_app().await;
        
        // Create skills with different categories
        let service = SkillService::new(pool);
        let backend_skill = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: None,
        };
        let frontend_skill = CreateSkill {
            name: "JavaScript".to_string(),
            category: "Frontend".to_string(),
            level: 5,
            years_experience: Some(5),
            description: None,
        };
        
        service.create_skill(backend_skill).await.unwrap();
        service.create_skill(frontend_skill).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/?category=Backend")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<Skill>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let skills = response_json.data.unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].category, "Backend");
        assert_eq!(skills[0].name, "Rust");
    }

    #[tokio::test]
    async fn test_get_skills_by_min_level() {
        let (app, pool) = create_test_app().await;
        
        // Create skills with different levels
        let service = SkillService::new(pool);
        let high_level_skill = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 5,
            years_experience: Some(5),
            description: None,
        };
        let low_level_skill = CreateSkill {
            name: "Python".to_string(),
            category: "Backend".to_string(),
            level: 2,
            years_experience: Some(1),
            description: None,
        };
        
        service.create_skill(high_level_skill).await.unwrap();
        service.create_skill(low_level_skill).await.unwrap();

        let request = Request::builder()
            .method(Method::GET)
            .uri("/?min_level=4")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Vec<Skill>> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let skills = response_json.data.unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].name, "Rust");
        assert_eq!(skills[0].level, 5);
    }

    #[tokio::test]
    async fn test_get_categories() {
        let (app, _pool) = create_test_app().await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/categories")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<SkillCategoriesResponse> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let categories = response_json.data.unwrap();
        assert!(categories.available.contains(&"Frontend".to_string()));
        assert!(categories.available.contains(&"Backend".to_string()));
    }

    #[tokio::test]
    async fn test_update_skill() {
        let (app, pool) = create_test_app().await;
        
        // First create a skill
        let service = SkillService::new(pool);
        let skill_data = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: Some("Systems programming language".to_string()),
        };
        let created_skill = service.create_skill(skill_data).await.unwrap();

        let update_data = json!({
            "name": "Advanced Rust",
            "level": 5
        });

        let request = Request::builder()
            .method(Method::PUT)
            .uri(&format!("/{}", created_skill.id))
            .header("content-type", "application/json")
            .body(Body::from(update_data.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<Skill> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.data.is_some());
        
        let skill = response_json.data.unwrap();
        assert_eq!(skill.name, "Advanced Rust");
        assert_eq!(skill.level, 5);
    }

    #[tokio::test]
    async fn test_delete_skill() {
        let (app, pool) = create_test_app().await;
        
        // First create a skill
        let service = SkillService::new(pool);
        let skill_data = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: Some("Systems programming language".to_string()),
        };
        let created_skill = service.create_skill(skill_data).await.unwrap();

        let request = Request::builder()
            .method(Method::DELETE)
            .uri(&format!("/{}", created_skill.id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response_json: ApiResponse<serde_json::Value> = serde_json::from_slice(&body).unwrap();
        
        assert!(response_json.success);
        assert!(response_json.message.is_some());
    }
}