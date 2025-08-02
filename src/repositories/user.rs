use crate::domain::{entities::User, errors::AppResult};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserRepository {
    users: Mutex<HashMap<Uuid, User>>,
}

impl UserRepository {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // Add sample data
        let user1 = User::new(
            "Alice Johnson".to_string(),
            "alice@example.com".to_string(),
            28,
        );
        let user2 = User::new(
            "Bob Smith".to_string(),
            "bob@example.com".to_string(),
            32,
        );
        
        users.insert(user1.id, user1);
        users.insert(user2.id, user2);

        Self {
            users: Mutex::new(users),
        }
    }

    pub fn create(&self, user: User) -> AppResult<User> {
        let mut users = self.users.lock().unwrap();
        
        // Check if email already exists
        if users.values().any(|u| u.email == user.email) {
            return Err(crate::domain::errors::AppError::UserAlreadyExists(user.email));
        }
        
        users.insert(user.id, user.clone());
        Ok(user)
    }

    pub fn find_by_id(&self, id: Uuid) -> AppResult<User> {
        let users = self.users.lock().unwrap();
        users
            .get(&id)
            .cloned()
            .ok_or(crate::domain::errors::AppError::UserNotFound(id))
    }

    pub fn find_all(&self) -> AppResult<Vec<User>> {
        let users = self.users.lock().unwrap();
        let mut user_list: Vec<User> = users.values().cloned().collect();
        user_list.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(user_list)
    }

    pub fn update(&self, id: Uuid, mut user: User) -> AppResult<User> {
        let mut users = self.users.lock().unwrap();
        
        // Check if user exists
        if !users.contains_key(&id) {
            return Err(crate::domain::errors::AppError::UserNotFound(id));
        }
        
        // Check if email already exists for another user
        if users.values().any(|u| u.id != id && u.email == user.email) {
            return Err(crate::domain::errors::AppError::UserAlreadyExists(user.email));
        }
        
        user.id = id; // Ensure ID matches
        users.insert(id, user.clone());
        Ok(user)
    }

    pub fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut users = self.users.lock().unwrap();
        users
            .remove(&id)
            .ok_or(crate::domain::errors::AppError::UserNotFound(id))?;
        Ok(())
    }

    pub fn count(&self) -> usize {
        let users = self.users.lock().unwrap();
        users.len()
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}