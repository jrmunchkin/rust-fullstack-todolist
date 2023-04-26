use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoCreateRequest {
    pub name: String,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoUpdateRequest {
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoResponse {
    pub id: String,
    pub name: String,
    pub is_complete: bool,
}

impl TodoResponse {
    pub fn of(todo: Todo) -> Self {
        Self {
            id: todo.id,
            name: todo.name,
            is_complete: todo.is_complete,
        }
    }
}
