use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MockResponse {
    message: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create(invoke_message: String) -> MockResponse {
    MockResponse {
        message: String::from(format!("create called: {}", invoke_message)),
    }
}

#[tauri::command]
pub async fn find(invoke_message: String) -> MockResponse {
    MockResponse {
        message: String::from(format!("find called: {}", invoke_message)),
    }
}

#[tauri::command]
pub async fn find_all(invoke_message: String) -> MockResponse {
    MockResponse {
        message: String::from(format!("find_all called: {}", invoke_message)),
    }
}

#[tauri::command]
pub async fn update(invoke_message: String) -> MockResponse {
    MockResponse {
        message: String::from(format!("update called: {}", invoke_message)),
    }
}

#[tauri::command]
pub async fn delete(invoke_message: String) -> MockResponse {
    MockResponse {
        message: String::from(format!("delete called: {}", invoke_message)),
    }
}
