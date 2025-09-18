use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;


/// API error types for the portfolio application
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Validation errors: {0:?}")]
    ValidationErrors(Vec<String>),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Forbidden access")]
    Forbidden,
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl ApiError {
    /// Create a validation error from validator errors
    pub fn from_validation_errors(errors: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    let message = error.message.as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid value for field '{}'", field));
                    format!("{}: {}", field, message)
                })
            })
            .collect();

        if error_messages.len() == 1 {
            ApiError::Validation(error_messages[0].clone())
        } else {
            ApiError::ValidationErrors(error_messages)
        }
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Validation(_) | ApiError::ValidationErrors(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Serialization(_) => StatusCode::BAD_REQUEST,
        }
    }

    /// Get the error message for the response
    pub fn message(&self) -> String {
        match self {
            ApiError::Database(_) => "A database error occurred".to_string(),
            ApiError::Validation(msg) => msg.clone(),
            ApiError::ValidationErrors(errors) => errors.join(", "),
            ApiError::NotFound(msg) => msg.clone(),
            ApiError::Unauthorized => "Unauthorized access".to_string(),
            ApiError::Forbidden => "Forbidden access".to_string(),
            ApiError::Conflict(msg) => msg.clone(),
            ApiError::BadRequest(msg) => msg.clone(),
            ApiError::InternalServerError(_) => "An internal server error occurred".to_string(),
            ApiError::Serialization(_) => "Invalid data format".to_string(),
        }
    }

    /// Get detailed error information (for development/debugging)
    pub fn details(&self) -> Option<String> {
        match self {
            ApiError::Database(e) => Some(e.to_string()),
            ApiError::InternalServerError(msg) => Some(msg.clone()),
            ApiError::Serialization(e) => Some(e.to_string()),
            _ => None,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.message();
        
        let mut response_body = json!({
            "success": false,
            "error": {
                "code": status.as_u16(),
                "message": message
            }
        });

        // Add details in development mode (you might want to make this configurable)
        if let Some(details) = self.details() {
            if let Some(error_obj) = response_body.get_mut("error") {
                error_obj["details"] = json!(details);
            }
        }

        // Add validation errors as a separate field for better client handling
        if let ApiError::ValidationErrors(errors) = &self {
            if let Some(error_obj) = response_body.get_mut("error") {
                error_obj["validation_errors"] = json!(errors);
            }
        }

        (status, Json(response_body)).into_response()
    }
}

/// Result type alias for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// Success response wrapper
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data,
            message: Some(message),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[derive(Validate)]
    struct TestStruct {
        #[validate(length(min = 1, message = "Name is required"))]
        name: String,
        #[validate(email(message = "Invalid email format"))]
        email: String,
    }

    #[test]
    fn test_api_error_status_codes() {
        assert_eq!(ApiError::NotFound("test".to_string()).status_code(), StatusCode::NOT_FOUND);
        assert_eq!(ApiError::Unauthorized.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ApiError::Validation("test".to_string()).status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(ApiError::Database(sqlx::Error::RowNotFound).status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_api_error_messages() {
        let error = ApiError::NotFound("User not found".to_string());
        assert_eq!(error.message(), "User not found");

        let error = ApiError::Unauthorized;
        assert_eq!(error.message(), "Unauthorized access");

        let error = ApiError::ValidationErrors(vec!["Name is required".to_string(), "Invalid email".to_string()]);
        assert_eq!(error.message(), "Name is required, Invalid email");
    }

    #[test]
    fn test_validation_errors_conversion() {
        let test_data = TestStruct {
            name: "".to_string(),
            email: "invalid-email".to_string(),
        };

        let validation_result = test_data.validate();
        assert!(validation_result.is_err());

        let validation_errors = validation_result.unwrap_err();
        let api_error = ApiError::from_validation_errors(validation_errors);

        match api_error {
            ApiError::ValidationErrors(errors) => {
                assert!(errors.len() >= 1);
                assert!(errors.iter().any(|e| e.contains("name")));
            }
            _ => panic!("Expected ValidationErrors"),
        }
    }

    #[test]
    fn test_api_response_creation() {
        let response = ApiResponse::new("test data");
        assert!(response.success);
        assert_eq!(response.data, "test data");
        assert!(response.message.is_none());

        let response = ApiResponse::with_message("test data", "Success message".to_string());
        assert!(response.success);
        assert_eq!(response.data, "test data");
        assert_eq!(response.message, Some("Success message".to_string()));
    }
}