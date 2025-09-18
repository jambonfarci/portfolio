#[cfg(test)]
mod project_tests {
    use super::*;

    #[test]
    fn test_create_project_validation() {
        let valid_project = CreateProject {
            title: "Valid Project".to_string(),
            description: "Valid description".to_string(),
            long_description: None,
            technologies: vec!["Rust".to_string()],
            github_url: None,
            demo_url: None,
            image_url: None,
            category: "Web".to_string(),
            featured: Some(false),
        };

        // This should not panic
        let _json = serde_json::to_string(&valid_project).expect("Valid project should serialize");
    }

    #[test]
    fn test_update_project_partial() {
        let update = UpdateProject {
            title: Some("Updated Title".to_string()),
            description: None,
            long_description: Some("New description".to_string()),
            technologies: None,
            github_url: Some("https://github.com/updated".to_string()),
            demo_url: None,
            image_url: None,
            category: None,
            featured: Some(true),
        };

        let json = serde_json::to_string(&update).expect("Failed to serialize update");
        let deserialized: UpdateProject = serde_json::from_str(&json).expect("Failed to deserialize update");
        
        assert_eq!(update.title, deserialized.title);
        assert_eq!(update.featured, deserialized.featured);
    }
}

#[cfg(test)]
mod skill_tests {
    use super::*;

    #[test]
    fn test_create_skill_validation() {
        let valid_skill = CreateSkill {
            name: "TypeScript".to_string(),
            category: "Frontend".to_string(),
            level: 5,
            years_experience: Some(4),
            description: Some("Typed JavaScript".to_string()),
        };

        let json = serde_json::to_string(&valid_skill).expect("Valid skill should serialize");
        let _deserialized: CreateSkill = serde_json::from_str(&json).expect("Valid skill should deserialize");
    }
}

#[cfg(test)]
mod contact_tests {
    use super::*;

    #[test]
    fn test_create_contact_message_validation() {
        let valid_message = CreateContactMessage {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test message content".to_string(),
        };

        let json = serde_json::to_string(&valid_message).expect("Valid message should serialize");
        let _deserialized: CreateContactMessage = serde_json::from_str(&json).expect("Valid message should deserialize");
    }
}