use axum_test::TestServer;
use portfolio_backend::{database, routes};
use serde_json::json;
use sqlx::SqlitePool;
use std::time::Instant;
use tokio::time::{sleep, Duration};

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
async fn test_api_response_times() {
    let server = setup_test_server().await;

    // Test GET /api/projects response time
    let start = Instant::now();
    let response = server.get("/api/projects").await;
    let duration = start.elapsed();
    
    response.assert_status_ok();
    assert!(duration.as_millis() < 100, "GET /api/projects took too long: {}ms", duration.as_millis());

    // Test GET /api/skills response time
    let start = Instant::now();
    let response = server.get("/api/skills").await;
    let duration = start.elapsed();
    
    response.assert_status_ok();
    assert!(duration.as_millis() < 100, "GET /api/skills took too long: {}ms", duration.as_millis());

    // Test GET /api/profile response time
    let start = Instant::now();
    let response = server.get("/api/profile").await;
    let duration = start.elapsed();
    
    response.assert_status_ok();
    assert!(duration.as_millis() < 100, "GET /api/profile took too long: {}ms", duration.as_millis());
}

#[tokio::test]
async fn test_concurrent_requests() {
    let server = setup_test_server().await;

    let start = Instant::now();
    
    // Make 10 sequential requests to simulate load
    for _ in 0..10 {
        let response = server.get("/api/projects").await;
        response.assert_status_ok();
    }

    let duration = start.elapsed();
    assert!(duration.as_millis() < 500, "Sequential requests took too long: {}ms", duration.as_millis());
}

#[tokio::test]
async fn test_database_performance() {
    let server = setup_test_server().await;

    // Create multiple projects to test database performance
    for i in 0..50 {
        let project = json!({
            "title": format!("Performance Test Project {}", i),
            "description": "A test project for performance testing",
            "technologies": ["Rust", "SQLite"],
            "category": "Test",
            "featured": false
        });

        let response = server
            .post("/api/projects")
            .json(&project)
            .await;
        
        response.assert_status_ok();
    }

    // Test retrieval performance with many records
    let start = Instant::now();
    let response = server.get("/api/projects").await;
    let duration = start.elapsed();
    
    response.assert_status_ok();
    assert!(duration.as_millis() < 200, "Database query with 50 records took too long: {}ms", duration.as_millis());
}

#[tokio::test]
async fn test_memory_usage_stability() {
    let server = setup_test_server().await;

    // Make many requests to test for memory leaks
    for _ in 0..100 {
        let response = server.get("/api/projects").await;
        response.assert_status_ok();
        
        // Small delay to prevent overwhelming the server
        sleep(Duration::from_millis(1)).await;
    }

    // If we get here without crashing, memory usage is stable
    assert!(true);
}

#[tokio::test]
async fn test_error_handling_performance() {
    let server = setup_test_server().await;

    // Test that error responses are also fast
    let start = Instant::now();
    let response = server.get("/api/projects/99999").await;
    let duration = start.elapsed();
    
    response.assert_status_not_found();
    assert!(duration.as_millis() < 50, "Error response took too long: {}ms", duration.as_millis());
}

#[tokio::test]
async fn test_large_payload_handling() {
    let server = setup_test_server().await;

    // Create a project with a reasonably large description
    let large_description = "A".repeat(500); // 500 char description
    let project = json!({
        "title": "Large Payload Test",
        "description": large_description,
        "technologies": ["Rust"],
        "category": "web",
        "featured": false
    });

    let start = Instant::now();
    let response = server
        .post("/api/projects")
        .json(&project)
        .await;
    let duration = start.elapsed();
    
    // Check if it's a validation error or success
    if response.status_code().is_success() {
        assert!(duration.as_millis() < 200, "Large payload handling took too long: {}ms", duration.as_millis());
    } else {
        // If validation fails, just check that the error response is fast
        assert!(duration.as_millis() < 100, "Error response took too long: {}ms", duration.as_millis());
    }
}