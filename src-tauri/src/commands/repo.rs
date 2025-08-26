use std::fs::File;
use std::path::PathBuf;

use crate::models::ModMeta;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bmm_lib::errors::AppError;

#[tauri::command]
pub async fn get_repo_path() -> Result<String, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;
    let repo_path = config_dir.join("Balatro").join("mod_index");
    Ok(repo_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn clone_repo(url: &str, path: &str) -> Result<(), String> {
    crate::github_repo::clone_repository(url, path).await
}

#[allow(non_snake_case)]
#[tauri::command]
pub async fn get_mod_thumbnail(modPath: String) -> Result<Option<String>, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;

    let full_path = config_dir
        .join("Balatro")
        .join("mod_index")
        .join("mods")
        .join(modPath)
        .join("thumbnail.jpg");

    let image_data = match std::fs::read(&full_path) {
        Ok(data) => data,
        Err(_) => return Ok(None),
    };

    let base64 = STANDARD.encode(image_data);
    Ok(Some(format!("data:image/jpeg;base64,{base64}")))
}

#[tauri::command]
pub async fn pull_repo(path: &str) -> Result<(), String> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(format!("Directory '{path}' does not exist"));
    }

    if !crate::github_repo::is_repository_directory(path) {
        let repo_url = "https://github.com/skyline69/balatro-mod-index";
        return crate::github_repo::clone_repository(repo_url, path).await;
    }

    crate::github_repo::pull_repository(path).await
}

#[tauri::command]
pub async fn list_directories(path: &str) -> Result<Vec<String>, String> {
    let dir = PathBuf::from(path);
    let entries = std::fs::read_dir(dir).map_err(|e| {
        AppError::FileRead {
            path: PathBuf::from(path),
            source: e.to_string(),
        }
        .to_string()
    })?;

    let mut dirs = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| {
            AppError::FileRead {
                path: PathBuf::from(path),
                source: e.to_string(),
            }
            .to_string()
        })?;

        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    dirs.push(name.to_string());
                }
            }
        }
    }
    Ok(dirs)
}

#[tauri::command]
pub async fn read_json_file(path: &str) -> Result<ModMeta, String> {
    let path = PathBuf::from(path);
    let file = File::open(&path).map_err(|e| {
        AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        }
        .to_string()
    })?;

    serde_json::from_reader(file).map_err(|e| {
        AppError::JsonParse {
            path,
            source: e.to_string(),
        }
        .to_string()
    })
}

#[tauri::command]
pub async fn read_text_file(path: &str) -> Result<String, String> {
    let path = PathBuf::from(path);
    std::fs::read_to_string(&path).map_err(|e| {
        AppError::FileRead {
            path,
            source: e.to_string(),
        }
        .to_string()
    })
}
