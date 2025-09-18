use sqlx::SqlitePool;
use validator::Validate;
use tracing::{info, warn, error};
use crate::{
    database::ProfileRepository,
    models::{Profile, UpdateProfile},
    error::{ApiError, ApiResult},
};

/// Service for profile-related business logic
pub struct ProfileService {
    repository: ProfileRepository,
}

impl ProfileService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repository: ProfileRepository::new(pool),
        }
    }

    /// Get the profile
    pub async fn get_profile(&self) -> ApiResult<Profile> {
        info!("Fetching profile");
        
        match self.repository.get().await {
            Ok(Some(profile)) => {
                info!("Successfully fetched profile for: {}", profile.name);
                Ok(profile)
            }
            Ok(None) => {
                warn!("Profile not found");
                Err(ApiError::NotFound("Profile not found".to_string()))
            }
            Err(e) => {
                error!("Failed to fetch profile: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Update the profile
    pub async fn update_profile(&self, mut profile_data: UpdateProfile) -> ApiResult<Profile> {
        info!("Updating profile");
        
        // Validate input data
        if let Err(validation_errors) = profile_data.validate() {
            warn!("Validation failed for profile update: {:?}", validation_errors);
            return Err(ApiError::from_validation_errors(validation_errors));
        }

        // Check if there are any updates to apply
        if !profile_data.has_updates() {
            return Err(ApiError::BadRequest("No updates provided".to_string()));
        }

        // Sanitize data if provided
        if let Some(ref mut name) = profile_data.name {
            *name = name.trim().to_string();
            if name.is_empty() {
                return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
            }
        }
        
        if let Some(ref mut title) = profile_data.title {
            *title = title.trim().to_string();
            if title.is_empty() {
                return Err(ApiError::BadRequest("Title cannot be empty".to_string()));
            }
        }
        
        if let Some(ref mut bio) = profile_data.bio {
            *bio = bio.trim().to_string();
            if bio.is_empty() {
                return Err(ApiError::BadRequest("Bio cannot be empty".to_string()));
            }
        }
        
        if let Some(ref mut email) = profile_data.email {
            *email = email.trim().to_lowercase();
        }
        
        if let Some(ref mut location) = profile_data.location {
            *location = location.trim().to_string();
            if location.is_empty() {
                return Err(ApiError::BadRequest("Location cannot be empty".to_string()));
            }
        }



        match self.repository.update(&profile_data).await {
            Ok(Some(profile)) => {
                info!("Successfully updated profile for: {}", profile.name);
                Ok(profile)
            }
            Ok(None) => {
                warn!("Profile not found for update");
                Err(ApiError::NotFound("Profile not found".to_string()))
            }
            Err(e) => {
                error!("Failed to update profile: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Check if profile exists
    pub async fn profile_exists(&self) -> ApiResult<bool> {
        info!("Checking if profile exists");
        
        match self.repository.exists().await {
            Ok(exists) => {
                info!("Profile exists: {}", exists);
                Ok(exists)
            }
            Err(e) => {
                error!("Failed to check profile existence: {}", e);
                Err(ApiError::Database(e))
            }
        }
    }

    /// Get profile summary (basic info only)
    pub async fn get_profile_summary(&self) -> ApiResult<ProfileSummary> {
        let profile = self.get_profile().await?;
        
        let social_links = profile.get_social_links();
        Ok(ProfileSummary {
            name: profile.name,
            title: profile.title,
            location: profile.location,
            social_links,
        })
    }

    /// Validate URL format (basic validation)
    fn is_valid_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

/// Profile summary for public display
#[derive(Debug, serde::Serialize)]
pub struct ProfileSummary {
    pub name: String,
    pub title: String,
    pub location: String,
    pub social_links: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_service() -> ProfileService {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS profile (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                title TEXT NOT NULL,
                bio TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT,
                location TEXT NOT NULL,

                linkedin_url TEXT,
                github_url TEXT,
                twitter_url TEXT,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        // Insert test profile
        sqlx::query(
            "INSERT INTO profile (id, name, title, bio, email, location) VALUES (1, 'Test User', 'Test Title', 'Test bio', 'test@example.com', 'Test Location')"
        )
        .execute(&pool)
        .await
        .unwrap();

        ProfileService::new(pool)
    }

    #[tokio::test]
    async fn test_get_profile() {
        let service = create_test_service().await;
        
        let profile = service.get_profile().await.unwrap();
        assert_eq!(profile.name, "Test User");
        assert_eq!(profile.title, "Test Title");
        assert_eq!(profile.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_update_profile() {
        let service = create_test_service().await;
        
        let update_data = UpdateProfile {
            name: Some("Updated Name".to_string()),
            title: Some("Updated Title".to_string()),
            bio: Some("Updated bio".to_string()),
            ..Default::default()
        };
        
        let updated = service.update_profile(update_data).await.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.bio, "Updated bio");
    }

    #[tokio::test]
    async fn test_update_profile_validation_error() {
        let service = create_test_service().await;
        
        let update_data = UpdateProfile {
            email: Some("invalid-email".to_string()),
            ..Default::default()
        };
        
        let result = service.update_profile(update_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::Validation(_) | ApiError::ValidationErrors(_) => {},
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_profile_empty_fields() {
        let service = create_test_service().await;
        
        let update_data = UpdateProfile {
            name: Some("   ".to_string()), // Empty after trim
            ..Default::default()
        };
        
        let result = service.update_profile(update_data).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ApiError::BadRequest(_) => {},
            _ => panic!("Expected bad request error"),
        }
    }

    #[tokio::test]
    async fn test_profile_exists() {
        let service = create_test_service().await;
        
        let exists = service.profile_exists().await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_get_profile_summary() {
        let service = create_test_service().await;
        
        let summary = service.get_profile_summary().await.unwrap();
        assert_eq!(summary.name, "Test User");
        assert_eq!(summary.title, "Test Title");
        assert_eq!(summary.location, "Test Location");
    }
}