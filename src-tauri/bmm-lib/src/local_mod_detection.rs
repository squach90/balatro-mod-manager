use crate::database::{Database, InstalledMod};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedMod {
    pub name: String,
    pub id: String,
    pub author: Vec<String>,
    pub description: String,
    pub prefix: String,
    pub version: Option<String>,
    pub path: String,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub is_tracked: bool,
}

pub fn detect_local_mods() -> Result<Vec<DetectedMod>, String> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| "Could not find config directory".to_string())?;

    let mod_dir = config_dir.join("Balatro").join("Mods");

    if !mod_dir.exists() {
        return Ok(Vec::new());
    }

    let mut detected_mods = Vec::new();
    let mut bundled_dependencies = HashSet::new();

    // Step 1: First scan to find bundled dependencies in the Mods subdirectory of any mod
    find_bundled_dependencies(&mod_dir, &mut bundled_dependencies)?;

    // Step 2: Do the actual mod detection, ignoring bundled dependencies
    detect_mods_recursive(&mod_dir, &mut detected_mods, &bundled_dependencies)?;

    Ok(detected_mods)
}

fn find_bundled_dependencies(dir: &Path, bundled_deps: &mut HashSet<String>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        // Skip lovely-related directories
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            let lower_name = file_name.to_lowercase();
            if lower_name.contains("lovely") {
                continue;
            }
        }

        // Check if this directory contains a "Mods" subdirectory
        let mods_subdir = path.join("Mods");
        if mods_subdir.exists() && mods_subdir.is_dir() {
            // This is likely a mod package with bundled dependencies
            // Mark all mods in the Mods subdirectory as bundled dependencies
            mark_bundled_dependencies(&mods_subdir, bundled_deps)?;
        }

        // Recursively check subdirectories (limited depth)
        let depth = count_path_depth(&path, dir);
        if depth <= 3 {
            // Increase depth to make sure we find nested packages
            find_bundled_dependencies(&path, bundled_deps)?;
        }
    }

    Ok(())
}

/// Mark all mods in a Mods subdirectory as bundled dependencies
fn mark_bundled_dependencies(
    mods_dir: &Path,
    bundled_deps: &mut HashSet<String>,
) -> Result<(), String> {
    let entries = fs::read_dir(mods_dir)
        .map_err(|e| format!("Failed to read directory {}: {}", mods_dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Add this dependency's normalized path to our set
            let normalized_path = normalize_path(&path);
            bundled_deps.insert(normalized_path);

            // Log for debugging
            log::debug!("Found bundled dependency: {}", path.display());
        }
    }

    Ok(())
}

/// Recursively scan for mods in directories
fn detect_mods_recursive(
    dir: &Path,
    detected_mods: &mut Vec<DetectedMod>,
    bundled_deps: &HashSet<String>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        // Skip lovely-related directories
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            let lower_name = file_name.to_lowercase();
            if lower_name.contains("lovely") {
                continue;
            }
        }

        // Skip bundled dependencies
        let normalized_path = normalize_path(&path);
        if bundled_deps.contains(&normalized_path) {
            log::debug!("Skipping bundled dependency: {}", path.display());
            continue;
        }

        // Check if this directory is a mod
        if let Some(detected_mod) = detect_mod_in_directory(&path)? {
            detected_mods.push(detected_mod);
            continue;
        }

        // If this is a "Mods" directory, recursively scan it
        if path.file_name().and_then(|n| n.to_str()) == Some("Mods") {
            detect_mods_recursive(&path, detected_mods, bundled_deps)?;
            continue;
        }

        // Regular directory, recursively scan up to 2 levels deep
        let depth = count_path_depth(&path, dir);
        if depth <= 2 {
            detect_mods_recursive(&path, detected_mods, bundled_deps)?;
        }
    }

    Ok(())
}

/// Normalize path for case-insensitive comparison on Windows
fn normalize_path(path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        path.to_string_lossy().to_lowercase()
    }
    #[cfg(not(target_os = "windows"))]
    {
        path.to_string_lossy().to_string()
    }
}

fn count_path_depth(path: &Path, base_path: &Path) -> usize {
    let path_str = path.to_string_lossy();
    let base_str = base_path.to_string_lossy();

    if !path_str.starts_with(&*base_str) {
        return 0;
    }

    let relative = &path_str[base_str.len()..];
    relative
        .chars()
        .filter(|&c| c == std::path::MAIN_SEPARATOR)
        .count()
}

