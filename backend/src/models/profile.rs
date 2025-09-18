use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use validator::Validate;

/// Profile model representing the developer's profile information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub bio: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: String,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub twitter_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Update profile request model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProfile {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 1000, message = "Bio must be between 1 and 1000 characters"))]
    pub bio: Option<String>,
    
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: Option<String>,
    
    #[validate(length(max = 20, message = "Phone number must be less than 20 characters"))]
    pub phone: Option<String>,
    
    #[validate(length(min = 1, max = 100, message = "Location must be between 1 and 100 characters"))]
    pub location: Option<String>,
    
    #[validate(url(message = "LinkedIn URL must be a valid URL"))]
    pub linkedin_url: Option<String>,
    
    #[validate(url(message = "GitHub URL must be a valid URL"))]
    pub github_url: Option<String>,
    
    #[validate(url(message = "Twitter URL must be a valid URL"))]
    pub twitter_url: Option<String>,
}

impl Profile {
    /// Get social media links as a vector of tuples (platform, url)
    pub fn get_social_links(&self) -> Vec<(String, String)> {
        let mut links = Vec::new();
        
        if let Some(ref linkedin) = self.linkedin_url {
            links.push(("LinkedIn".to_string(), linkedin.clone()));
        }
        
        if let Some(ref github) = self.github_url {
            links.push(("GitHub".to_string(), github.clone()));
        }
        
        if let Some(ref twitter) = self.twitter_url {
            links.push(("Twitter".to_string(), twitter.clone()));
        }
        
        links
    }

    /// Check if profile has complete basic information
    pub fn is_complete(&self) -> bool {
        !self.name.is_empty() 
            && !self.title.is_empty() 
            && !self.bio.is_empty() 
            && !self.email.is_empty() 
            && !self.location.is_empty()
    }

    /// Get display name (name or email if name is empty)
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.email
        } else {
            &self.name
        }
    }
}

impl UpdateProfile {
    /// Check if any field is being updated
    pub fn has_updates(&self) -> bool {
        self.name.is_some()
            || self.title.is_some()
            || self.bio.is_some()
            || self.email.is_some()
            || self.phone.is_some()
            || self.location.is_some()
            || self.linkedin_url.is_some()
            || self.github_url.is_some()
            || self.twitter_url.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    fn create_test_profile() -> Profile {
        Profile {
            id: 1,
            name: "John Doe".to_string(),
            title: "Full Stack Developer".to_string(),
            bio: "Passionate developer with expertise in modern web technologies.".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: Some("+1234567890".to_string()),
            location: "Paris, France".to_string(),
            linkedin_url: Some("https://linkedin.com/in/johndoe".to_string()),
            github_url: Some("https://github.com/johndoe".to_string()),
            twitter_url: Some("https://twitter.com/johndoe".to_string()),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_update_profile_validation_success() {
        let update = UpdateProfile {
            name: Some("Jane Doe".to_string()),
            title: Some("Senior Developer".to_string()),
            bio: Some("Updated bio".to_string()),
            email: Some("jane.doe@example.com".to_string()),
            phone: Some("+0987654321".to_string()),
            location: Some("London, UK".to_string()),
            linkedin_url: Some("https://linkedin.com/in/janedoe".to_string()),
            github_url: Some("https://github.com/janedoe".to_string()),
            twitter_url: Some("https://twitter.com/janedoe".to_string()),
        };

        assert!(update.validate().is_ok());
    }

    #[test]
    fn test_update_profile_validation_invalid_email() {
        let update = UpdateProfile {
            name: None,
            title: None,
            bio: None,
            email: Some("invalid-email".to_string()),
            phone: None,
            location: None,
            avatar_url: None,
            linkedin_url: None,
            github_url: None,
            twitter_url: None,
        };

        assert!(update.validate().is_err());
    }

    #[test]
    fn test_update_profile_validation_invalid_url() {
        let update = UpdateProfile {
            name: None,
            title: None,
            bio: None,
            email: None,
            phone: None,
            location: None,
            linkedin_url: None,
            github_url: None,
            twitter_url: None,
        };

        assert!(update.validate().is_err());
    }

    #[test]
    fn test_profile_get_social_links() {
        let profile = create_test_profile();
        let links = profile.get_social_links();
        
        assert_eq!(links.len(), 3);
        assert!(links.iter().any(|(platform, _)| platform == "LinkedIn"));
        assert!(links.iter().any(|(platform, _)| platform == "GitHub"));
        assert!(links.iter().any(|(platform, _)| platform == "Twitter"));
    }

    #[test]
    fn test_profile_is_complete() {
        let complete_profile = create_test_profile();
        assert!(complete_profile.is_complete());

        let incomplete_profile = Profile {
            name: "".to_string(),
            ..complete_profile
        };
        assert!(!incomplete_profile.is_complete());
    }

    #[test]
    fn test_profile_display_name() {
        let profile = create_test_profile();
        assert_eq!(profile.display_name(), "John Doe");

        let profile_no_name = Profile {
            name: "".to_string(),
            ..profile
        };
        assert_eq!(profile_no_name.display_name(), "john.doe@example.com");
    }

    #[test]
    fn test_update_profile_has_updates() {
        let update_with_changes = UpdateProfile {
            name: Some("New Name".to_string()),
            title: None,
            bio: None,
            email: None,
            phone: None,
            location: None,
            linkedin_url: None,
            github_url: None,
            twitter_url: None,
        };
        assert!(update_with_changes.has_updates());

        let update_no_changes = UpdateProfile {
            name: None,
            title: None,
            bio: None,
            email: None,
            phone: None,
            location: None,
            linkedin_url: None,
            github_url: None,
            twitter_url: None,
        };
        assert!(!update_no_changes.has_updates());
    }
}