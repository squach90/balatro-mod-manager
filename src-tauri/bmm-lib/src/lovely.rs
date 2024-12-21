// bmm-lib/src/lovely.rs
use std::path::PathBuf;
use std::fs;
use libloading::Library;

pub fn ensure_lovely_exists() -> Result<PathBuf, String> {
    #[cfg(target_os = "macos")] {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| "Could not find config directory".to_string())?;
        let bins_dir = config_dir.join("Balatro").join("bins");
        fs::create_dir_all(&bins_dir).map_err(|e| e.to_string())?;
        
        let lovely_path = bins_dir.join("liblovely.dylib");
        
        // Verify lovely exists and is loadable
        if lovely_path.exists() {
            // Try loading the library to verify it's valid
            unsafe {
                Library::new(&lovely_path)
                    .map_err(|e| format!("Failed to load lovely: {}", e))?;
            }
        } else {
            return Err("Lovely binary not found. Please install it first.".to_string());
        }
        
        Ok(lovely_path)
    }
    
    #[cfg(not(target_os = "macos"))] {
        Err("Lovely is only supported on macOS".to_string())
    }
}

