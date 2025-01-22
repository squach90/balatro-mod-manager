use reqwest::Client;
use std::fs;
use std::io;
use std::io::Cursor;
use std::path::PathBuf;

pub async fn install_mod(url: String) -> Result<PathBuf, String> {
    let client = Client::new();
    let response = client.get(&url).send().await.unwrap();
    let file = response.bytes().await.unwrap();
    let file_type = infer::get(&file).unwrap().mime_type();
    // let game_path = get_balatro_paths();
    // let game_name: PathBuf = game_path
    //     .first()
    //     .unwrap_or_else(|| panic!("Failed to find Balatro installation path. Is it installed?"))
    //     .to_path_buf();

    let mod_dir = dirs::config_dir().unwrap().join("Balatro").join("Mods");

    let mod_name = url
        .split('/')
        .last()
        .and_then(|s| s.split('.').next())
        .unwrap_or("unknown_mod");

    log::info!("Installing mod: {}", url);

    let mut installed_path = mod_dir.clone();
    if file_type == "application/zip" {
        log::info!("Installing zip file");
        let cursor = Cursor::new(file);
        let mut zip = zip::ZipArchive::new(cursor).unwrap();

        // Check for root files
        let has_root_files = (0..zip.len()).any(|i| {
            let file = zip.by_index(i).unwrap();
            !file.name().contains('/')
        });

        if has_root_files {
            // Handle loose files in root
            let new_dir = mod_dir.join(mod_name);
            fs::create_dir_all(&new_dir).unwrap();
            installed_path = new_dir.clone();

            for i in 0..zip.len() {
                let mut file = zip.by_index(i).unwrap();
                let path = new_dir.join(file.name());

                if file.is_dir() {
                    fs::create_dir_all(&path).unwrap();
                } else {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    let mut output = fs::File::create(&path).unwrap();
                    io::copy(&mut file, &mut output).unwrap();
                }
            }
        } else {
            // Handle structured zip files
            let root_dir = {
                let first_entry = zip.by_index(0).unwrap();
                let name_parts: Vec<&str> = first_entry.name().split('/').collect();
                name_parts[0].to_string()
            };
            installed_path = mod_dir.join(&root_dir);

            // Extract all files
            for i in 0..zip.len() {
                let mut file = zip.by_index(i).unwrap();
                let path = mod_dir.join(file.mangled_name());

                if file.is_dir() {
                    fs::create_dir_all(&path).unwrap();
                } else {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    let mut output = fs::File::create(&path).unwrap();
                    io::copy(&mut file, &mut output).unwrap();
                }
            }
        }
    } else if file_type == "application/x-tar" {
        log::info!("Installing tar file");
        let cursor = Cursor::new(file);
        let mut tar = tar::Archive::new(cursor);

        // Get first entry to determine root path
        if let Ok(mut entries) = tar.entries() {
            if let Some(Ok(first_entry)) = entries.next() {
                installed_path = mod_dir.join(first_entry.path().unwrap());
                if installed_path.is_file() {
                    installed_path = installed_path.parent().unwrap_or(&mod_dir).to_path_buf();
                }
            }
        }

        for entry in tar.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = mod_dir.join(entry.path().unwrap());

            if entry.header().entry_type().is_dir() {
                fs::create_dir_all(&path).unwrap();
            } else {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut output = fs::File::create(&path).unwrap();
                io::copy(&mut entry, &mut output).unwrap();
            }
        }
    } else if file_type == "application/gzip" {
        log::info!("Installing tar.gz file");
        let cursor = Cursor::new(file);
        let gz = flate2::read::GzDecoder::new(cursor);
        let mut tar = tar::Archive::new(gz);

        // Get first entry to determine root path
        if let Ok(mut entries) = tar.entries() {
            if let Some(Ok(first_entry)) = entries.next() {
                installed_path = mod_dir.join(first_entry.path().unwrap());
                if installed_path.is_file() {
                    installed_path = installed_path.parent().unwrap_or(&mod_dir).to_path_buf();
                }
            }
        }

        for entry in tar.entries().unwrap() {
            let mut entry = entry.unwrap();
            let path = mod_dir.join(entry.path().unwrap());

            if entry.header().entry_type().is_dir() {
                fs::create_dir_all(&path).unwrap();
            } else {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut output = fs::File::create(&path).unwrap();
                io::copy(&mut entry, &mut output).unwrap();
            }
        }
    }

    log::info!("Mod installed successfully at: {:?}", installed_path);
    Ok(installed_path)
}


pub fn uninstall_mod(path: PathBuf) {
    log::info!("Uninstalling mod: {:?}", path);
    
    let mods_dir = dirs::config_dir()
        .expect("Config dir")
        .join("Balatro")
        .join("Mods");

    // Enhanced safety checks
    if !path.exists() {
        log::error!("Path doesn't exist: {:?}", path);
        return;
    }

    if path == mods_dir {
        log::error!("Blocked attempt to delete Mods directory");
        return;
    }

    if !path.starts_with(&mods_dir) {
        log::error!("Path outside Mods directory: {:?}", path);
        return;
    }

    // Special handling for Steamodded patterns
    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
        if dir_name.starts_with("Steamodded-smods-") {
            log::info!("Uninstalling Steamodded variant: {}", dir_name);
        }
    }

    match fs::remove_dir_all(&path) {
        Ok(_) => log::info!("Successfully removed mod"),
        Err(e) => log::error!("Failed to remove mod: {}", e),
    }
}
