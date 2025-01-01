// TODO: 1. Create a way to handle the different paths for mods, which the modloader will use
// (Steamodded, lovely-only, etc.) (Also make sure for Mod-Collections!)
// TODO: 2. If the user presses "Launch",  the modloader will be called with the path to the mods
// (also implement animations for it)
// TODO: 2.1 Inject the game with the embedded binary from lovely (for steamodded)

use std::path::PathBuf;
use std::sync::Mutex;

use crate::lovely::ensure_lovely_exists;
use bmm_lib::balamod::find_balatros;
use bmm_lib::database::Database;
use bmm_lib::finder::is_steam_running;
use bmm_lib::finder::is_balatro_running;
use bmm_lib::lovely;
use std::process::Command;

use tauri::Manager;

// Create a state structure to hold the database
struct AppState {
    db: Mutex<Database>,
}

#[tauri::command]
async fn check_steam_running() -> bool {
    is_steam_running()
}

#[tauri::command]
async fn check_balatro_running() -> bool {
    is_balatro_running()
}

// Add launch command
#[tauri::command]
async fn launch_balatro(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let path_str = db
        .get_installation_path()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No Balatro installation path set".to_string())?;

    let path = PathBuf::from(&path_str);

    #[cfg(target_os = "macos")]
    {
        let lovely_path = ensure_lovely_exists()?;
        let balatro_exe = path.join("Balatro.app/Contents/MacOS/love");

        let mut command = Command::new(&balatro_exe);
        command.env("DYLD_INSERT_LIBRARIES", &lovely_path);
        command.current_dir(&path);

        command
            .spawn()
            .map_err(|e| format!("Failed to launch Balatro: {}", e))?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        let mut command = Command::new(&path);
        command
            .spawn()
            .map_err(|e| format!("Failed to launch Balatro: {}", e))?;
    }

    Ok(())
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
async fn get_balatro_path(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_installation_path().map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_balatro_path(state: tauri::State<'_, AppState>, path: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.set_installation_path(&path).map_err(|e| e.to_string())
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

            // Ensure lovely exists in config directory
            #[cfg(target_os = "macos")]
            {
                lovely::ensure_lovely_exists()
                    .map_err(|e| format!("Failed to setup lovely: {}", e))?;
            }

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
            get_modloader,
            get_balatro_path,
            set_balatro_path,
            launch_balatro,
            check_steam_running,
            check_balatro_running
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
