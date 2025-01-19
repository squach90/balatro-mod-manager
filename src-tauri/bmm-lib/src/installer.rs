use crate::finder::get_balatro_paths;
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
    let game_path = get_balatro_paths();
    let game_name: PathBuf = game_path
        .first()
        .unwrap_or_else(|| panic!("Failed to find Balatro installation path. Is it installed?"))
        .to_path_buf();

    let mod_dir = dirs::config_dir()
        .unwrap()
        .join(&game_name)
        .join("steamodded-mods");

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

        // Check if there are any files directly in the root
        let has_root_files = (0..zip.len()).any(|i| {
            let file = zip.by_index(i).unwrap();
            !file.name().contains('/')
        });

        if has_root_files {
            // Create a directory for the loose files
            let new_dir = mod_dir.join(mod_name);
            fs::create_dir_all(&new_dir).unwrap();
            installed_path = new_dir.clone();

            // Extract all files into the new directory
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
            // Files are already in directories, extract normally
            for i in 0..zip.len() {
                let mut file = zip.by_index(i).unwrap();
                let path = mod_dir.join(file.mangled_name());

                if file.is_dir() {
                    fs::create_dir_all(&path).unwrap();
                    installed_path = path;
                } else {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent).unwrap();
                        installed_path = parent.to_path_buf();
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
    match fs::remove_dir_all(path) {
        Ok(_) => log::info!("Mod uninstalled successfully"),
        Err(e) => eprintln!("Error at uninstalling mod: {}", e),
    };
    log::info!("Mod uninstalled successfully");
}
