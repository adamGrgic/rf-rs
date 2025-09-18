use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct Todo {
    pub id: i32,
    pub item: String,
    pub description: String
}
