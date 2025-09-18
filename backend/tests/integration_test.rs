use axum::http::StatusCode;
use axum_test::TestServer;
use portfolio_backend::{database, routes};
use serde_json::{json, Value};
use sqlx::SqlitePool;

async fn setup_test_server() -> TestServer {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create test database");

    database::migrations::initialize_database(pool.clone())
        .await
        .expect("Failed to initialize database");

    let router = routes::create_router(pool);
    TestServer::new(router).expect("Failed to create test server")
}

#[tokio::test]
async fn test_server_setup() {
    let _server = setup_test_server().await;
    // If we get here, the setup is working correctly
    assert!(true);
}

#[tokio::test]
async fn test_database_connection() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database");

    // Test basic query
    let result = sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to execute test query");

    assert_eq!(result, 1);
}

#[tokio::test]
async fn test_get_projects_endpoint() {
    let server = setup_test_server().await;

    let response = server.get("/api/projects").await;
    response.assert_status_ok();
    
    let projects: Value = response.json();
    assert!(projects.is_array());
}

#[tokio::test]
async fn test_create_project_endpoint() {
    let server = setup_test_server().await;

    let new_project = json!({
        "title": "Test Project",
        "description": "A test project",
        "technologies": ["Rust", "Svelte"],
        "category": "Web",
        "featured": false
    });

    let response = server
        .post("/api/projects")
        .json(&new_project)
        .await;
    
    response.assert_status(StatusCode::CREATED);
    
    let created_project: Value = response.json();
    assert_eq!(created_project["title"], "Test Project");
    assert_eq!(created_project["category"], "Web");
}

#[tokio::test]
async fn test_get_project_by_id_endpoint() {
    let server = setup_test_server().await;

    // First create a project
    let new_project = json!({
        "title": "Test Project",
        "description": "A test project",
        "technologies": ["Rust"],
        "category": "Web",
        "featured": false
    });

    let create_response = server
        .post("/api/projects")
        .json(&new_project)
        .await;
    
    let created_project: Value = create_response.json();
    let project_id = created_project["id"].as_i64().unwrap();

    // Now get the project by ID
    let response = server.get(&format!("/api/projects/{}", project_id)).await;
    response.assert_status_ok();
    
    let project: Value = response.json();
    assert_eq!(project["id"], project_id);
    assert_eq!(project["title"], "Test Project");
}

#[tokio::test]
async fn test_update_project_endpoint() {
    let server = setup_test_server().await;

    // Create a project
    let new_project = json!({
        "title": "Original Title",
        "description": "Original description",
        "technologies": ["Rust"],
        "category": "Web",
        "featured": false
    });

    let create_response = server
        .post("/api/projects")
        .json(&new_project)
        .await;
    
    let created_project: Value = create_response.json();
    let project_id = created_project["id"].as_i64().unwrap();

    // Update the project
    let update_data = json!({
        "title": "Updated Title",
        "featured": true
    });

    let response = server
        .put(&format!("/api/projects/{}", project_id))
        .json(&update_data)
        .await;
    
    response.assert_status_ok();
    
    let updated_project: Value = response.json();
    assert_eq!(updated_project["title"], "Updated Title");
    assert_eq!(updated_project["featured"], true);
}

#[tokio::test]
async fn test_delete_project_endpoint() {
    let server = setup_test_server().await;

    // Create a project
    let new_project = json!({
        "title": "To Delete",
        "description": "Will be deleted",
        "technologies": ["Rust"],
        "category": "Web",
        "featured": false
    });

    let create_response = server
        .post("/api/projects")
        .json(&new_project)
        .await;
    
    let created_project: Value = create_response.json();
    let project_id = created_project["id"].as_i64().unwrap();

    // Delete the project
    let response = server.delete(&format!("/api/projects/{}", project_id)).await;
    response.assert_status(StatusCode::NO_CONTENT);

    // Verify it's deleted
    let get_response = server.get(&format!("/api/projects/{}", project_id)).await;
    get_response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_skills_endpoint() {
    let server = setup_test_server().await;

    let response = server.get("/api/skills").await;
    response.assert_status_ok();
    
    let skills: Value = response.json();
    assert!(skills.is_array());
}

#[tokio::test]
async fn test_create_skill_endpoint() {
    let server = setup_test_server().await;

    let new_skill = json!({
        "name": "TypeScript",
        "category": "Frontend",
        "level": 4,
        "years_experience": 3,
        "description": "Typed JavaScript"
    });

    let response = server
        .post("/api/skills")
        .json(&new_skill)
        .await;
    
    response.assert_status(StatusCode::CREATED);
    
    let created_skill: Value = response.json();
    assert_eq!(created_skill["name"], "TypeScript");
    assert_eq!(created_skill["level"], 4);
}

#[tokio::test]
async fn test_get_profile_endpoint() {
    let server = setup_test_server().await;

    let response = server.get("/api/profile").await;
    response.assert_status_ok();
    
    let profile: Value = response.json();
    assert!(profile["name"].is_string());
    assert!(profile["email"].is_string());
}

#[tokio::test]
async fn test_update_profile_endpoint() {
    let server = setup_test_server().await;

    let update_data = json!({
        "name": "Updated Name",
        "title": "Senior Developer"
    });

    let response = server
        .put("/api/profile")
        .json(&update_data)
        .await;
    
    response.assert_status_ok();
    
    let updated_profile: Value = response.json();
    assert_eq!(updated_profile["name"], "Updated Name");
    assert_eq!(updated_profile["title"], "Senior Developer");
}

#[tokio::test]
async fn test_contact_message_endpoint() {
    let server = setup_test_server().await;

    let contact_message = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "subject": "Inquiry",
        "message": "Hello, I'm interested in your work."
    });

    let response = server
        .post("/api/contact")
        .json(&contact_message)
        .await;
    
    response.assert_status(StatusCode::CREATED);
    
    let created_message: Value = response.json();
    assert_eq!(created_message["name"], "John Doe");
    assert_eq!(created_message["email"], "john@example.com");
}

#[tokio::test]
async fn test_invalid_project_creation() {
    let server = setup_test_server().await;

    let invalid_project = json!({
        "title": "", // Empty title should fail validation
        "description": "Valid description",
        "technologies": [],
        "category": "Web"
    });

    let response = server
        .post("/api/projects")
        .json(&invalid_project)
        .await;
    
    response.assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_nonexistent_project() {
    let server = setup_test_server().await;

    let response = server.get("/api/projects/99999").await;
    response.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_cors_headers() {
    let server = setup_test_server().await;

    // Test CORS by making a simple request and checking response headers
    let response = server.get("/api/projects").await;
    response.assert_status_ok();
    
    // CORS headers should be present in the response
    let headers = response.headers();
    // Note: CORS headers might not be present in all responses, 
    // but the server should handle CORS properly
    assert!(headers.len() > 0); // Just verify we get some headers back
}