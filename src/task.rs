use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    // Конструктор для створення нового завдання
    pub fn new(id: u32, title: String, description: String, completed: bool) -> Self {
        Task {
            id,
            title,
            description,
            completed,
        }
    }

    // Метод для відображення завдання (як String)
    pub fn to_string(&self) -> String {
        format!(
            "[{}] {}: {} - {}",
            if self.completed { "X" } else { " " },
            self.id,
            self.title,
            self.description
        )
    }

    // Метод для серіалізації завдання в JSON
    pub fn to_hash(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "title": self.title,
            "description": self.description,
            "completed": self.completed,
        })
    }

    // Метод для десеріалізації завдання з JSON
    pub fn from_hash(hash: &serde_json::Value) -> Self {
        Task {
            id: hash["id"].as_u64().unwrap() as u32,
            title: hash["title"].as_str().unwrap().to_string(),
            description: hash["description"].as_str().unwrap().to_string(),
            completed: hash["completed"].as_bool().unwrap(),
        }
    }
}
