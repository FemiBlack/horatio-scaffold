use common::AppInfo;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct StorageState(Mutex<HashMap<String, String>>);

// Example Tauri command that returns shared AppInfo from crates/common
#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo::default()
}

#[tauri::command]
fn set_storage_item(state: State<StorageState>, key: String, value: String) {
    state.0.lock().unwrap().insert(key, value);
}

#[tauri::command]
fn get_storage_item(state: State<StorageState>, key: String) -> Option<String> {
    state.0.lock().unwrap().get(&key).cloned()
}

#[tauri::command]
fn remove_storage_item(state: State<StorageState>, key: String) {
    state.0.lock().unwrap().remove(&key);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(StorageState::default())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_storage_item,
            set_storage_item,
            remove_storage_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
