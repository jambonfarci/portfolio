use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use validator::Validate;

/// Project model representing a portfolio project
#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub technologies: String, // JSON array as string
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    pub image_url: Option<String>,
    pub category: String,
    pub featured: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Project model for API responses with parsed technologies
#[derive(Debug, Clone, Serialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub long_description: Option<String>,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    pub image_url: Option<String>,
    pub category: String,
    pub featured: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        let technologies = project.get_technologies().unwrap_or_default();
        Self {
            id: project.id,
            title: project.title,
            description: project.description,
            long_description: project.long_description,
            technologies,
            github_url: project.github_url,
            demo_url: project.demo_url,
            image_url: project.image_url,
            category: project.category,
            featured: project.featured,
            created_at: project.created_at,
        }
    }
}

/// Create project request model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProject {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: String,
    
    #[validate(length(min = 1, max = 500, message = "Description must be between 1 and 500 characters"))]
    pub description: String,
    
    #[validate(length(max = 2000, message = "Long description must be less than 2000 characters"))]
    pub long_description: Option<String>,
    
    #[validate(length(min = 1, message = "At least one technology must be specified"))]
    pub technologies: Vec<String>,
    
    #[validate(url(message = "GitHub URL must be a valid URL"))]
    pub github_url: Option<String>,
    
    #[validate(url(message = "Demo URL must be a valid URL"))]
    pub demo_url: Option<String>,
    
    #[validate(url(message = "Image URL must be a valid URL"))]
    pub image_url: Option<String>,
    
    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: String,
    
    pub featured: Option<bool>,
}

/// Update project request model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProject {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 500, message = "Description must be between 1 and 500 characters"))]
    pub description: Option<String>,
    
    #[validate(length(max = 2000, message = "Long description must be less than 2000 characters"))]
    pub long_description: Option<String>,
    
    pub technologies: Option<Vec<String>>,
    
    #[validate(url(message = "GitHub URL must be a valid URL"))]
    pub github_url: Option<String>,
    
    #[validate(url(message = "Demo URL must be a valid URL"))]
    pub demo_url: Option<String>,
    
    #[validate(url(message = "Image URL must be a valid URL"))]
    pub image_url: Option<String>,
    
    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: Option<String>,
    
    pub featured: Option<bool>,
}

impl Project {
    /// Parse technologies from JSON string
    pub fn get_technologies(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_str(&self.technologies)
    }
}

impl CreateProject {
    /// Convert technologies to JSON string
    pub fn technologies_as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.technologies)
    }
}

impl UpdateProject {
    /// Convert technologies to JSON string if present
    pub fn technologies_as_json(&self) -> Result<Option<String>, serde_json::Error> {
        match &self.technologies {
            Some(techs) => Ok(Some(serde_json::to_string(techs)?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_project_validation_success() {
        let project = CreateProject {
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

        assert!(project.validate().is_ok());
    }

    #[test]
    fn test_create_project_validation_empty_title() {
        let project = CreateProject {
            title: "".to_string(),
            description: "A test project description".to_string(),
            long_description: None,
            technologies: vec!["Rust".to_string()],
            github_url: None,
            demo_url: None,
            image_url: None,
            category: "web".to_string(),
            featured: None,
        };

        assert!(project.validate().is_err());
    }

    #[test]
    fn test_create_project_validation_invalid_url() {
        let project = CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: None,
            technologies: vec!["Rust".to_string()],
            github_url: Some("not-a-url".to_string()),
            demo_url: None,
            image_url: None,
            category: "web".to_string(),
            featured: None,
        };

        assert!(project.validate().is_err());
    }

    #[test]
    fn test_create_project_technologies_json() {
        let project = CreateProject {
            title: "Test Project".to_string(),
            description: "A test project description".to_string(),
            long_description: None,
            technologies: vec!["Rust".to_string(), "SQLite".to_string()],
            github_url: None,
            demo_url: None,
            image_url: None,
            category: "web".to_string(),
            featured: None,
        };

        let json = project.technologies_as_json().unwrap();
        assert_eq!(json, r#"["Rust","SQLite"]"#);
    }

    #[test]
    fn test_project_get_technologies() {
        let project = Project {
            id: 1,
            title: "Test Project".to_string(),
            description: "A test project".to_string(),
            long_description: None,
            technologies: r#"["Rust","SQLite"]"#.to_string(),
            github_url: None,
            demo_url: None,
            image_url: None,
            category: "web".to_string(),
            featured: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let techs = project.get_technologies().unwrap();
        assert_eq!(techs, vec!["Rust", "SQLite"]);
    }
}