use crate::domain::{
    entities::User,
    errors::{AppError, AppResult},
    requests::{CreateUserRequest, ListUsersQuery, UpdateUserRequest},
    responses::{UserProfileResponse, UserResponse, UsersListResponse},
};
use crate::repositories::user::UserRepository;
use std::sync::Arc;
use tracing;
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
        tracing::info!("Creating new user");
        
        // Validate request
        request.validate()
            .map_err(|e| {
                tracing::warn!(error = %e, "User creation validation failed");
                AppError::InvalidInput(e)
            })?;

        // Create user entity
        let user = User::new(request.name.trim().to_string(), request.email.trim().to_lowercase(), request.age);

        tracing::debug!(user_id = %user.id, email = %user.email, "User entity created");

        // Save to repository
        let created_user = self.repository.create(user)?;

        tracing::info!(
            user_id = %created_user.id,
            user_name = %created_user.name,
            user_email = %created_user.email,
            "User created successfully"
        );

        Ok(UserResponse::from(created_user))
    }

    pub fn get_user(&self, id: Uuid) -> AppResult<UserResponse> {
        tracing::debug!(user_id = %id, "Fetching user");
        
        let user = self.repository.find_by_id(id)?;
        
        tracing::info!(
            user_id = %user.id,
            user_name = %user.name,
            "User retrieved successfully"
        );
        
        Ok(UserResponse::from(user))
    }

    pub fn get_user_profile(&self, id: Uuid) -> AppResult<UserProfileResponse> {
        tracing::debug!(user_id = %id, "Fetching user profile");
        
        let user = self.repository.find_by_id(id)?;
        
        tracing::info!(
            user_id = %user.id,
            user_name = %user.name,
            "User profile retrieved successfully"
        );
        
        Ok(UserProfileResponse::from(user))
    }

    pub fn list_users(&self, query: ListUsersQuery) -> AppResult<UsersListResponse> {
        tracing::debug!(
            limit = query.limit,
            offset = query.offset,
            "Listing users"
        );
        
        // Validate query
        query.validate()
            .map_err(|e| {
                tracing::warn!(error = %e, "User list validation failed");
                AppError::InvalidInput(e)
            })?;

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

        tracing::info!(
            total_users = total,
            returned_count = users.len(),
            offset = offset,
            limit = limit,
            "Users listed successfully"
        );

        Ok(UsersListResponse {
            users,
            total,
            limit,
            offset,
        })
    }

    pub fn update_user(&self, id: Uuid, request: UpdateUserRequest) -> AppResult<UserResponse> {
        tracing::info!(user_id = %id, "Updating user");
        
        // Validate request
        request.validate()
            .map_err(|e| {
                tracing::warn!(user_id = %id, error = %e, "User update validation failed");
                AppError::InvalidInput(e)
            })?;

        if !request.has_updates() {
            tracing::warn!(user_id = %id, "Update request has no updates");
            return Err(AppError::InvalidInput("No updates provided".to_string()));
        }

        // Get existing user
        let mut user = self.repository.find_by_id(id)?;
        let original_name = user.name.clone();
        let original_email = user.email.clone();
        let original_age = user.age;

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

        tracing::info!(
            user_id = %updated_user.id,
            user_name = %updated_user.name,
            user_email = %updated_user.email,
            original_name = %original_name,
            original_email = %original_email,
            original_age = original_age,
            "User updated successfully"
        );

        Ok(UserResponse::from(updated_user))
    }

    pub fn delete_user(&self, id: Uuid) -> AppResult<()> {
        tracing::info!(user_id = %id, "Deleting user");
        
        // Verify user exists before deletion
        let user = self.repository.find_by_id(id)?;
        
        // Delete user
        self.repository.delete(id)?;

        tracing::info!(
            user_id = %user.id,
            user_name = %user.name,
            user_email = %user.email,
            "User deleted successfully"
        );

        Ok(())
    }
}