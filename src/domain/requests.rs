use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub age: u32,
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if self.name.len() > 100 {
            return Err("Name cannot exceed 100 characters".to_string());
        }

        if !self.email.contains('@') || self.email.len() < 5 {
            return Err("Invalid email format".to_string());
        }

        if self.email.len() > 255 {
            return Err("Email cannot exceed 255 characters".to_string());
        }

        if self.age > 150 {
            return Err("Age must be realistic".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u32>,
}

impl UpdateUserRequest {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("Name cannot be empty".to_string());
            }
            if name.len() > 100 {
                return Err("Name cannot exceed 100 characters".to_string());
            }
        }

        if let Some(ref email) = self.email {
            if !email.contains('@') || email.len() < 5 {
                return Err("Invalid email format".to_string());
            }
            if email.len() > 255 {
                return Err("Email cannot exceed 255 characters".to_string());
            }
        }

        if let Some(age) = self.age {
            if age > 150 {
                return Err("Age must be realistic".to_string());
            }
        }

        Ok(())
    }

    pub fn has_updates(&self) -> bool {
        self.name.is_some() || self.email.is_some() || self.age.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Default for ListUsersQuery {
    fn default() -> Self {
        Self {
            limit: Some(10),
            offset: Some(0),
        }
    }
}

impl ListUsersQuery {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 100 {
                return Err("Limit must be between 1 and 100".to_string());
            }
        }

        Ok(())
    }

    pub fn get_limit(&self) -> usize {
        self.limit.unwrap_or(10).min(100)
    }

    pub fn get_offset(&self) -> usize {
        self.offset.unwrap_or(0)
    }
}