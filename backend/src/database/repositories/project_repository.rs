use sqlx::SqlitePool;
use chrono::Utc;
use crate::models::{Project, CreateProject, UpdateProject};

/// Repository for project database operations
pub struct ProjectRepository {
    pool: SqlitePool,
}

impl ProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get all projects
    pub async fn get_all(&self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "SELECT id, title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at FROM projects ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get project by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "SELECT id, title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at FROM projects WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get projects by category
    pub async fn get_by_category(&self, category: &str) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "SELECT id, title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at FROM projects WHERE category = ? ORDER BY created_at DESC"
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await
    }

    /// Get featured projects
    pub async fn get_featured(&self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "SELECT id, title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at FROM projects WHERE featured = true ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Create a new project
    pub async fn create(&self, project: &CreateProject) -> Result<Project, sqlx::Error> {
        let technologies_json = project.technologies_as_json()
            .map_err(|e| sqlx::Error::decode(e))?;
        
        let now = Utc::now();
        
        let result = sqlx::query(
            r#"
            INSERT INTO projects (title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&project.title)
        .bind(&project.description)
        .bind(&project.long_description)
        .bind(&technologies_json)
        .bind(&project.github_url)
        .bind(&project.demo_url)
        .bind(&project.image_url)
        .bind(&project.category)
        .bind(project.featured.unwrap_or(false))
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid() as i32;
        
        // Fetch the created project
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    /// Update an existing project
    pub async fn update(&self, id: i32, project: &UpdateProject) -> Result<Option<Project>, sqlx::Error> {
        // Check if project exists first
        if self.get_by_id(id).await?.is_none() {
            return Ok(None);
        }

        let now = Utc::now();
        let technologies_json = if project.technologies.is_some() {
            Some(project.technologies_as_json()
                .map_err(|e| sqlx::Error::decode(e))?)
        } else {
            None
        };

        // Use a comprehensive update query with COALESCE to keep existing values
        sqlx::query(
            r#"
            UPDATE projects SET 
                title = COALESCE(?, title),
                description = COALESCE(?, description),
                long_description = COALESCE(?, long_description),
                technologies = COALESCE(?, technologies),
                github_url = COALESCE(?, github_url),
                demo_url = COALESCE(?, demo_url),
                image_url = COALESCE(?, image_url),
                category = COALESCE(?, category),
                featured = COALESCE(?, featured),
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&project.title)
        .bind(&project.description)
        .bind(&project.long_description)
        .bind(technologies_json.flatten())
        .bind(&project.github_url)
        .bind(&project.demo_url)
        .bind(&project.image_url)
        .bind(&project.category)
        .bind(project.featured)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_by_id(id).await
    }

    /// Delete a project
    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get projects with pagination
    pub async fn get_paginated(&self, limit: i64, offset: i64) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>(
            "SELECT id, title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at FROM projects ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// Count total projects
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.pool)
            .await
    }

    /// Search projects by title or description
    pub async fn search(&self, query: &str) -> Result<Vec<Project>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);
        
        sqlx::query_as::<_, Project>(
            "SELECT id, title, description, long_description, technologies, github_url, demo_url, image_url, category, featured, created_at, updated_at FROM projects WHERE title LIKE ? OR description LIKE ? ORDER BY created_at DESC"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn create_test_repository() -> ProjectRepository {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables manually for testing
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

        ProjectRepository::new(pool)
    }

    fn create_test_project() -> CreateProject {
        CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: Some("A longer description for testing".to_string()),
            technologies: vec!["Rust".to_string(), "SQLite".to_string()],
            github_url: Some("https://github.com/test/project".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            category: "web".to_string(),
            featured: Some(true),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_project() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        let created = repo.create(&project_data).await.unwrap();
        assert_eq!(created.title, project_data.title);
        assert_eq!(created.description, project_data.description);
        assert_eq!(created.featured, true);

        let retrieved = repo.get_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.title, created.title);
    }

    #[tokio::test]
    async fn test_get_all_projects() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        repo.create(&project_data).await.unwrap();
        
        let projects = repo.get_all().await.unwrap();
        assert!(projects.len() >= 1);
    }

    #[tokio::test]
    async fn test_get_by_category() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        repo.create(&project_data).await.unwrap();
        
        let projects = repo.get_by_category("web").await.unwrap();
        assert!(projects.len() >= 1);
        assert!(projects.iter().all(|p| p.category == "web"));
    }

    #[tokio::test]
    async fn test_get_featured() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        repo.create(&project_data).await.unwrap();
        
        let featured = repo.get_featured().await.unwrap();
        assert!(featured.len() >= 1);
        assert!(featured.iter().all(|p| p.featured));
    }

    #[tokio::test]
    async fn test_update_project() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        let created = repo.create(&project_data).await.unwrap();
        
        let update_data = UpdateProject {
            title: Some("Updated Title".to_string()),
            description: Some("Updated description".to_string()),
            featured: Some(false),
            ..Default::default()
        };
        
        let updated = repo.update(created.id, &update_data).await.unwrap().unwrap();
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.description, "Updated description");
        assert_eq!(updated.featured, false);
    }

    #[tokio::test]
    async fn test_delete_project() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        let created = repo.create(&project_data).await.unwrap();
        
        let deleted = repo.delete(created.id).await.unwrap();
        assert!(deleted);
        
        let retrieved = repo.get_by_id(created.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_search_projects() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        repo.create(&project_data).await.unwrap();
        
        let results = repo.search("Test").await.unwrap();
        assert!(results.len() >= 1);
        assert!(results.iter().any(|p| p.title.contains("Test")));
    }

    #[tokio::test]
    async fn test_count_projects() {
        let repo = create_test_repository().await;
        let project_data = create_test_project();
        
        let initial_count = repo.count().await.unwrap();
        repo.create(&project_data).await.unwrap();
        let new_count = repo.count().await.unwrap();
        
        assert_eq!(new_count, initial_count + 1);
    }
}

impl Default for UpdateProject {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            long_description: None,
            technologies: None,
            github_url: None,
            demo_url: None,
            image_url: None,
            category: None,
            featured: None,
        }
    }
}