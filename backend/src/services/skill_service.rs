use sqlx::SqlitePool;
use validator::Validate;
use tracing::{info, warn, error};
use crate::{
    database::SkillRepository,
    models::{Skill, CreateSkill, UpdateSkill},
    models::skill::SkillCategory,
    error::{ApiError, ApiResult},
};

/// Service for skill-related business logic
pub struct SkillService {
    repository: SkillRepository,
}

impl SkillService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repository: SkillRepository::new(pool),
        }
    }

    /// Get all skills
    pub async fn get_all_skills(&self) -> ApiResult<Vec<Skill>> {
        info!("Fetching all skills");
        
        match self.repository.get_all().await {
            Ok(skills) => {
                info!("Successfully fetched {} skills", skills.len());
                Ok(skills)
            }
            Err(e) => {
                error!("Failed to fetch skills: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get skill by ID
    pub async fn get_skill_by_id(&self, id: i32) -> ApiResult<Skill> {
        info!("Fetching skill with ID: {}", id);
        
        match self.repository.get_by_id(id).await {
            Ok(Some(skill)) => {
                info!("Successfully fetched skill: {}", skill.name);
                Ok(skill)
            }
            Ok(None) => {
                warn!("Skill with ID {} not found", id);
                Err(ApiError::NotFound(format!("Skill with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to fetch skill {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get skills by category
    pub async fn get_skills_by_category(&self, category: &str) -> ApiResult<Vec<Skill>> {
        info!("Fetching skills for category: {}", category);
        
        // Validate category
        if SkillCategory::from_str(category).is_none() {
            return Err(ApiError::BadRequest(format!("Invalid skill category: {}", category)));
        }
        
        match self.repository.get_by_category(category).await {
            Ok(skills) => {
                info!("Successfully fetched {} skills for category '{}'", skills.len(), category);
                Ok(skills)
            }
            Err(e) => {
                error!("Failed to fetch skills for category '{}': {}", category, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get skills by minimum level
    pub async fn get_skills_by_min_level(&self, min_level: i32) -> ApiResult<Vec<Skill>> {
        info!("Fetching skills with minimum level: {}", min_level);
        
        if min_level < 1 || min_level > 5 {
            return Err(ApiError::BadRequest("Skill level must be between 1 and 5".to_string()));
        }
        
        match self.repository.get_by_min_level(min_level).await {
            Ok(skills) => {
                info!("Successfully fetched {} skills with level >= {}", skills.len(), min_level);
                Ok(skills)
            }
            Err(e) => {
                error!("Failed to fetch skills with min level {}: {}", min_level, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Create a new skill
    pub async fn create_skill(&self, mut skill_data: CreateSkill) -> ApiResult<Skill> {
        info!("Creating new skill: {}", skill_data.name);
        
        // Validate input data
        if let Err(validation_errors) = skill_data.validate() {
            warn!("Validation failed for skill creation: {:?}", validation_errors);
            return Err(ApiError::from_validation_errors(validation_errors));
        }

        // Validate category
        if SkillCategory::from_str(&skill_data.category).is_none() {
            return Err(ApiError::BadRequest(format!("Invalid skill category: {}", skill_data.category)));
        }

        // Sanitize and normalize data
        skill_data.name = skill_data.name.trim().to_string();
        skill_data.category = skill_data.category.trim().to_string();

        // Check for duplicate skill names (case-insensitive)
        if let Ok(existing_skills) = self.repository.get_all().await {
            if existing_skills.iter().any(|s| s.name.to_lowercase() == skill_data.name.to_lowercase()) {
                return Err(ApiError::Conflict("A skill with this name already exists".to_string()));
            }
        }

        match self.repository.create(&skill_data).await {
            Ok(skill) => {
                info!("Successfully created skill: {} (ID: {})", skill.name, skill.id);
                Ok(skill)
            }
            Err(e) => {
                error!("Failed to create skill '{}': {}", skill_data.name, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Update an existing skill
    pub async fn update_skill(&self, id: i32, mut skill_data: UpdateSkill) -> ApiResult<Skill> {
        info!("Updating skill with ID: {}", id);
        
        // Validate input data
        if let Err(validation_errors) = skill_data.validate() {
            warn!("Validation failed for skill update: {:?}", validation_errors);
            return Err(ApiError::from_validation_errors(validation_errors));
        }

        // Check if there are any updates to apply
        if !self.has_updates(&skill_data) {
            return Err(ApiError::BadRequest("No updates provided".to_string()));
        }

        // Validate category if provided
        if let Some(ref category) = skill_data.category {
            if SkillCategory::from_str(category).is_none() {
                return Err(ApiError::BadRequest(format!("Invalid skill category: {}", category)));
            }
        }

        // Sanitize data if provided
        if let Some(ref mut name) = skill_data.name {
            *name = name.trim().to_string();
        }
        if let Some(ref mut category) = skill_data.category {
            *category = category.trim().to_string();
        }

        match self.repository.update(id, &skill_data).await {
            Ok(Some(skill)) => {
                info!("Successfully updated skill: {} (ID: {})", skill.name, skill.id);
                Ok(skill)
            }
            Ok(None) => {
                warn!("Skill with ID {} not found for update", id);
                Err(ApiError::NotFound(format!("Skill with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to update skill {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Delete a skill
    pub async fn delete_skill(&self, id: i32) -> ApiResult<()> {
        info!("Deleting skill with ID: {}", id);
        
        match self.repository.delete(id).await {
            Ok(true) => {
                info!("Successfully deleted skill with ID: {}", id);
                Ok(())
            }
            Ok(false) => {
                warn!("Skill with ID {} not found for deletion", id);
                Err(ApiError::NotFound(format!("Skill with ID {} not found", id)))
            }
            Err(e) => {
                error!("Failed to delete skill {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get all skill categories
    pub async fn get_categories(&self) -> ApiResult<Vec<String>> {
        info!("Fetching skill categories");
        
        match self.repository.get_categories().await {
            Ok(categories) => {
                info!("Successfully fetched {} categories", categories.len());
                Ok(categories)
            }
            Err(e) => {
                error!("Failed to fetch categories: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get available skill categories (predefined list)
    pub fn get_available_categories(&self) -> Vec<&'static str> {
        SkillCategory::all()
    }

    /// Check if update data contains any changes
    fn has_updates(&self, update_data: &UpdateSkill) -> bool {
        update_data.name.is_some()
            || update_data.category.is_some()
            || update_data.level.is_some()
            || update_data.years_experience.is_some()
            || update_data.description.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_service() -> SkillService {
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

        SkillService::new(pool)
    }

    fn create_test_skill() -> CreateSkill {
        CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: Some("Systems programming language".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_skill() {
        let service = create_test_service().await;
        let skill_data = create_test_skill();
        
        let created = service.create_skill(skill_data).await.unwrap();
        assert_eq!(created.name, "Rust");
        assert_eq!(created.category, "Backend");
        assert_eq!(created.level, 4);

        let retrieved = service.get_skill_by_id(created.id).await.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, created.name);
    }

    #[tokio::test]
    async fn test_create_skill_invalid_category() {
        let service = create_test_service().await;
        let mut skill_data = create_test_skill();
        skill_data.category = "InvalidCategory".to_string();
        
        let result = service.create_skill(skill_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::BadRequest(_) => {},
            _ => panic!("Expected bad request error"),
        }
    }

    #[tokio::test]
    async fn test_create_duplicate_skill() {
        let service = create_test_service().await;
        let skill_data = create_test_skill();
        
        // Create first skill
        service.create_skill(skill_data.clone()).await.unwrap();
        
        // Try to create duplicate
        let result = service.create_skill(skill_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::Conflict(_) => {},
            _ => panic!("Expected conflict error"),
        }
    }

    #[tokio::test]
    async fn test_get_skills_by_category() {
        let service = create_test_service().await;
        let skill_data = create_test_skill();
        
        service.create_skill(skill_data).await.unwrap();
        
        let skills = service.get_skills_by_category("Backend").await.unwrap();
        assert!(skills.len() >= 1);
        assert!(skills.iter().all(|s| s.category == "Backend"));
    }

    #[tokio::test]
    async fn test_get_skills_by_min_level() {
        let service = create_test_service().await;
        let skill_data = create_test_skill();
        
        service.create_skill(skill_data).await.unwrap();
        
        let skills = service.get_skills_by_min_level(3).await.unwrap();
        assert!(skills.len() >= 1);
        assert!(skills.iter().all(|s| s.level >= 3));
    }

    #[tokio::test]
    async fn test_get_available_categories() {
        let service = create_test_service().await;
        let categories = service.get_available_categories();
        
        assert!(categories.contains(&"Frontend"));
        assert!(categories.contains(&"Backend"));
        assert!(categories.contains(&"Database"));
    }
}