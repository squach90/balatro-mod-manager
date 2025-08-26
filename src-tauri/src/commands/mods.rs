use std::path::{Path, PathBuf};

use crate::state::AppState;
use bmm_lib::errors::AppError;
use rayon::prelude::*;
use std::fs;

#[tauri::command]
pub async fn is_mod_enabled(
    state: tauri::State<'_, AppState>,
    mod_name: String,
) -> Result<bool, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    let installed_mods = db.get_installed_mods()?;
    let mod_dir = &installed_mods
        .iter()
        .find(|m| m.name == mod_name)
        .ok_or_else(|| format!("Mod not found: {mod_name}"))?
        .path
        .clone();
    let mod_dir: &Path = Path::new(mod_dir);

    if !mod_dir.exists() {
        return Err(format!("Mod directory not found: {mod_name}"));
    }

    let ignore_file_path = mod_dir.join(".lovelyignore");
    Ok(!ignore_file_path.exists())
}

#[tauri::command]
pub async fn toggle_mod_enabled(
    state: tauri::State<'_, AppState>,
    mod_name: String,
    enabled: bool,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    let installed_mods = db.get_installed_mods()?;
    let mod_dir = &installed_mods
        .iter()
        .find(|m| m.name == mod_name)
        .ok_or_else(|| format!("Mod not found: {mod_name}"))?
        .path
        .clone();
    let mod_dir: &Path = Path::new(mod_dir);

    if !mod_dir.exists() {
        return Err(format!("Mod directory not found: {mod_name}"));
    }

    let entries: Vec<_> = fs::read_dir(mod_dir)
        .map_err(|e| format!("Failed to read mod directory: {e}"))?
        .collect::<Result<_, _>>()
        .map_err(|e| format!("Failed to read entry: {e}"))?;

    let ignore_file_path = mod_dir.join(".lovelyignore");

    if enabled {
        entries
            .par_iter()
            .filter(|entry| entry.path().is_dir())
            .try_for_each(|entry| {
                let ignore_path = entry.path().join(".lovelyignore");
                if ignore_path.exists() {
                    fs::remove_file(&ignore_path).map_err(|e| {
                        format!(
                            "Failed to remove .lovelyignore in {}: {}",
                            entry.path().display(),
                            e
                        )
                    })
                } else {
                    Ok(())
                }
            })?;

        if ignore_file_path.exists() {
            fs::remove_file(&ignore_file_path)
                .map_err(|e| format!("Failed to remove top-level .lovelyignore: {e}"))?;
        }
    } else {
        entries
            .par_iter()
            .filter(|entry| entry.path().is_dir())
            .try_for_each(|entry| {
                fs::write(entry.path().join(".lovelyignore"), "").map_err(|e| {
                    format!(
                        "Failed to create .lovelyignore in {}: {}",
                        entry.path().display(),
                        e
                    )
                })
            })?;

        fs::write(&ignore_file_path, "")
            .map_err(|e| format!("Failed to create top-level .lovelyignore: {e}"))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn is_mod_enabled_by_path(mod_path: String) -> Result<bool, String> {
    let path = PathBuf::from(&mod_path);
    if !path.exists() {
        return Err(format!("Mod path does not exist: {mod_path}"));
    }
    let ignore_file_path = path.join(".lovelyignore");
    Ok(!ignore_file_path.exists())
}

#[tauri::command]
pub async fn toggle_mod_enabled_by_path(mod_path: String, enabled: bool) -> Result<(), String> {
    let path = PathBuf::from(&mod_path);
    if !path.exists() {
        return Err(format!("Mod path does not exist: {mod_path}"));
    }

    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Failed to read mod directory: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect directory entries: {e}"))?;

    let ignore_file_path = path.join(".lovelyignore");

    if enabled {
        entries
            .par_iter()
            .filter(|entry| entry.path().is_dir())
            .try_for_each(|entry| {
                let subdir_ignore = entry.path().join(".lovelyignore");
                if subdir_ignore.exists() {
                    fs::remove_file(&subdir_ignore).map_err(|e| {
                        format!(
                            "Failed to remove .lovelyignore in {}: {}",
                            entry.path().display(),
                            e
                        )
                    })
                } else {
                    Ok(())
                }
            })?;

        if ignore_file_path.exists() {
            fs::remove_file(&ignore_file_path)
                .map_err(|e| format!("Failed to remove .lovelyignore file: {e}"))?;
        }
    } else {
        entries
            .par_iter()
            .filter(|entry| entry.path().is_dir())
            .try_for_each(|entry| {
                fs::write(entry.path().join(".lovelyignore"), "").map_err(|e| {
                    format!(
                        "Failed to create .lovelyignore in {}: {}",
                        entry.path().display(),
                        e
                    )
                })
            })?;

        fs::write(&ignore_file_path, "")
            .map_err(|e| format!("Failed to create .lovelyignore file: {e}"))?;
    }

    Ok(())
}

