use serde::{Deserialize, Serialize};

// This is the canonical Todo struct that represents a record in your database.
// I've added `#[derive(Debug)]` so it can be printed in main.rs.
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    // pub completed: bool,
    // Using chrono::NaiveDateTime would be another good option if you add chrono as a dependency.
    // pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTodo {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveTodo {
    pub id: i32,
}

// Added Serialize and Deserialize for consistency
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
}
