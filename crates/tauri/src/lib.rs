use common::AppInfo;

// Example Tauri command that returns shared AppInfo from crates/common
#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo::default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
