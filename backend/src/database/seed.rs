use sqlx::SqlitePool;
use tracing::{info, error};
use serde_json::json;

/// Seed data error types
#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Seed the database with initial data
pub async fn seed_database(pool: &SqlitePool) -> Result<(), SeedError> {
    info!("Starting database seeding...");

    seed_profile(pool).await?;
    seed_skills(pool).await?;
    seed_projects(pool).await?;

    info!("Database seeding completed successfully");
    Ok(())
}

/// Seed profile data
async fn seed_profile(pool: &SqlitePool) -> Result<(), SeedError> {
    // Check if profile already exists
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM profile")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        info!("Profile data already exists, skipping seed");
        return Ok(());
    }

    info!("Seeding profile data...");
    
    sqlx::query(
        r#"
        INSERT INTO profile (
            id, name, title, bio, email, location,
            linkedin_url, github_url, twitter_url
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(1)
    .bind("John Doe")
    .bind("Full Stack Developer")
    .bind("Passionate developer with expertise in modern web technologies including Rust, TypeScript, and cloud infrastructure. I love building scalable applications and exploring new technologies.")
    .bind("john.doe@example.com")
    .bind("Paris, France")
    .bind("https://linkedin.com/in/johndoe")
    .bind("https://github.com/johndoe")
    .bind("https://twitter.com/johndoe")
    .execute(pool)
    .await?;

    info!("Profile data seeded successfully");
    Ok(())
}

/// Seed skills data
async fn seed_skills(pool: &SqlitePool) -> Result<(), SeedError> {
    // Check if skills already exist
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM skills")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        info!("Skills data already exists, skipping seed");
        return Ok(());
    }

    info!("Seeding skills data...");

    let skills = vec![
        ("Rust", "Backend", 4, Some(3), "Systems programming and web backends"),
        ("TypeScript", "Frontend", 5, Some(5), "Modern JavaScript development"),
        ("React", "Frontend", 4, Some(4), "Component-based UI development"),
        ("Svelte", "Frontend", 3, Some(2), "Lightweight reactive framework"),
        ("Node.js", "Backend", 4, Some(4), "Server-side JavaScript"),
        ("PostgreSQL", "Database", 4, Some(4), "Relational database management"),
        ("SQLite", "Database", 3, Some(2), "Embedded database solutions"),
        ("Docker", "DevOps", 4, Some(3), "Containerization and deployment"),
        ("Git", "Tools", 5, Some(6), "Version control and collaboration"),
        ("Linux", "Tools", 4, Some(5), "System administration and scripting"),
    ];

    for (name, category, level, years, description) in skills {
        sqlx::query(
            "INSERT INTO skills (name, category, level, years_experience, description) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(category)
        .bind(level)
        .bind(years)
        .bind(description)
        .execute(pool)
        .await?;
    }

    info!("Skills data seeded successfully");
    Ok(())
}

/// Seed projects data
async fn seed_projects(pool: &SqlitePool) -> Result<(), SeedError> {
    // Check if projects already exist
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        info!("Projects data already exists, skipping seed");
        return Ok(());
    }

    info!("Seeding projects data...");

    let projects = vec![
        (
            "Portfolio Website",
            "Modern portfolio website built with Rust and Svelte",
            "A full-stack portfolio application showcasing modern web development practices. Built with Rust backend using Axum framework and Svelte frontend with TypeScript. Features include project management, skills showcase, and contact form.",
            json!(["Rust", "Svelte", "TypeScript", "SQLite", "Docker"]).to_string(),
            Some("https://github.com/johndoe/portfolio"),
            Some("https://johndoe.dev"),
            "web",
            true,
        ),
        (
            "Task Management API",
            "RESTful API for task management with authentication",
            "A robust REST API built with Rust and Axum for managing tasks and projects. Features JWT authentication, role-based access control, and comprehensive error handling.",
            json!(["Rust", "Axum", "PostgreSQL", "JWT", "Docker"]).to_string(),
            Some("https://github.com/johndoe/task-api"),
            None,
            "backend",
            true,
        ),
        (
            "Weather Dashboard",
            "Real-time weather dashboard with interactive maps",
            "Interactive weather dashboard built with React and TypeScript. Integrates with multiple weather APIs to provide real-time weather data, forecasts, and interactive maps.",
            json!(["React", "TypeScript", "Node.js", "Express", "MongoDB"]).to_string(),
            Some("https://github.com/johndoe/weather-dashboard"),
            Some("https://weather.johndoe.dev"),
            "frontend",
            false,
        ),
    ];

    for (title, description, long_description, technologies, github_url, demo_url, category, featured) in projects {
        sqlx::query(
            r#"
            INSERT INTO projects (
                title, description, long_description, technologies,
                github_url, demo_url, category, featured
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(title)
        .bind(description)
        .bind(long_description)
        .bind(technologies)
        .bind(github_url)
        .bind(demo_url)
        .bind(category)
        .bind(featured)
        .execute(pool)
        .await?;
    }

    info!("Projects data seeded successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn create_test_pool_with_schema() -> SqlitePool {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables
        let schema = include_str!("../../migrations/001_initial_schema.sql");
        for statement in schema.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("INSERT") {
                sqlx::query(statement).execute(&pool).await.unwrap();
            }
        }

        pool
    }

    #[tokio::test]
    async fn test_seed_profile() {
        let pool = create_test_pool_with_schema().await;
        
        let result = seed_profile(&pool).await;
        assert!(result.is_ok());

        // Verify profile was inserted
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM profile")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(count, 1);

        // Test idempotency - should not insert again
        let result = seed_profile(&pool).await;
        assert!(result.is_ok());

        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM profile")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_seed_skills() {
        let pool = create_test_pool_with_schema().await;
        
        let result = seed_skills(&pool).await;
        assert!(result.is_ok());

        // Verify skills were inserted
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM skills")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_seed_projects() {
        let pool = create_test_pool_with_schema().await;
        
        let result = seed_projects(&pool).await;
        assert!(result.is_ok());

        // Verify projects were inserted
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_full_seed() {
        let pool = create_test_pool_with_schema().await;
        
        let result = seed_database(&pool).await;
        assert!(result.is_ok());

        // Verify all tables have data
        let profile_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM profile")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let skills_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM skills")
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let projects_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(profile_count, 1);
        assert!(skills_count > 0);
        assert!(projects_count > 0);
    }
}