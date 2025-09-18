use sqlx::SqlitePool;
use validator::Validate;
use tracing::{info, warn, error};
use crate::{
    database::ProjectRepository,
    models::{Project, CreateProject, UpdateProject},
    error::{ApiError, ApiResult},
};

/// Service for project-related business logic
pub struct ProjectService {
    repository: ProjectRepository,
}

impl ProjectService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repository: ProjectRepository::new(pool),
        }
    }

    /// Get all projects
    pub async fn get_all_projects(&self) -> ApiResult<Vec<Project>> {
        info!("Fetching all projects");
        
        match self.repository.get_all().await {
            Ok(projects) => {
                info!("Successfully fetched {} projects", projects.len());
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to fetch projects: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get project by ID
    pub async fn get_project_by_id(&self, id: i32) -> ApiResult<Project> {
        info!("Fetching project with ID: {}", id);
        
        match self.repository.get_by_id(id).await {
            Ok(Some(project)) => {
                info!("Successfully fetched project: {}", project.title);
                Ok(project)
            }
            Ok(None) => {
                warn!("Project with ID {} not found", id);
                Err(ApiError::NotFound(format!("Project with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to fetch project {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get projects by category
    pub async fn get_projects_by_category(&self, category: &str) -> ApiResult<Vec<Project>> {
        info!("Fetching projects for category: {}", category);
        
        match self.repository.get_by_category(category).await {
            Ok(projects) => {
                info!("Successfully fetched {} projects for category '{}'", projects.len(), category);
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to fetch projects for category '{}': {}", category, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get featured projects
    pub async fn get_featured_projects(&self) -> ApiResult<Vec<Project>> {
        info!("Fetching featured projects");
        
        match self.repository.get_featured().await {
            Ok(projects) => {
                info!("Successfully fetched {} featured projects", projects.len());
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to fetch featured projects: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Create a new project
    pub async fn create_project(&self, mut project_data: CreateProject) -> ApiResult<Project> {
        info!("Creating new project: {}", project_data.title);
        
        // Validate input data
        if let Err(validation_errors) = project_data.validate() {
            warn!("Validation failed for project creation: {:?}", validation_errors);
            return Err(ApiError::from_validation_errors(validation_errors));
        }

        // Additional business logic validation
        if project_data.technologies.is_empty() {
            return Err(ApiError::Validation("At least one technology must be specified".to_string()));
        }

        // Sanitize and normalize data
        project_data.title = project_data.title.trim().to_string();
        project_data.description = project_data.description.trim().to_string();
        project_data.category = project_data.category.trim().to_lowercase();

        // Check for duplicate titles (business rule)
        if let Ok(existing_projects) = self.repository.search(&project_data.title).await {
            if existing_projects.iter().any(|p| p.title.to_lowercase() == project_data.title.to_lowercase()) {
                return Err(ApiError::Conflict("A project with this title already exists".to_string()));
            }
        }

        match self.repository.create(&project_data).await {
            Ok(project) => {
                info!("Successfully created project: {} (ID: {})", project.title, project.id);
                Ok(project)
            }
            Err(e) => {
                error!("Failed to create project '{}': {}", project_data.title, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Update an existing project
    pub async fn update_project(&self, id: i32, mut project_data: UpdateProject) -> ApiResult<Project> {
        info!("Updating project with ID: {}", id);
        
        // Validate input data
        if let Err(validation_errors) = project_data.validate() {
            warn!("Validation failed for project update: {:?}", validation_errors);
            return Err(ApiError::from_validation_errors(validation_errors));
        }

        // Check if there are any updates to apply
        if !self.has_updates(&project_data) {
            return Err(ApiError::BadRequest("No updates provided".to_string()));
        }

        // Sanitize data if provided
        if let Some(ref mut title) = project_data.title {
            *title = title.trim().to_string();
        }
        if let Some(ref mut description) = project_data.description {
            *description = description.trim().to_string();
        }
        if let Some(ref mut category) = project_data.category {
            *category = category.trim().to_lowercase();
        }

        match self.repository.update(id, &project_data).await {
            Ok(Some(project)) => {
                info!("Successfully updated project: {} (ID: {})", project.title, project.id);
                Ok(project)
            }
            Ok(None) => {
                warn!("Project with ID {} not found for update", id);
                Err(ApiError::NotFound(format!("Project with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to update project {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Delete a project
    pub async fn delete_project(&self, id: i32) -> ApiResult<()> {
        info!("Deleting project with ID: {}", id);
        
        match self.repository.delete(id).await {
            Ok(true) => {
                info!("Successfully deleted project with ID: {}", id);
                Ok(())
            }
            Ok(false) => {
                warn!("Project with ID {} not found for deletion", id);
                Err(ApiError::NotFound(format!("Project with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to delete project {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Search projects
    pub async fn search_projects(&self, query: &str) -> ApiResult<Vec<Project>> {
        info!("Searching projects with query: '{}'", query);
        
        if query.trim().is_empty() {
            return Err(ApiError::BadRequest("Search query cannot be empty".to_string()));
        }

        match self.repository.search(query).await {
            Ok(projects) => {
                info!("Found {} projects matching query '{}'", projects.len(), query);
                Ok(projects)
            }
            Err(e) => {
                error!("Failed to search projects with query '{}': {}", query, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get projects with pagination
    pub async fn get_projects_paginated(&self, page: u32, page_size: u32) -> ApiResult<(Vec<Project>, u64)> {
        info!("Fetching projects page {} with size {}", page, page_size);
        
        if page_size == 0 || page_size > 100 {
            return Err(ApiError::BadRequest("Page size must be between 1 and 100".to_string()));
        }

        let offset = (page.saturating_sub(1) * page_size) as i64;
        let limit = page_size as i64;

        match tokio::try_join!(
            self.repository.get_paginated(limit, offset),
            self.repository.count()
        ) {
            Ok((projects, total_count)) => {
                info!("Successfully fetched {} projects (page {}, total: {})", projects.len(), page, total_count);
                Ok((projects, total_count as u64))
            }
            Err(e) => {
                error!("Failed to fetch paginated projects: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Check if update data contains any changes
    fn has_updates(&self, update_data: &UpdateProject) -> bool {
        update_data.title.is_some()
            || update_data.description.is_some()
            || update_data.long_description.is_some()
            || update_data.technologies.is_some()
            || update_data.github_url.is_some()
            || update_data.demo_url.is_some()
            || update_data.image_url.is_some()
            || update_data.category.is_some()
            || update_data.featured.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_service() -> ProjectService {
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

        ProjectService::new(pool)
    }

    fn create_test_project() -> CreateProject {
        CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: Some("A longer description".to_string()),
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
        let service = create_test_service().await;
        let project_data = create_test_project();
        
        let created = service.create_project(project_data).await.unwrap();
        assert_eq!(created.title, "Test Project");
        assert_eq!(created.category, "web");
        assert!(created.featured);

        let retrieved = service.get_project_by_id(created.id).await.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.title, created.title);
    }

    #[tokio::test]
    async fn test_create_project_validation_error() {
        let service = create_test_service().await;
        let mut project_data = create_test_project();
        project_data.title = "".to_string(); // Invalid title
        
        let result = service.create_project(project_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::Validation(_) => {},
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_duplicate_project() {
        let service = create_test_service().await;
        let project_data = create_test_project();
        
        // Create first project
        service.create_project(project_data.clone()).await.unwrap();
        
        // Try to create duplicate
        let result = service.create_project(project_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::Conflict(_) => {},
            _ => panic!("Expected conflict error"),
        }
    }

    #[tokio::test]
    async fn test_update_project() {
        let service = create_test_service().await;
        let project_data = create_test_project();
        
        let created = service.create_project(project_data).await.unwrap();
        
        let update_data = UpdateProject {
            title: Some("Updated Title".to_string()),
            description: Some("Updated description".to_string()),
            featured: Some(false),
            ..Default::default()
        };
        
        let updated = service.update_project(created.id, update_data).await.unwrap();
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.description, "Updated description");
        assert!(!updated.featured);
    }

    #[tokio::test]
    async fn test_delete_project() {
        let service = create_test_service().await;
        let project_data = create_test_project();
        
        let created = service.create_project(project_data).await.unwrap();
        
        service.delete_project(created.id).await.unwrap();
        
        let result = service.get_project_by_id(created.id).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::NotFound(_) => {},
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_search_projects() {
        let service = create_test_service().await;
        let project_data = create_test_project();
        
        service.create_project(project_data).await.unwrap();
        
        let results = service.search_projects("Test").await.unwrap();
        assert!(results.len() >= 1);
        assert!(results.iter().any(|p| p.title.contains("Test")));
    }

    #[tokio::test]
    async fn test_get_projects_paginated() {
        let service = create_test_service().await;
        
        // Create multiple projects
        for i in 0..5 {
            let mut project_data = create_test_project();
            project_data.title = format!("Test Project {}", i);
            service.create_project(project_data).await.unwrap();
        }
        
        let (projects, total) = service.get_projects_paginated(1, 3).await.unwrap();
        assert!(projects.len() <= 3);
        assert_eq!(total, 5);
    }
}