use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use validator::Validate;

/// Skill model representing a technical skill
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub level: i32,
    pub years_experience: Option<i32>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Create skill request model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSkill {
    #[validate(length(min = 1, max = 100, message = "Skill name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: String,
    
    #[validate(range(min = 1, max = 5, message = "Level must be between 1 and 5"))]
    pub level: i32,
    
    #[validate(range(min = 0, max = 50, message = "Years of experience must be between 0 and 50"))]
    pub years_experience: Option<i32>,
    
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
}

/// Update skill request model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateSkill {
    #[validate(length(min = 1, max = 100, message = "Skill name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: Option<String>,
    
    #[validate(range(min = 1, max = 5, message = "Level must be between 1 and 5"))]
    pub level: Option<i32>,
    
    #[validate(range(min = 0, max = 50, message = "Years of experience must be between 0 and 50"))]
    pub years_experience: Option<i32>,
    
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
}

/// Skill categories enum for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillCategory {
    Frontend,
    Backend,
    Database,
    DevOps,
    Tools,
    Mobile,
    Other,
}

impl SkillCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkillCategory::Frontend => "Frontend",
            SkillCategory::Backend => "Backend",
            SkillCategory::Database => "Database",
            SkillCategory::DevOps => "DevOps",
            SkillCategory::Tools => "Tools",
            SkillCategory::Mobile => "Mobile",
            SkillCategory::Other => "Other",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Frontend" => Some(SkillCategory::Frontend),
            "Backend" => Some(SkillCategory::Backend),
            "Database" => Some(SkillCategory::Database),
            "DevOps" => Some(SkillCategory::DevOps),
            "Tools" => Some(SkillCategory::Tools),
            "Mobile" => Some(SkillCategory::Mobile),
            "Other" => Some(SkillCategory::Other),
            _ => None,
        }
    }

    pub fn all() -> Vec<&'static str> {
        vec!["Frontend", "Backend", "Database", "DevOps", "Tools", "Mobile", "Other"]
    }
}

impl Skill {
    /// Get skill level as a descriptive string
    pub fn level_description(&self) -> &'static str {
        match self.level {
            1 => "Beginner",
            2 => "Novice",
            3 => "Intermediate",
            4 => "Advanced",
            5 => "Expert",
            _ => "Unknown",
        }
    }

    /// Check if skill category is valid
    pub fn is_valid_category(&self) -> bool {
        SkillCategory::from_str(&self.category).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_skill_validation_success() {
        let skill = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: Some("Systems programming language".to_string()),
        };

        assert!(skill.validate().is_ok());
    }

    #[test]
    fn test_create_skill_validation_empty_name() {
        let skill = CreateSkill {
            name: "".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: None,
        };

        assert!(skill.validate().is_err());
    }

    #[test]
    fn test_create_skill_validation_invalid_level() {
        let skill = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 6, // Invalid level
            years_experience: Some(3),
            description: None,
        };

        assert!(skill.validate().is_err());
    }

    #[test]
    fn test_create_skill_validation_negative_years() {
        let skill = CreateSkill {
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(-1), // Invalid years
            description: None,
        };

        assert!(skill.validate().is_err());
    }

    #[test]
    fn test_skill_level_description() {
        let skill = Skill {
            id: 1,
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: None,
            created_at: Utc::now(),
        };

        assert_eq!(skill.level_description(), "Advanced");
    }

    #[test]
    fn test_skill_category_validation() {
        let skill = Skill {
            id: 1,
            name: "Rust".to_string(),
            category: "Backend".to_string(),
            level: 4,
            years_experience: Some(3),
            description: None,
            created_at: Utc::now(),
        };

        assert!(skill.is_valid_category());

        let invalid_skill = Skill {
            id: 2,
            name: "Test".to_string(),
            category: "InvalidCategory".to_string(),
            level: 3,
            years_experience: None,
            description: None,
            created_at: Utc::now(),
        };

        assert!(!invalid_skill.is_valid_category());
    }

    #[test]
    fn test_skill_category_enum() {
        assert_eq!(SkillCategory::Backend.as_str(), "Backend");
        assert!(SkillCategory::from_str("Frontend").is_some());
        assert!(SkillCategory::from_str("InvalidCategory").is_none());
        
        let all_categories = SkillCategory::all();
        assert!(all_categories.contains(&"Frontend"));
        assert!(all_categories.contains(&"Backend"));
    }
}