use crate::domain::entities::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            age: user.age,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub profile_url: String,
    pub created_at: String,
    pub is_adult: bool,
}

impl From<User> for UserProfileResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            age: user.age,
            profile_url: format!("/api/v1/users/{}/profile", user.id),
            created_at: "2024-01-01T00:00:00Z".to_string(), // Mock timestamp
            is_adult: user.is_adult(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub service: String,
    pub version: String,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            service: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}