fn detect_mod_in_directory(mod_path: &Path) -> Result<Option<DetectedMod>, String> {
    // Get directory name
    let dir_name = mod_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("Invalid directory name: {}", mod_path.display()))?;

    // First check for JSON configuration with the same name as the directory
    let json_path = mod_path.join(format!("{}.json", dir_name));

    if json_path.exists() {
        if let Some(detected_mod) = parse_mod_json(&json_path, mod_path)? {
            return Ok(Some(detected_mod));
        }
    }

    // Check for smods.json
    let smods_json_path = mod_path.join("smods.json");
    if smods_json_path.exists() {
        if let Some(detected_mod) = parse_mod_json(&smods_json_path, mod_path)? {
            return Ok(Some(detected_mod));
        }
    }

    // Look for any Lua file with the same name as the directory
    let lua_path = mod_path.join(format!("{}.lua", dir_name));
    if lua_path.exists() {
        if let Some(detected_mod) = parse_mod_lua_header(&lua_path, mod_path)? {
            return Ok(Some(detected_mod));
        }
    }

    // Special handling for mod packages that have a structure like:
    // ModName/Mods/ModName/ModName.lua
    let potential_mod_dir = mod_path.join("Mods").join(dir_name);
    let potential_lua_path = potential_mod_dir.join(format!("{}.lua", dir_name));

    if potential_lua_path.exists() {
        if let Some(detected_mod) = parse_mod_lua_header(&potential_lua_path, mod_path)? {
            return Ok(Some(detected_mod));
        }
    }

    // If we have a Mods directory with content, this might be a mod package
    let mods_dir = mod_path.join("Mods");
    if mods_dir.exists() && mods_dir.is_dir() {
        // Look for a README.md or similar to infer the mod name
        let readme_path = mod_path.join("README.md");
        let readme_alt_path = mod_path.join("README.MD");

        if readme_path.exists() || readme_alt_path.exists() {
            // This looks like a mod package - create a mod entry for it
            return Ok(Some(DetectedMod {
                name: dir_name.to_string(),
                id: dir_name.replace(" ", ""),
                author: vec!["Unknown".to_string()],
                description: format!("Mod package found in {}", mod_path.display()),
                prefix: if dir_name.len() >= 4 {
                    dir_name[0..4].to_lowercase()
                } else {
                    dir_name.to_lowercase()
                },
                version: None,
                path: mod_path.to_string_lossy().to_string(),
                dependencies: Vec::new(),
                conflicts: Vec::new(),
                is_tracked: false,
            }));
        }
    }

    // If no direct match found, check all Lua files in the directory
    for entry in fs::read_dir(mod_path)
        .map_err(|e| format!("Failed to read mod directory {}: {}", mod_path.display(), e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("lua") {
            if let Some(detected_mod) = parse_mod_lua_header(&path, mod_path)? {
                return Ok(Some(detected_mod));
            }
        }
    }

    // No mod configuration found
    Ok(None)
}

/// JSON schema for mod configuration
#[derive(Debug, Serialize, Deserialize)]
struct ModJson {
    id: String,
    name: String,
    author: Vec<String>,
    description: String,
    prefix: String,
    main_file: String,
    #[serde(default)]
    priority: i32,
    #[serde(default = "default_badge_color")]
    badge_colour: String,
    #[serde(default = "default_text_color")]
    badge_text_colour: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    conflicts: Vec<String>,
    #[serde(default)]
    provides: Vec<String>,
    #[serde(default)]
    dump_loc: bool,
}

fn default_badge_color() -> String {
    "666665".to_string()
}

fn default_text_color() -> String {
    "FFFFFF".to_string()
}

/// Parse mod info from JSON file
fn parse_mod_json(json_path: &Path, mod_path: &Path) -> Result<Option<DetectedMod>, String> {
    let file = match File::open(json_path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open JSON file {}: {}", json_path.display(), e);
            return Ok(None);
        }
    };

    let mod_json: ModJson = match serde_json::from_reader(file) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Failed to parse JSON file {}: {}", json_path.display(), e);
            return Ok(None);
        }
    };

    // Check if ID is valid (not one of the disallowed values)
    let disallowed_ids = ["Steamodded", "Lovely", "Balatro"];
    if disallowed_ids.contains(&mod_json.id.as_str()) {
        log::info!("Mod {} has a disallowed ID: {}", mod_json.name, mod_json.id);
        return Ok(None);
    }

    Ok(Some(DetectedMod {
        name: mod_json.name,
        id: mod_json.id,
        author: mod_json.author,
        description: mod_json.description,
        prefix: mod_json.prefix,
        version: mod_json.version,
        path: mod_path.to_string_lossy().to_string(),
        dependencies: mod_json.dependencies,
        conflicts: mod_json.conflicts,
        is_tracked: false,
    }))
}

