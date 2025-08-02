use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, email: String, age: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            age,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn update_age(&mut self, age: u32) {
        self.age = age;
    }
}