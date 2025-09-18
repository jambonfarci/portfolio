use sqlx::SqlitePool;
use chrono::Utc;
use crate::models::{Profile, UpdateProfile};

/// Repository for profile database operations
pub struct ProfileRepository {
    pool: SqlitePool,
}

impl ProfileRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get the profile (there should only be one)
    pub async fn get(&self) -> Result<Option<Profile>, sqlx::Error> {
        sqlx::query_as::<_, Profile>(
            "SELECT id, name, title, bio, email, phone, location, linkedin_url, github_url, twitter_url, updated_at FROM profile WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// Update the profile
    pub async fn update(&self, profile: &UpdateProfile) -> Result<Option<Profile>, sqlx::Error> {
        // Check if profile exists
        if self.get().await?.is_none() {
            return Ok(None);
        }

        let now = Utc::now();

        // Use COALESCE to keep existing values for fields that are None
        sqlx::query(
            r#"
            UPDATE profile SET 
                name = COALESCE(?, name),
                title = COALESCE(?, title),
                bio = COALESCE(?, bio),
                email = COALESCE(?, email),
                phone = COALESCE(?, phone),
                location = COALESCE(?, location),
                linkedin_url = COALESCE(?, linkedin_url),
                github_url = COALESCE(?, github_url),
                twitter_url = COALESCE(?, twitter_url),
                updated_at = ?
            WHERE id = 1
            "#
        )
        .bind(&profile.name)
        .bind(&profile.title)
        .bind(&profile.bio)
        .bind(&profile.email)
        .bind(&profile.phone)
        .bind(&profile.location)
        .bind(&profile.linkedin_url)
        .bind(&profile.github_url)
        .bind(&profile.twitter_url)
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        self.get().await
    }

    /// Create initial profile (used during setup)
    pub async fn create_initial(&self, name: &str, title: &str, bio: &str, email: &str, location: &str) -> Result<Profile, sqlx::Error> {
        let now = Utc::now();
        
        sqlx::query(
            "INSERT OR REPLACE INTO profile (id, name, title, bio, email, location, updated_at) VALUES (1, ?, ?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(title)
        .bind(bio)
        .bind(email)
        .bind(location)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get().await?.ok_or(sqlx::Error::RowNotFound)
    }

    /// Check if profile exists
    pub async fn exists(&self) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM profile WHERE id = 1")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    async fn create_test_repository() -> ProfileRepository {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables manually for testing
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

        ProfileRepository::new(pool)
    }

    #[tokio::test]
    async fn test_get_profile() {
        let repo = create_test_repository().await;
        
        // Profile should exist from seed data
        let profile = repo.get().await.unwrap();
        assert!(profile.is_some());
        
        let profile = profile.unwrap();
        assert_eq!(profile.id, 1);
        assert!(!profile.name.is_empty());
        assert!(!profile.email.is_empty());
    }

    #[tokio::test]
    async fn test_update_profile() {
        let repo = create_test_repository().await;
        
        let update_data = UpdateProfile {
            name: Some("Updated Name".to_string()),
            title: Some("Updated Title".to_string()),
            bio: Some("Updated bio content".to_string()),
            phone: Some("+1234567890".to_string()),
            ..Default::default()
        };
        
        let updated = repo.update(&update_data).await.unwrap().unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.bio, "Updated bio content");
        assert_eq!(updated.phone, Some("+1234567890".to_string()));
    }

    #[tokio::test]
    async fn test_profile_exists() {
        let repo = create_test_repository().await;
        
        let exists = repo.exists().await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_create_initial_profile() {
        // Create a fresh database without seed data
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        // Create tables manually
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

        let repo = ProfileRepository::new(pool);
        
        let profile = repo.create_initial(
            "Test User",
            "Test Title",
            "Test bio",
            "test@example.com",
            "Test Location"
        ).await.unwrap();
        
        assert_eq!(profile.name, "Test User");
        assert_eq!(profile.title, "Test Title");
        assert_eq!(profile.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_update_empty_profile() {
        let repo = create_test_repository().await;
        
        let update_data = UpdateProfile::default();
        
        // Should return existing profile without changes
        let result = repo.update(&update_data).await.unwrap();
        assert!(result.is_some());
    }
}

impl Default for UpdateProfile {
    fn default() -> Self {
        Self {
            name: None,
            title: None,
            bio: None,
            email: None,
            phone: None,
            location: None,

            linkedin_url: None,
            github_url: None,
            twitter_url: None,
        }
    }
}