fn parse_mod_lua_header(lua_path: &Path, mod_path: &Path) -> Result<Option<DetectedMod>, String> {
    let file = match File::open(lua_path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open Lua file {}: {}", lua_path.display(), e);
            return Ok(None);
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .take(20) // Only check first 20 lines for efficiency
        .map(|line| line.map_err(|e| format!("Failed to read line: {}", e)))
        .collect::<Result<Vec<String>, String>>()?;

    if lines.is_empty() {
        return Ok(None);
    }

    // Check if any line has the header marker
    let has_header = lines
        .iter()
        .any(|line| line.trim() == "--- STEAMODDED HEADER");
    if !has_header {
        // Try to infer mod info from filename if no header
        if let Some(mod_name) = mod_path.file_name().and_then(|n| n.to_str()) {
            // Simple inference based on directory name
            return Ok(Some(DetectedMod {
                name: mod_name.to_string(),
                id: mod_name.to_string().replace(" ", ""),
                author: vec!["Unknown".to_string()],
                description: format!("Local mod found in {}", mod_path.display()),
                prefix: if mod_name.len() >= 4 {
                    mod_name[0..4].to_lowercase()
                } else {
                    mod_name.to_lowercase()
                },
                version: None,
                path: mod_path.to_string_lossy().to_string(),
                dependencies: Vec::new(),
                conflicts: Vec::new(),
                is_tracked: false,
            }));
        }
        return Ok(None);
    }

    // Parse the rest as before...
    let mut name = String::new();
    let mut id = String::new();
    let mut author = Vec::new();
    let mut description = String::new();
    let mut prefix = String::new();
    let mut version = None;
    let mut dependencies = Vec::new();
    let mut conflicts = Vec::new();

    // Parse the header lines
    for line in &lines {
        let line = line.trim();
        if !line.starts_with("---") {
            continue;
        }

        let line = &line[3..].trim();

        if let Some(value) = line.strip_prefix("MOD_NAME:") {
            name = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("MOD_ID:") {
            id = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("MOD_AUTHOR:") {
            // Parse author list [Author1, Author2, ...]
            if let Some(author_str) = value
                .trim()
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
            {
                author = author_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
        } else if let Some(value) = line.strip_prefix("MOD_DESCRIPTION:") {
            description = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("PREFIX:") {
            prefix = value.trim().to_string();
        } else if let Some(value) = line.strip_prefix("VERSION:") {
            version = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("DEPENDENCIES:") {
            // Parse dependencies list
            if let Some(deps_str) = value
                .trim()
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
            {
                dependencies = deps_str.split(',').map(|s| s.trim().to_string()).collect();
            }
        } else if let Some(value) = line.strip_prefix("CONFLICTS:") {
            // Parse conflicts list
            if let Some(conf_str) = value
                .trim()
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
            {
                conflicts = conf_str.split(',').map(|s| s.trim().to_string()).collect();
            }
        }
    }

    // If we couldn't find required fields, try to infer from the directory/file name
    if name.is_empty() {
        if let Some(file_name) = lua_path.file_stem().and_then(|s| s.to_str()) {
            name = file_name.to_string();
        }
    }

    if id.is_empty() {
        if let Some(file_name) = lua_path.file_stem().and_then(|s| s.to_str()) {
            id = file_name.replace(" ", "");
        }
    }

    if author.is_empty() {
        author.push("Unknown".to_string());
    }

    if description.is_empty() {
        description = format!("Local mod found in {}", mod_path.display());
    }

    // If prefix is empty, use first 4 letters of ID
    if prefix.is_empty() && !id.is_empty() {
        if id.len() >= 4 {
            prefix = id[0..4].to_lowercase();
        } else {
            prefix = id.to_lowercase();
        }
    }

    Ok(Some(DetectedMod {
        name,
        id,
        author,
        description,
        prefix,
        version,
        path: mod_path.to_string_lossy().to_string(),
        dependencies,
        conflicts,
        is_tracked: false,
    }))
}

/// Get all detected mods and mark which ones are tracked in the database
pub fn get_all_detected_mods(db: &Database) -> Result<Vec<DetectedMod>, String> {
    let detected_mods = detect_local_mods()?;
    let installed_mods = db
        .get_installed_mods()
        .map_err(|e| format!("Failed to get installed mods: {}", e))?;

    // Mark each mod with its tracking status
    let result = detected_mods
        .into_iter()
        .map(|mut mod_info| {
            mod_info.is_tracked = is_mod_tracked(&mod_info, &installed_mods);
            mod_info
        })
        .collect();

    Ok(result)
}

/// Checks which detected mods are not already tracked in the database
pub fn get_untracked_mods(db: &Database) -> Result<Vec<DetectedMod>, String> {
    let all_mods = get_all_detected_mods(db)?;

    // Filter out mods that are already tracked in the database
    Ok(all_mods
        .into_iter()
        .filter(|mod_info| !mod_info.is_tracked)
        .collect())
}

/// Helper function to check if a mod is tracked in the database
fn is_mod_tracked(detected_mod: &DetectedMod, installed_mods: &[InstalledMod]) -> bool {
    let detected_id = detected_mod.id.to_lowercase();
    let detected_name = detected_mod.name.to_lowercase();

    for installed_mod in installed_mods {
        // Check for name match (case-insensitive)
        if installed_mod.name.to_lowercase() == detected_name {
            return true;
        }

        // Check for ID match in path (case-insensitive)
        if installed_mod.path.to_lowercase().contains(&detected_id) {
            return true;
        }

        // Check for exact path match
        #[cfg(target_os = "windows")]
        {
            if installed_mod.path.to_lowercase() == detected_mod.path.to_lowercase() {
                return true;
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if installed_mod.path == detected_mod.path {
                return true;
            }
        }
    }

    false
}

/// Register a detected mod in the database
pub fn register_detected_mod(db: &Database, detected_mod: &DetectedMod) -> Result<(), String> {
    db.add_installed_mod(
        &detected_mod.name,
        &detected_mod.path,
        &detected_mod.dependencies,
        detected_mod.version.clone(),
    )
    .map_err(|e| format!("Failed to add detected mod to database: {}", e))
}
