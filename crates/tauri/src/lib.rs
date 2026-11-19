use common::AppInfo;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct AppState {
    storage: Mutex<HashMap<String, String>>,
    session_token: Mutex<Option<String>>,
}

// Example Tauri command that returns shared AppInfo from crates/common
#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo::default()
}

#[tauri::command]
fn set_storage_item(state: State<AppState>, key: String, value: String) {
    state.storage.lock().unwrap().insert(key, value);
}

#[tauri::command]
fn get_storage_item(state: State<AppState>, key: String) -> Option<String> {
    state.storage.lock().unwrap().get(&key).cloned()
}

#[tauri::command]
fn remove_storage_item(state: State<AppState>, key: String) {
    state.storage.lock().unwrap().remove(&key);
}

#[tauri::command]
fn get_session_token(state: State<AppState>) -> Option<String> {
    state.session_token.lock().unwrap().clone()
}

#[tauri::command]
fn set_session_token(state: State<AppState>, token: String) {
    *state.session_token.lock().unwrap() = Some(token);
}

#[tauri::command]
fn clear_session_token(state: State<AppState>) {
    *state.session_token.lock().unwrap() = None;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_storage_item,
            set_storage_item,
            remove_storage_item,
            get_session_token,
            set_session_token,
            clear_session_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
