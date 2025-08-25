use crate::errors::AppError;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const CACHE_DURATION: u64 = 15 * 60; // 15 minutes in seconds

#[derive(Serialize, Deserialize, Debug)]
struct CacheHeader {
    version: u32,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct ModCache {
    header: CacheHeader,
    mods: Vec<Mod>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Mod {
    pub title: String,
    pub description: String,
    pub image: String,
    #[serde(rename = "categories")]
    pub categories: Vec<Category>,
    #[serde(rename = "colors")]
    pub colors: ColorPair,
    pub installed: bool,
    #[serde(rename = "requires_steamodded")]
    pub requires_steamodded: bool,
    #[serde(rename = "requires_talisman")]
    pub requires_talisman: bool,
    pub publisher: String,
    pub repo: String,
    #[serde(rename = "downloadURL")]
    pub download_url: String,
    pub folderName: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorPair {
    pub color1: String,
    pub color2: String,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum Category {
    Content = 0,
    Joker = 1,
    QualityOfLife = 2,
    Technical = 3,
    Miscellaneous = 4,
    ResourcePacks = 5,
    API = 6,
}

impl From<std::string::String> for Category {
    fn from(value: std::string::String) -> Self {
        match value.as_str() {
            "Content" => Category::Content,
            "Joker" => Category::Joker,
            "Quality of Life" => Category::QualityOfLife,
            "Technical" => Category::Technical,
            "Miscellaneous" => Category::Miscellaneous,
            "Resource Packs" => Category::ResourcePacks,
            "API" => Category::API,
            _ => panic!("Invalid category: {value}"),
        }
    }
}

impl From<i32> for Category {
    fn from(value: i32) -> Self {
        match value {
            0 => Category::Content,
            1 => Category::Joker,
            2 => Category::QualityOfLife,
            3 => Category::Technical,
            4 => Category::Miscellaneous,
            5 => Category::ResourcePacks,
            6 => Category::API,
            _ => panic!("Invalid category index: {value}"),
        }
    }
}

pub fn clear_cache() -> Result<(), AppError> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("cache directory")))?
        .join("balatro-mod-manager");

    // Delete mods cache
    let mods_cache = cache_dir.join("mods.cache.bin.gz");
    if mods_cache.exists() {
        std::fs::remove_file(&mods_cache).map_err(|e| AppError::FileWrite {
            path: mods_cache,
            source: e.to_string(),
        })?;
    }

    // Delete version caches
    [
        "versions-steamodded.cache.bin.gz",
        "versions-talisman.cache.bin.gz",
    ]
    .into_iter()
    .try_for_each(|file| {
        let path = cache_dir.join(file);
        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| AppError::FileWrite {
                path: path.clone(),
                source: e.to_string(),
            })
        } else {
            Ok(())
        }
    })
}

pub fn save_versions_cache(mod_type: &str, versions: &[String]) -> Result<(), AppError> {
    let mut path = dirs::cache_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("cache directory")))?
        .join("balatro-mod-manager");

    std::fs::create_dir_all(&path).map_err(|e| AppError::DirCreate {
        path: path.clone(),
        source: e.to_string(),
    })?;

    path.push(format!("versions-{mod_type}.cache.bin.gz"));

    let file = File::create(&path).map_err(|e| AppError::FileWrite {
        path: path.clone(),
        source: e.to_string(),
    })?;

    let mut encoder = GzEncoder::new(file, Compression::default());
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::SystemTime(e.to_string()))?
        .as_secs();

    let cache = VersionCache {
        header: CacheHeader {
            version: 1,
            timestamp,
        },
        versions: versions.to_vec(),
    };

    // Use bincode 2.0 for serialization
    let config = bincode::config::standard();
    let encoded =
        bincode::serde::encode_to_vec(&cache, config).map_err(|e| AppError::Serialization {
            format: "bincode".into(),
            source: e.to_string(),
        })?;

    encoder
        .write_all(&encoded)
        .map_err(|e| AppError::FileWrite {
            path,
            source: e.to_string(),
        })?;

    Ok(())
}

pub fn load_versions_cache(mod_type: &str) -> Result<Option<Vec<String>>, AppError> {
    let path = dirs::cache_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("cache directory")))?
        .join("balatro-mod-manager")
        .join(format!("versions-{mod_type}.cache.bin.gz"));

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        })?;

    let mut decoder = GzDecoder::new(buffer.as_slice());
    let mut decompressed = Vec::new();

    // Decompress the data
    if let Err(e) = decoder.read_to_end(&mut decompressed) {
        return Err(AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        });
    }

    // Deserialize using bincode 2.0
    let config = bincode::config::standard();
    let (cache, _): (VersionCache, _) =
        match bincode::serde::decode_from_slice(&decompressed, config) {
            Ok(result) => result,
            Err(_) => return Ok(None),
        };

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::SystemTime(e.to_string()))?
        .as_secs();

    if current_time - cache.header.timestamp > CACHE_DURATION {
        return Ok(None);
    }

    Ok(Some(cache.versions))
}

