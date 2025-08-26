use std::path::PathBuf;
use std::process::Command;

use crate::state::AppState;
use crate::util::map_error;
use bmm_lib::errors::AppError;
use bmm_lib::lovely;
use bmm_lib::smods_installer::{ModInstaller, ModType};
use bmm_lib::{cache, database::InstalledMod};

#[tauri::command]
pub async fn launch_balatro(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let (path_str, lovely_console_enabled) = {
        let db = state
            .db
            .lock()
            .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;

        (
            db.get_installation_path()?
                .ok_or_else(|| AppError::InvalidState("No installation path set".to_string()))?,
            db.is_lovely_console_enabled()?,
        )
    };

    let path = PathBuf::from(path_str);

    #[cfg(target_os = "macos")]
    {
        let lovely_path = map_error(lovely::ensure_lovely_exists().await)?;
        let balatro_executable = path.join("Balatro.app/Contents/MacOS/love");

        if lovely_console_enabled {
            let disable_arg = if !lovely_console_enabled { " --disable-console" } else { "" };
            let command_line = format!(
                "cd '{}' && DYLD_INSERT_LIBRARIES='{}' '{}'{}",
                path.display(),
                lovely_path.display(),
                balatro_executable.display(),
                disable_arg
            );

            let applescript = format!(
                "tell application \"Terminal\" to do script \"{command_line}\"",
            );

            Command::new("osascript")
                .arg("-e")
                .arg(applescript)
                .status()
                .map_err(|e| e.to_string())?;
        } else {
            let cmd = format!(
                "DYLD_INSERT_LIBRARIES='{}' '{}'",
                lovely_path.display(),
                balatro_executable.display()
            );
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .status()
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        Command::new(path.join("Balatro.exe"))
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[tauri::command]
pub async fn get_steamodded_versions() -> Result<Vec<String>, String> {
    let installer = ModInstaller::new(ModType::Steamodded);
    installer
        .get_available_versions()
        .await
        .map(|versions| versions.into_iter().map(|v| v.to_string()).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_steamodded_version(version: String) -> Result<String, String> {
    let installer = ModInstaller::new(ModType::Steamodded);
    installer
        .install_version(&version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_talisman_versions() -> Result<Vec<String>, String> {
    let installer = ModInstaller::new(ModType::Talisman);
    installer
        .get_available_versions()
        .await
        .map(|versions| versions.into_iter().map(|v| v.to_string()).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_latest_steamodded_release() -> Result<String, String> {
    if let Ok(Some(versions)) = cache::load_versions_cache("steamodded") {
        if !versions.is_empty() {
            let version = &versions[0];
            return Ok(format!(
                "https://github.com/Steamodded/smods/archive/refs/tags/{version}.zip"
            ));
        }
    }

    let installer = ModInstaller::new(ModType::Steamodded);
    installer
        .get_latest_release()
        .await
        .map(|version| match installer.mod_type {
            ModType::Steamodded => {
                format!("https://github.com/Steamodded/smods/archive/refs/tags/{version}.zip")
            }
            _ => format!("https://github.com/Steamodded/smods/archive/refs/tags/{version}.zip"),
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_talisman_version(version: String) -> Result<String, String> {
    let installer = ModInstaller::new(ModType::Talisman);
    installer
        .install_version(&version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_dependents(mod_name: String) -> Result<Vec<String>, String> {
    let db = bmm_lib::database::Database::new().map_err(|e| e.to_string())?;
    let all_dependents = db.get_dependents(&mod_name).map_err(|e| e.to_string())?;
    let filtered: Vec<String> = all_dependents.into_iter().filter(|d| d != &mod_name).collect();
    Ok(filtered)
}

#[tauri::command]
pub async fn cascade_uninstall(
    state: tauri::State<'_, AppState>,
    root_mod: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut to_uninstall = vec![root_mod.clone()];
    let mut processed = std::collections::HashSet::new();

    while let Some(current) = to_uninstall.pop() {
        if processed.contains(&current) {
            continue;
        }
        processed.insert(current.clone());

        let mod_details = map_error(db.get_mod_details(&current))?;
        let dependents = map_error(db.get_dependents(&current))?;
        to_uninstall.extend(dependents);

        map_error(bmm_lib::installer::uninstall_mod(PathBuf::from(mod_details.path)))?;
        map_error(db.remove_installed_mod(&current))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn force_remove_mod(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
) -> Result<(), String> {
    map_error(bmm_lib::installer::uninstall_mod(PathBuf::from(path)))?;
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.remove_installed_mod(&name))
}

#[tauri::command]
pub async fn remove_installed_mod(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    let is_framework = name.to_lowercase() == "steamodded" || name.to_lowercase() == "talisman";
    if is_framework {
        let all_dependents = map_error(db.get_dependents(&name))?;
        let real_deps: Vec<String> = all_dependents.into_iter().filter(|d| d != &name).collect();
        if !real_deps.is_empty() {
            return Err(format!(
                "Use cascade_uninstall to remove {} with {} dependents",
                name,
                real_deps.len()
            ));
        }
    }

    map_error(bmm_lib::installer::uninstall_mod(PathBuf::from(path)))?;
    map_error(db.remove_installed_mod(&name))
}

#[tauri::command]
pub async fn install_mod(url: String, folder_name: String) -> Result<PathBuf, String> {
    let folder_name = if folder_name.is_empty() { None } else { Some(folder_name) };
    map_error(bmm_lib::installer::install_mod(url, folder_name).await)
}

#[tauri::command]
pub async fn get_installed_mods_from_db(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<InstalledMod>, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.get_installed_mods())
}

#[tauri::command]
pub async fn add_installed_mod(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
    dependencies: Vec<String>,
    current_version: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let current_version = if current_version.is_empty() {
        None
    } else {
        Some(current_version)
    };
    map_error(db.add_installed_mod(&name, &path, &dependencies, current_version))
}

