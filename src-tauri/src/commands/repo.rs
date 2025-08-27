use std::fs::File;
use std::path::PathBuf;

use crate::models::ModMeta;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bmm_lib::errors::AppError;
use serde::Deserialize;

const GITLAB_PROJECT: &str = "balatro-mod-index/repo";
const GITLAB_BASE: &str = "https://gitlab.com/balatro-mod-index/repo";

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

#[derive(Deserialize)]
struct GitLabTreeEntry {
    name: String,
    #[allow(dead_code)]
    r#type: String,
}

#[tauri::command]
pub async fn list_gitlab_mods() -> Result<Vec<String>, String> {
    let project = urlencoding::encode(GITLAB_PROJECT);
    let url = format!(
        "https://gitlab.com/api/v4/projects/{}/repository/tree?path=mods&ref=main&per_page=500",
        project
    );
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("GitLab API error: {}", e))?;
    if !resp.status().is_success() {
        return Err(format!("GitLab API status: {}", resp.status()));
    }
    let entries: Vec<GitLabTreeEntry> = resp
        .json()
        .await
        .map_err(|e| format!("Parse GitLab tree failed: {}", e))?;
    Ok(entries
        .into_iter()
        .filter(|e| e.r#type == "tree")
        .map(|e| e.name)
        .collect())
}

#[tauri::command]
pub async fn get_gitlab_file(path: &str) -> Result<String, String> {
    // Encode path by segments so slashes remain
    let encoded: String = path
        .split('/')
        .map(urlencoding::encode)
        .map(|s| s.into_owned())
        .collect::<Vec<_>>()
        .join("/");

    let u1 = format!("{}/-/raw/main/{}", GITLAB_BASE, encoded);
    let u2 = format!("{}/-/raw/master/{}", GITLAB_BASE, encoded);

    let mut resp = reqwest::get(&u1).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        resp = reqwest::get(&u2).await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("Failed to fetch {}: {}", path, resp.status()));
        }
    }
    resp.text().await.map_err(|e| e.to_string())
}

#[allow(non_snake_case)]
#[tauri::command]
pub async fn get_gitlab_thumbnail_url(dirName: String) -> Result<Option<String>, String> {
    // Try unencoded then encoded, on main then master
    let enc = urlencoding::encode(&dirName);
    let candidates = [
        format!("{}/-/raw/main/mods/{}/thumbnail.jpg", GITLAB_BASE, dirName),
        format!("{}/-/raw/main/mods/{}/thumbnail.jpg", GITLAB_BASE, enc),
        format!("{}/-/raw/master/mods/{}/thumbnail.jpg", GITLAB_BASE, dirName),
        format!("{}/-/raw/master/mods/{}/thumbnail.jpg", GITLAB_BASE, enc),
    ];

    use reqwest::header::RANGE;
    let client = reqwest::Client::new();
    for url in candidates {
        let resp = client
            .get(&url)
            .header(RANGE, "bytes=0-0")
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.status().is_success() {
            return Ok(Some(url));
        }
    }
    Ok(None)
}
