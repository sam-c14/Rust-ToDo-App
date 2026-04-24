use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

pub type DB = Mutex<Vec<Task>>;

pub type BorrowedDB<'a> = MutexGuard<'a, Vec<Task>>;

#[derive(Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AppResponse {
    pub data: Option<Vec<Task>>,
    pub status_code: u16,
    pub message: String,
}

pub struct AppState {
    pub tasks: DB,
}
