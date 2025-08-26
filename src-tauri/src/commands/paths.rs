use std::path::PathBuf;

use crate::state::AppState;
use crate::util::map_error;
use bmm_lib::balamod::find_balatros;
use bmm_lib::errors::AppError;

#[tauri::command]
pub async fn open_directory(path: String) -> Result<(), String> {
    match open::that(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open directory: {e}")),
    }
}

#[tauri::command]
pub async fn get_mods_folder() -> Result<String, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;
    Ok(config_dir
        .join("Balatro")
        .join("Mods")
        .to_string_lossy()
        .into_owned())
}

#[tauri::command]
pub async fn get_balatro_path(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.get_installation_path())
}

#[tauri::command]
pub async fn set_balatro_path(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<(), String> {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(e) => return Err(e.to_string()),
    };
    map_error(db.set_installation_path(&path))
}

#[tauri::command]
pub async fn find_steam_balatro(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let balatros = find_balatros();
    if let Some(path) = balatros.first() {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        map_error(db.set_installation_path(&path.path.to_string_lossy()))?;
    }

    Ok(balatros
        .iter()
        .map(|b| b.path.to_string_lossy().into_owned())
        .collect())
}

#[tauri::command]
pub async fn verify_path_exists(path: String) -> bool {
    match std::fs::exists(PathBuf::from(path)) {
        Ok(exists) => exists,
        Err(e) => {
            log::error!("Failed to check path existence: {e}");
            false
        }
    }
}

#[tauri::command]
pub async fn path_exists(path: String) -> Result<bool, String> {
    let path = PathBuf::from(path);
    Ok(path.exists())
}

#[tauri::command]
pub async fn check_custom_balatro(
    state: tauri::State<'_, AppState>,
    path: String,
) -> Result<bool, String> {
    let path = PathBuf::from(&path);

    let path_to_check = if path.is_file() {
        path.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(path.clone())
    } else {
        path.clone()
    };

    let is_valid = bmm_lib::balamod::Balatro::from_custom_path(path_to_check.clone()).is_some();

    if is_valid {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        map_error(db.set_installation_path(&path_to_check.to_string_lossy()))?;
    }

    Ok(is_valid)
}

#[tauri::command]
pub async fn check_existing_installation(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    if let Some(path) = db.get_installation_path()? {
        let path_buf = PathBuf::from(&path);
        if bmm_lib::balamod::Balatro::from_custom_path(path_buf).is_some() {
            Ok(Some(path))
        } else {
            map_error(db.remove_installation_path())?;
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
