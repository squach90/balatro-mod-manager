use bmm_lib::finder::{is_balatro_running, is_steam_running};

#[tauri::command]
pub async fn check_steam_running() -> bool {
    is_steam_running()
}

#[tauri::command]
pub async fn check_balatro_running() -> bool {
    is_balatro_running()
}

