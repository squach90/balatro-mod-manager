use std::path::PathBuf;
use std::sync::Mutex;

use bmm_lib::balamod::find_balatros;
use bmm_lib::database::Database;

use tauri::Manager;

// #[tauri::command]
// async fn find_steam_balatro() -> Result<Vec<String>, String> {
//     let balatros = find_balatros();
//     Ok(balatros
//         .iter()
//         .map(|b| b.path.to_string_lossy().into_owned())
//         .collect())
// }

// Create a state structure to hold the database
struct AppState {
    db: Mutex<Database>,
}

#[tauri::command]
async fn check_existing_installation(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    if let Some(path) = db.get_installation_path().map_err(|e| e.to_string())? {
        // Verify the path still exists and is valid
        let path_buf = PathBuf::from(&path);
        if bmm_lib::balamod::Balatro::from_custom_path(path_buf).is_some() {
            return Ok(Some(path));
        } else {
            db.remove_installation_path().map_err(|e| e.to_string())?;
        }
    }
    Ok(None)
}

#[tauri::command]
async fn set_modloader(state: tauri::State<'_, AppState>, modloader: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.set_setting("current_modloader", &modloader)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_modloader(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_setting("current_modloader")
        .map_err(|e| e.to_string())
        .map(|s| s.unwrap_or_else(|| "steamodded".to_string()))
}

#[tauri::command]
async fn find_steam_balatro(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let balatros = find_balatros();
    if let Some(path) = balatros.first() {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.set_installation_path(&path.path.to_string_lossy())
            .map_err(|e| e.to_string())?;
    }

    Ok(balatros
        .iter()
        .map(|b| b.path.to_string_lossy().into_owned())
        .collect())
}

// #[tauri::command]
// async fn check_custom_balatro(path: String) -> Result<bool, String> {
//     let path = PathBuf::from(path);
//     Ok(bmm_lib::balamod::Balatro::from_custom_path(path).is_some())
// }
#[tauri::command]
async fn check_custom_balatro(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<bool, String> {
    let path = PathBuf::from(&path);
    let is_valid = bmm_lib::balamod::Balatro::from_custom_path(path.clone()).is_some();

    if is_valid {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.set_installation_path(&path.to_string_lossy())
            .map_err(|e| e.to_string())?;
    }

    Ok(is_valid)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    pretty_env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let db = Database::new().map_err(|e| e.to_string())?;
            app.manage(AppState { db: Mutex::new(db) });

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            find_steam_balatro,
            check_custom_balatro,
            check_existing_installation,
            set_modloader,
            get_modloader
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
