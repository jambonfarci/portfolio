use sqlx::SqlitePool;
use chrono::Utc;
use crate::models::{Skill, CreateSkill, UpdateSkill};

/// Repository for skill database operations
pub struct SkillRepository {
    pool: SqlitePool,
}

impl SkillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get all skills
    pub async fn get_all(&self) -> Result<Vec<Skill>, sqlx::Error> {
        sqlx::query_as::<_, Skill>(
            "SELECT id, name, category, level, years_experience, description, created_at FROM skills ORDER BY category, name"
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Get skill by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Skill>, sqlx::Error> {
        sqlx::query_as::<_, Skill>(
            "SELECT id, name, category, level, years_experience, description, created_at FROM skills WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    /// Get skills by category
    pub async fn get_by_category(&self, category: &str) -> Result<Vec<Skill>, sqlx::Error> {
        sqlx::query_as::<_, Skill>(
            "SELECT id, name, category, level, years_experience, description, created_at FROM skills WHERE category = ? ORDER BY level DESC, name"
        )
        .bind(category)
        .fetch_all(&self.pool)
        .await
    }

    /// Get skills by minimum level
    pub async fn get_by_min_level(&self, min_level: i32) -> Result<Vec<Skill>, sqlx::Error> {
        sqlx::query_as::<_, Skill>(
            "SELECT id, name, category, level, years_experience, description, created_at FROM skills WHERE level >= ? ORDER BY level DESC, name"
        )
        .bind(min_level)
        .fetch_all(&self.pool)
        .await
    }

    /// Create a new skill
    pub async fn create(&self, skill: &CreateSkill) -> Result<Skill, sqlx::Error> {
        let now = Utc::now();
        
        let result = sqlx::query(
            "INSERT INTO skills (name, category, level, years_experience, description, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&skill.name)
        .bind(&skill.category)
        .bind(skill.level)
        .bind(skill.years_experience)
        .bind(&skill.description)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid() as i32;
        
        // Fetch the created skill
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    /// Update an existing skill
    pub async fn update(&self, id: i32, skill: &UpdateSkill) -> Result<Option<Skill>, sqlx::Error> {
        // Check if skill exists first
        if self.get_by_id(id).await?.is_none() {
            return Ok(None);
        }

        // Use COALESCE to keep existing values for fields that are None
        sqlx::query(
            r#"
            UPDATE skills SET 
                name = COALESCE(?, name),
                category = COALESCE(?, category),
                level = COALESCE(?, level),
                years_experience = COALESCE(?, years_experience),
                description = COALESCE(?, description)
            WHERE id = ?
            "#
        )
        .bind(&skill.name)
        .bind(&skill.category)
        .bind(skill.level)
        .bind(skill.years_experience)
        .bind(&skill.description)
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id).await
    }

    /// Delete a skill
    pub async fn delete(&self, id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM skills WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get unique categories
    pub async fn get_categories(&self) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar("SELECT DISTINCT category FROM skills ORDER BY category")
            .fetch_all(&self.pool)
            .await
    }

    /// Count skills by category
    pub async fn count_by_category(&self, category: &str) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM skills WHERE category = ?")
            .bind(category)
            .fetch_one(&self.pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn create_test_repository() -> SkillRepository {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables manually for testing
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

        SkillRepository::new(pool)
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
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        let created = repo.create(&skill_data).await.unwrap();
        assert_eq!(created.name, skill_data.name);
        assert_eq!(created.category, skill_data.category);
        assert_eq!(created.level, skill_data.level);

        let retrieved = repo.get_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, created.name);
    }

    #[tokio::test]
    async fn test_get_all_skills() {
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        repo.create(&skill_data).await.unwrap();
        
        let skills = repo.get_all().await.unwrap();
        assert!(skills.len() >= 1);
    }

    #[tokio::test]
    async fn test_get_by_category() {
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        repo.create(&skill_data).await.unwrap();
        
        let skills = repo.get_by_category("Backend").await.unwrap();
        assert!(skills.len() >= 1);
        assert!(skills.iter().all(|s| s.category == "Backend"));
    }

    #[tokio::test]
    async fn test_get_by_min_level() {
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        repo.create(&skill_data).await.unwrap();
        
        let skills = repo.get_by_min_level(3).await.unwrap();
        assert!(skills.len() >= 1);
        assert!(skills.iter().all(|s| s.level >= 3));
    }

    #[tokio::test]
    async fn test_update_skill() {
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        let created = repo.create(&skill_data).await.unwrap();
        
        let update_data = UpdateSkill {
            name: Some("Advanced Rust".to_string()),
            level: Some(5),
            years_experience: Some(5),
            ..Default::default()
        };
        
        let updated = repo.update(created.id, &update_data).await.unwrap().unwrap();
        assert_eq!(updated.name, "Advanced Rust");
        assert_eq!(updated.level, 5);
        assert_eq!(updated.years_experience, Some(5));
    }

    #[tokio::test]
    async fn test_delete_skill() {
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        let created = repo.create(&skill_data).await.unwrap();
        
        let deleted = repo.delete(created.id).await.unwrap();
        assert!(deleted);
        
        let retrieved = repo.get_by_id(created.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_get_categories() {
        let repo = create_test_repository().await;
        let skill_data = create_test_skill();
        
        repo.create(&skill_data).await.unwrap();
        
        let categories = repo.get_categories().await.unwrap();
        assert!(categories.contains(&"Backend".to_string()));
    }
}

impl Default for UpdateSkill {
    fn default() -> Self {
        Self {
            name: None,
            category: None,
            level: None,
            years_experience: None,
            description: None,
        }
    }
}