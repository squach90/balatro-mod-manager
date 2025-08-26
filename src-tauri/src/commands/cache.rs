use std::time::{SystemTime, UNIX_EPOCH};

use bmm_lib::cache;
use bmm_lib::cache::Mod;

use crate::state::AppState;
use crate::util::map_error;

#[tauri::command]
pub async fn save_versions_cache(mod_type: String, versions: Vec<String>) -> Result<(), String> {
    map_error(cache::save_versions_cache(&mod_type, &versions))
}

#[tauri::command]
pub async fn load_versions_cache(mod_type: String) -> Result<Option<(Vec<String>, u64)>, String> {
    cache::load_versions_cache(&mod_type)
        .map(|res| {
            res.map(|versions| {
                (
                    versions,
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                )
            })
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_mods_cache(mods: Vec<Mod>) -> Result<(), String> {
    map_error(cache::save_cache(&mods))
}

#[tauri::command]
pub async fn load_mods_cache() -> Result<Option<(Vec<Mod>, u64)>, String> {
    map_error(cache::load_cache())
}

#[tauri::command]
pub async fn clear_cache() -> Result<(), String> {
    map_error(cache::clear_cache())
}

#[tauri::command]
pub async fn get_last_fetched(state: tauri::State<'_, AppState>) -> Result<u64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_last_fetched().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_last_fetched(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.set_last_fetched(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mod_update_available(
    mod_name: String,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let last_installed_version = db
        .get_last_installed_version(&mod_name)
        .map_err(|e| e.to_string())?;

    if last_installed_version.is_empty() {
        return Ok(false);
    }

    let cached_mods = match cache::load_cache().map_err(|e| e.to_string())? {
        Some((mods, _)) => mods,
        None => return Ok(false),
    };

    for cached_mod in cached_mods {
        if cached_mod.title == mod_name || (cached_mod.folderName.as_ref() == Some(&mod_name)) {
            if let Some(remote_version) = cached_mod.version {
                return Ok(remote_version != last_installed_version);
            }
            break;
        }
    }

    Ok(false)
}