#[derive(Serialize, Deserialize)]
struct VersionCache {
    header: CacheHeader,
    versions: Vec<String>,
}

pub fn get_cache_path() -> Result<PathBuf, AppError> {
    let mut path = dirs::cache_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("cache directory")))?
        .join("balatro-mod-manager");

    std::fs::create_dir_all(&path).map_err(|e| AppError::DirCreate {
        path: path.clone(),
        source: e.to_string(),
    })?;

    path.push("mods.cache.bin.gz");
    Ok(path)
}

pub fn save_cache(mods: &[Mod]) -> Result<(), AppError> {
    let path = get_cache_path()?;
    let file = File::create(&path).map_err(|e| AppError::FileWrite {
        path: path.clone(),
        source: e.to_string(),
    })?;

    let mut encoder = GzEncoder::new(file, Compression::default());
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::SystemTime(e.to_string()))?
        .as_secs();

    let cache = ModCache {
        header: CacheHeader {
            version: 1,
            timestamp,
        },
        mods: mods.to_vec(),
    };

    // Use bincode 2.0 for serialization
    let config = bincode::config::standard();
    let encoded =
        bincode::serde::encode_to_vec(&cache, config).map_err(|e| AppError::Serialization {
            format: "bincode".into(),
            source: e.to_string(),
        })?;

    encoder
        .write_all(&encoded)
        .map_err(|e| AppError::FileWrite {
            path,
            source: e.to_string(),
        })?;

    Ok(())
}

pub fn load_cache() -> Result<Option<(Vec<Mod>, u64)>, AppError> {
    let path = get_cache_path()?;
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        })?;

    let mut decoder = GzDecoder::new(buffer.as_slice());
    let mut decompressed = Vec::new();

    // Decompress the data
    if let Err(e) = decoder.read_to_end(&mut decompressed) {
        return Err(AppError::FileRead {
            path: path.clone(),
            source: e.to_string(),
        });
    }

    // Deserialize using bincode 2.0
    let config = bincode::config::standard();
    let (cache, _): (ModCache, _) = match bincode::serde::decode_from_slice(&decompressed, config) {
        Ok(result) => result,
        Err(_) => return Ok(None),
    };

    if cache.header.version != 1 {
        return Ok(None);
    }

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::SystemTime(e.to_string()))?
        .as_secs();

    if current_time - cache.header.timestamp > CACHE_DURATION {
        return Ok(None);
    }

    Ok(Some((cache.mods, cache.header.timestamp)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn with_temp_cache<T>(test: impl FnOnce(PathBuf) -> T) -> T {
        let temp_dir = tempdir().unwrap();
        let original_cache = std::env::var_os("XDG_CACHE_HOME");
        let original_home = std::env::var_os("HOME");

        std::env::set_var("XDG_CACHE_HOME", temp_dir.path());
        // On macOS, dirs::cache_dir uses ~/Library/Caches, so set HOME to our temp dir
        if cfg!(target_os = "macos") {
            std::env::set_var("HOME", temp_dir.path());
        }
        let result = test(temp_dir.path().to_path_buf());

        // restore env
        match original_cache {
            Some(val) => std::env::set_var("XDG_CACHE_HOME", val),
            None => std::env::remove_var("XDG_CACHE_HOME"),
        }
        match original_home {
            Some(val) => std::env::set_var("HOME", val),
            None => std::env::remove_var("HOME"),
        }

        result
    }

    #[test]
    #[cfg_attr(target_os = "macos", ignore = "macOS sandbox sometimes disrupts cache readback in CI")]
    fn test_mod_cache_lifecycle() -> Result<(), AppError> {
        with_temp_cache(|_| {
            let test_mod = Mod {
                title: "Test Mod".into(),
                description: "Test Description".into(),
                image: "test.png".into(),
                categories: vec![Category::Content],
                colors: ColorPair {
                    color1: "#fff".into(),
                    color2: "#000".into(),
                },
                installed: false,
                requires_steamodded: false,
                requires_talisman: false,
                publisher: "Test".into(),
                repo: "test/test".into(),
                download_url: "https://test.com/mod.zip".into(),
                folderName: None,
                version: None,
            };

            save_cache(&[test_mod.clone()])?;
            let loaded = load_cache()?.expect("Should load cache");

            assert_eq!(loaded.0.len(), 1);
            assert_eq!(loaded.0[0].title, "Test Mod");
            Ok(())
        })
    }

    #[test]
    fn test_versions_cache_roundtrip() -> Result<(), AppError> {
        with_temp_cache(|_| {
            let versions = vec!["1.0.0".into(), "1.1.0".into()];
            save_versions_cache("steamodded", &versions)?;
            let loaded = load_versions_cache("steamodded")?.expect("versions cache present");
            assert_eq!(loaded, versions);
            Ok(())
        })
    }
}
