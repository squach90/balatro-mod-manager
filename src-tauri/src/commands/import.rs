use std::io::Cursor;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use tar::Archive;
use zip::ZipArchive;

use bmm_lib::errors::AppError;

#[tauri::command]
pub async fn process_dropped_file(path: String) -> Result<String, String> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| "Could not find config directory".to_string())?;
    let mods_dir = config_dir.join("Balatro").join("Mods");
    std::fs::create_dir_all(&mods_dir).map_err(|e| format!("Failed to create mods directory: {e}"))?;

    let file_path = std::path::Path::new(&path);
    let file_name = file_path
        .file_name()
        .ok_or_else(|| "Invalid file path".to_string())?
        .to_str()
        .ok_or_else(|| "Invalid file name".to_string())?;

    let mut file = std::fs::File::open(file_path)
        .map_err(|e| format!("Failed to open file: {e}"))?;
    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut file, &mut buffer)
        .map_err(|e| format!("Failed to read file: {e}"))?;

    process_mod_archive(file_name.to_string(), buffer).await
}

#[tauri::command]
pub async fn process_mod_archive(filename: String, data: Vec<u8>) -> Result<String, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;
    let mods_dir = config_dir.join("Balatro").join("Mods");
    std::fs::create_dir_all(&mods_dir)
        .map_err(|e| format!("Failed to create mods directory: {e}"))?;

    let mod_dir_name = filename
        .trim_end_matches(".zip")
        .trim_end_matches(".tar.gz")
        .trim_end_matches(".tgz")
        .trim_end_matches(".tar")
        .to_string();
    let mod_dir = mods_dir.join(&mod_dir_name);

    if mod_dir.exists() {
        std::fs::remove_dir_all(&mod_dir)
            .map_err(|e| format!("Failed to remove existing mod directory: {e}"))?;
    }

    let cursor = Cursor::new(data);
    if filename.ends_with(".zip") {
        extract_zip_from_memory(cursor, &mod_dir)?;
    } else if filename.ends_with(".tar") {
        extract_tar_from_memory(cursor, &mod_dir)?;
    } else if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
        extract_tar_gz_from_memory(cursor, &mod_dir)?;
    } else {
        return Err("Unsupported file format. Only ZIP, TAR, and TAR.GZ are supported.".to_string());
    }

    if let Ok(entries) = std::fs::read_dir(&mod_dir) {
        let dirs: Vec<_> = entries
            .filter_map(Result::ok)
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .collect();
        if dirs.len() == 1 && std::fs::read_dir(&mod_dir).map(|e| e.count()).unwrap_or(0) == 1 {
            let nested_dir = dirs[0].path();
            for entry in std::fs::read_dir(&nested_dir)
                .map_err(|e| format!("Failed to read nested directory: {e}"))?
            {
                let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
                let target_path = mod_dir.join(entry.file_name());
                if entry.file_type().map_err(|e| format!("Failed to get file type: {e}"))?.is_dir() {
                    std::fs::rename(entry.path(), &target_path)
                        .map_err(|e| format!("Failed to move directory: {e}"))?;
                } else {
                    std::fs::rename(entry.path(), &target_path)
                        .map_err(|e| format!("Failed to move file: {e}"))?;
                }
            }
            std::fs::remove_dir_all(&nested_dir)
                .map_err(|e| format!("Failed to remove nested directory: {e}"))?;
        }
    }

    Ok(mod_dir.to_string_lossy().to_string())
}

fn extract_zip_from_memory(cursor: Cursor<Vec<u8>>, target_dir: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create target directory: {e}"))?;
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("Failed to open ZIP archive: {e}"))?;
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {e}"))?;
        if file.name().starts_with("__MACOSX/") {
            continue;
        }
        let file_path = match file.enclosed_name() { Some(path) => path.to_owned(), None => continue };
        let output_path = target_dir.join(&file_path);
        if file.is_dir() {
            std::fs::create_dir_all(&output_path)
                .map_err(|e| format!("Failed to create directory: {e}"))?;
        } else {
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {e}"))?;
            }
            let mut outfile = std::fs::File::create(&output_path)
                .map_err(|e| format!("Failed to create file {}: {e}", output_path.display()))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to write file {}: {e}", output_path.display()))?;
        }
    }
    Ok(())
}

fn extract_tar_from_memory(cursor: Cursor<Vec<u8>>, target_dir: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create target directory: {e}"))?;
    let mut archive = Archive::new(cursor);
    for entry in archive
        .entries()
        .map_err(|e| format!("Failed to read TAR entries: {e}"))?
    {
        let mut entry = entry.map_err(|e| format!("Failed to read TAR entry: {e}"))?;
        let path = entry.path().map_err(|e| format!("Failed to get entry path: {e}"))?;
        let output_path = target_dir.join(path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {e}"))?;
        }
        entry
            .unpack(&output_path)
            .map_err(|e| format!("Failed to unpack file {}: {e}", output_path.display()))?;
    }
    Ok(())
}

fn extract_tar_gz_from_memory(cursor: Cursor<Vec<u8>>, target_dir: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(target_dir)
        .map_err(|e| format!("Failed to create target directory: {e}"))?;
    let gz = GzDecoder::new(cursor);
    let mut archive = Archive::new(gz);
    for entry in archive
        .entries()
        .map_err(|e| format!("Failed to read TAR entries: {e}"))?
    {
        let mut entry = entry.map_err(|e| format!("Failed to read TAR entry: {e}"))?;
        let path = entry.path().map_err(|e| format!("Failed to get entry path: {e}"))?;
        let output_path = target_dir.join(path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory: {e}"))?;
        }
        entry
            .unpack(&output_path)
            .map_err(|e| format!("Failed to unpack file {}: {e}", output_path.display()))?;
    }
    Ok(())
}
