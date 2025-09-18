use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use validator::Validate;

/// Contact message model representing messages from the contact form
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContactMessage {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

/// Create contact message request model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateContactMessage {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 1, max = 200, message = "Subject must be between 1 and 200 characters"))]
    pub subject: String,
    
    #[validate(length(min = 1, max = 2000, message = "Message must be between 1 and 2000 characters"))]
    pub message: String,
}

impl ContactMessage {
    /// Get a short preview of the message (first 100 characters)
    pub fn message_preview(&self) -> String {
        if self.message.len() <= 100 {
            self.message.clone()
        } else {
            format!("{}...", &self.message[..97])
        }
    }

    /// Check if the message is recent (within last 24 hours)
    pub fn is_recent(&self) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.created_at);
        duration.num_hours() < 24
    }

    /// Get formatted creation date
    pub fn formatted_date(&self) -> String {
        self.created_at.format("%Y-%m-%d %H:%M UTC").to_string()
    }

    /// Check if message appears to be spam (basic heuristics)
    pub fn is_likely_spam(&self) -> bool {
        let spam_keywords = [
            "viagra", "casino", "lottery", "winner", "congratulations",
            "click here", "free money", "urgent", "act now", "limited time"
        ];
        
        let message_lower = self.message.to_lowercase();
        let subject_lower = self.subject.to_lowercase();
        
        // Check for spam keywords
        let has_spam_keywords = spam_keywords.iter().any(|&keyword| {
            message_lower.contains(keyword) || subject_lower.contains(keyword)
        });
        
        // Check for excessive capitalization
        let caps_ratio = self.message.chars()
            .filter(|c| c.is_alphabetic())
            .fold((0, 0), |(caps, total), c| {
                if c.is_uppercase() {
                    (caps + 1, total + 1)
                } else {
                    (caps, total + 1)
                }
            });
        
        let excessive_caps = if caps_ratio.1 > 0 {
            (caps_ratio.0 as f32 / caps_ratio.1 as f32) > 0.5
        } else {
            false
        };
        
        // Check for excessive exclamation marks
        let exclamation_count = self.message.matches('!').count();
        let excessive_exclamations = exclamation_count > 5;
        
        has_spam_keywords || excessive_caps || excessive_exclamations
    }
}

impl CreateContactMessage {
    /// Sanitize input by trimming whitespace and removing potentially harmful content
    pub fn sanitize(&mut self) {
        self.name = self.name.trim().to_string();
        self.email = self.email.trim().to_lowercase();
        self.subject = self.subject.trim().to_string();
        self.message = self.message.trim().to_string();
        
        // Remove any null bytes or control characters
        self.name = self.name.chars().filter(|c| !c.is_control() || *c == '\n' || *c == '\t').collect();
        self.subject = self.subject.chars().filter(|c| !c.is_control() || *c == '\n' || *c == '\t').collect();
        self.message = self.message.chars().filter(|c| !c.is_control() || *c == '\n' || *c == '\t').collect();
    }

    /// Check if the message content appears to be valid
    pub fn is_valid_content(&self) -> bool {
        // Check for minimum meaningful content
        let word_count = self.message.split_whitespace().count();
        word_count >= 3 && !self.message.chars().all(|c| c.is_numeric() || c.is_whitespace())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    fn create_test_contact_message() -> ContactMessage {
        ContactMessage {
            id: 1,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Inquiry about services".to_string(),
            message: "Hello, I'm interested in your web development services. Could you please provide more information about your rates and availability?".to_string(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_create_contact_message_validation_success() {
        let message = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message with sufficient content.".to_string(),
        };

        assert!(message.validate().is_ok());
    }

    #[test]
    fn test_create_contact_message_validation_empty_name() {
        let message = CreateContactMessage {
            name: "".to_string(),
            email: "john.doe@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message.".to_string(),
        };

        assert!(message.validate().is_err());
    }

    #[test]
    fn test_create_contact_message_validation_invalid_email() {
        let message = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "invalid-email".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a test message.".to_string(),
        };

        assert!(message.validate().is_err());
    }

    #[test]
    fn test_contact_message_preview() {
        let message = create_test_contact_message();
        let preview = message.message_preview();
        
        assert!(preview.len() <= 100);
        if message.message.len() > 100 {
            assert!(preview.ends_with("..."));
        }
    }

    #[test]
    fn test_contact_message_is_recent() {
        let recent_message = ContactMessage {
            created_at: Utc::now(),
            ..create_test_contact_message()
        };
        assert!(recent_message.is_recent());

        let old_message = ContactMessage {
            created_at: Utc::now() - chrono::Duration::days(2),
            ..create_test_contact_message()
        };
        assert!(!old_message.is_recent());
    }

    #[test]
    fn test_contact_message_formatted_date() {
        let message = create_test_contact_message();
        let formatted = message.formatted_date();
        
        assert!(formatted.contains("UTC"));
        assert!(formatted.len() > 10); // Should have date and time
    }

    #[test]
    fn test_contact_message_spam_detection() {
        let spam_message = ContactMessage {
            subject: "URGENT: You won the lottery!".to_string(),
            message: "CONGRATULATIONS!!! Click here to claim your FREE MONEY!!!".to_string(),
            ..create_test_contact_message()
        };
        assert!(spam_message.is_likely_spam());

        let normal_message = create_test_contact_message();
        assert!(!normal_message.is_likely_spam());
    }

    #[test]
    fn test_create_contact_message_sanitize() {
        let mut message = CreateContactMessage {
            name: "  John Doe  ".to_string(),
            email: "  JOHN.DOE@EXAMPLE.COM  ".to_string(),
            subject: "  Test Subject  ".to_string(),
            message: "  This is a test message.  ".to_string(),
        };

        message.sanitize();

        assert_eq!(message.name, "John Doe");
        assert_eq!(message.email, "john.doe@example.com");
        assert_eq!(message.subject, "Test Subject");
        assert_eq!(message.message, "This is a test message.");
    }

    #[test]
    fn test_create_contact_message_valid_content() {
        let valid_message = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test".to_string(),
            message: "This is a valid message with multiple words.".to_string(),
        };
        assert!(valid_message.is_valid_content());

        let invalid_message = CreateContactMessage {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test".to_string(),
            message: "123".to_string(),
        };
        assert!(!invalid_message.is_valid_content());
    }
}