use crate::domain::{
    entities::User,
    errors::{AppError, AppResult},
    requests::{CreateUserRequest, ListUsersQuery, UpdateUserRequest},
    responses::{UserProfileResponse, UserResponse, UsersListResponse},
};
use crate::repositories::user::UserRepository;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserService {
    repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }

    pub fn create_user(&self, request: CreateUserRequest) -> AppResult<UserResponse> {
        // Validate request
        request.validate()
            .map_err(|e| AppError::InvalidInput(e))?;

        // Create user entity
        let user = User::new(request.name.trim().to_string(), request.email.trim().to_lowercase(), request.age);

        // Save to repository
        let created_user = self.repository.create(user)?;

        info!("Created new user: {} ({})", created_user.name, created_user.id);

        Ok(UserResponse::from(created_user))
    }

    pub fn get_user(&self, id: Uuid) -> AppResult<UserResponse> {
        let user = self.repository.find_by_id(id)?;
        Ok(UserResponse::from(user))
    }

    pub fn get_user_profile(&self, id: Uuid) -> AppResult<UserProfileResponse> {
        let user = self.repository.find_by_id(id)?;
        Ok(UserProfileResponse::from(user))
    }

    pub fn list_users(&self, query: ListUsersQuery) -> AppResult<UsersListResponse> {
        // Validate query
        query.validate()
            .map_err(|e| AppError::InvalidInput(e))?;

        // Get all users
        let all_users = self.repository.find_all()?;
        let total = all_users.len();

        // Apply pagination
        let offset = query.get_offset();
        let limit = query.get_limit();

        let users: Vec<UserResponse> = all_users
            .into_iter()
            .skip(offset)
            .take(limit)
            .map(UserResponse::from)
            .collect();

        Ok(UsersListResponse {
            users,
            total,
            limit,
            offset,
        })
    }

    pub fn update_user(&self, id: Uuid, request: UpdateUserRequest) -> AppResult<UserResponse> {
        // Validate request
        request.validate()
            .map_err(|e| AppError::InvalidInput(e))?;

        if !request.has_updates() {
            warn!("Update request for user {} has no updates", id);
            return Err(AppError::InvalidInput("No updates provided".to_string()));
        }

        // Get existing user
        let mut user = self.repository.find_by_id(id)?;

        // Apply updates
        if let Some(name) = request.name {
            user.update_name(name.trim().to_string());
        }
        if let Some(email) = request.email {
            user.update_email(email.trim().to_lowercase());
        }
        if let Some(age) = request.age {
            user.update_age(age);
        }

        // Save updated user
        let updated_user = self.repository.update(id, user)?;

        info!("Updated user: {} ({})", updated_user.name, updated_user.id);

        Ok(UserResponse::from(updated_user))
    }

    pub fn delete_user(&self, id: Uuid) -> AppResult<()> {
        // Verify user exists before deletion
        let user = self.repository.find_by_id(id)?;
        
        // Delete user
        self.repository.delete(id)?;

        info!("Deleted user: {} ({})", user.name, user.id);

        Ok(())
    }
}