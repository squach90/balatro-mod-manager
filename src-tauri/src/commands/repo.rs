use std::fs::File;
use std::path::PathBuf;

use crate::models::ModMeta;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bmm_lib::errors::AppError;
use serde::{Deserialize, Serialize};

const GITLAB_PROJECT: &str = "balatro-mod-index/repo";
const GITLAB_BASE: &str = "https://gitlab.com/balatro-mod-index/repo";
const GITLAB_RAW_MAIN: &str = "https://gitlab.com/balatro-mod-index/repo/-/raw/main";
const GITLAB_RAW_MASTER: &str = "https://gitlab.com/balatro-mod-index/repo/-/raw/master";

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
    use tokio::time::{sleep, Duration};

    // Encode path by segments so slashes remain
    let encoded: String = path
        .split('/')
        .map(urlencoding::encode)
        .map(|s| s.into_owned())
        .collect::<Vec<_>>()
        .join("/");

    let urls = [
        format!("{}/-/raw/main/{}", GITLAB_BASE, encoded),
        format!("{}/-/raw/master/{}", GITLAB_BASE, encoded),
    ];

    let client = reqwest::Client::new();
    let mut delay = Duration::from_millis(250);
    for attempt in 0..4 {
        for u in &urls {
            let resp = client.get(u).send().await.map_err(|e| e.to_string())?;
            if resp.status().is_success() {
                return resp.text().await.map_err(|e| e.to_string());
            }
            let code = resp.status().as_u16();
            // 404/410: not found — no point retrying this URL
            if code == 404 || code == 410 { continue; }
            // 429/5xx: temporary, retry after delay
        }
        if attempt < 3 { sleep(delay).await; delay = delay.saturating_mul(2); }
    }
    Err(format!("Failed to fetch {} after retries", path))
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ArchiveModItem {
    pub dir_name: String,
    pub meta: ModMeta,
    pub description: String,
    pub image_url: String,
}

// Fetch all mod metadata and descriptions via a single GitLab archive request.
// LFS images are not included (only pointer files), so we avoid downloading thumbnails here.
#[tauri::command]
pub async fn fetch_gitlab_mods_archive() -> Result<Vec<ArchiveModItem>, String> {
    use flate2::read::GzDecoder;
    use std::collections::HashMap;
    use std::io::Read;
    use tar::Archive;
    use std::time::Instant;

    // Cache location in config dir
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;
    let cache_dir = config_dir.join("Balatro").join("mod_index_cache");
    std::fs::create_dir_all(&cache_dir).map_err(|e| {
        AppError::DirCreate {
            path: cache_dir.clone(),
            source: e.to_string(),
        }
        .to_string()
    })?;
    let cache_file = cache_dir.join("mod_index_archive.json");

    #[derive(Serialize, Deserialize)]
    struct ArchiveCache {
        etag: Option<String>,
        branch: String,
        items: Vec<ArchiveModItem>,
    }

    // Try to load existing cache to get ETag and for offline fallback
    let mut existing_cache: Option<ArchiveCache> = std::fs::File::open(&cache_file)
        .ok()
        .and_then(|f| serde_json::from_reader::<_, ArchiveCache>(f).ok());

    // Build archive URL for main branch; fall back to master if needed.
    let project = urlencoding::encode(GITLAB_PROJECT);
    // Limit archive to the `mods` directory only to reduce size significantly
    let urls = [
        format!(
            "https://gitlab.com/api/v4/projects/{}/repository/archive.tar.gz?sha=main&path=mods",
            project
        ),
        format!(
            "https://gitlab.com/api/v4/projects/{}/repository/archive.tar.gz?sha=master&path=mods",
            project
        ),
    ];

    // Download archive bytes
    let mut bytes_opt: Option<Vec<u8>> = None;
    use reqwest::header::IF_NONE_MATCH;
    let client = reqwest::Client::new();
    let mut used_branch: Option<&'static str> = None;
    let mut received_etag: Option<String> = None;
    let fetch_start = Instant::now();
    for (idx, u) in urls.iter().enumerate() {
        let mut req = client.get(u.clone());
        if let Some(c) = &existing_cache {
            if let Some(et) = &c.etag {
                req = req.header(IF_NONE_MATCH, et);
            }
        }
        match req.send().await {
            Ok(resp) => {
                if resp.status().as_u16() == 304 {
                    if let Some(c) = existing_cache {
                        log::info!("GitLab archive 304 Not Modified (branch: {}), using cached items: {}", c.branch, c.items.len());
                        return Ok(c.items);
                    }
                    // No cache to use; try next URL branch without ETag
                    existing_cache = None;
                    continue;
                }
                if resp.status().is_success() {
                    if let Some(et) = resp.headers().get(reqwest::header::ETAG) {
                        if let Ok(s) = et.to_str() {
                            received_etag = Some(s.to_string());
                        }
                    }
                    match resp.bytes().await {
                        Ok(b) => {
                            bytes_opt = Some(b.to_vec());
                            used_branch = Some(if idx == 0 { "main" } else { "master" });
                            log::info!(
                                "GitLab archive downloaded: {} bytes in {} ms (branch: {})",
                                bytes_opt.as_ref().map(|v| v.len()).unwrap_or(0),
                                fetch_start.elapsed().as_millis(),
                                used_branch.unwrap_or("main")
                            );
                            break;
                        }
                        Err(e) => return Err(format!("Failed to read archive: {}", e)),
                    }
                } else {
                    log::debug!("GitLab archive status {} for {}", resp.status(), u);
                    continue;
                }
            }
            Err(e) => {
                log::debug!("GitLab archive request error for {}: {}", u, e);
                continue;
            }
        }
    }

    let bytes = match bytes_opt {
        Some(b) => b,
        None => return Err("Failed to fetch GitLab archive for main/master".into()),
    };

    // Decompress and iterate entries
    let parse_start = Instant::now();
    let gz = GzDecoder::new(std::io::Cursor::new(bytes));
    let mut archive = Archive::new(gz);

    // Collect per-mod pieces we care about
    #[derive(Default)]
    struct Parts {
        meta: Option<ModMeta>,
        desc: Option<String>,
    }
    let mut map: HashMap<String, Parts> = HashMap::new();

    for entry in archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let path = match entry.path() {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Expect paths like: <top> / mods / <dir> / (meta.json|description.md)
        let mut comps = path.components();
        match comps.next() { Some(_) => (), None => continue };
        let mods_dir = match comps.next() { Some(c) => c, None => continue };
        if mods_dir.as_os_str() != "mods" { continue; }
        let dir_os = match comps.next() { Some(c) => c.as_os_str().to_owned(), None => continue };
        let dir_name = match dir_os.to_str() { Some(s) => s.to_string(), None => continue };
        let filename = match comps.next() { Some(c) => c.as_os_str().to_string_lossy().to_string(), None => continue };

        if filename == "meta.json" {
            let mut buf = String::new();
            entry.read_to_string(&mut buf).map_err(|e| e.to_string())?;
            if let Ok(meta) = serde_json::from_str::<ModMeta>(&buf) {
                map.entry(dir_name).or_default().meta = Some(meta);
            }
        } else if filename == "description.md" {
            let mut buf = String::new();
            entry.read_to_string(&mut buf).map_err(|e| e.to_string())?;
            map.entry(dir_name).or_default().desc = Some(buf);
        } else {
            // Skip other files (including images/LFS pointers)
            continue;
        }
    }

    // Build result. Use branch-based raw URL for thumbnails (no extra requests).
    let image_branch = used_branch.unwrap_or("main"); // build URLs against the branch we fetched
    let mut out: Vec<ArchiveModItem> = Vec::with_capacity(map.len());
    for (dir, parts) in map.into_iter() {
        if let Some(meta) = parts.meta {
            let description = parts.desc.unwrap_or_default();
            let image_url = format!(
                "{}/-/raw/{}/mods/{}/thumbnail.jpg",
                GITLAB_BASE, image_branch, dir
            );
            out.push(ArchiveModItem {
                dir_name: dir,
                meta,
                description,
                image_url,
            });
        }
    }

    // Sort by title asc for stability
    out.sort_by(|a, b| a.meta.title.to_lowercase().cmp(&b.meta.title.to_lowercase()));

    log::info!(
        "Parsed {} mods from archive in {} ms",
        out.len(),
        parse_start.elapsed().as_millis()
    );

    // Save cache with ETag for future 304 validations
    let cache = ArchiveCache {
        etag: received_etag,
        branch: image_branch.to_string(),
        items: out.clone(),
    };
    if let Ok(f) = std::fs::File::create(&cache_file) {
        let _ = serde_json::to_writer_pretty(f, &cache);
    }

    Ok(out)
}

#[derive(Serialize, Deserialize)]
struct IndexFileV1 {
    version: Option<u32>,
    mods: Vec<ArchiveModItem>,
}

// Try a lightweight index.json first; fall back to archive if missing.
#[tauri::command]
pub async fn fetch_gitlab_mods() -> Result<Vec<ArchiveModItem>, String> {
    use reqwest::header::{ETAG, IF_NONE_MATCH};

    // Cache location
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")).to_string())?;
    let cache_dir = config_dir.join("Balatro").join("mod_index_cache");
    std::fs::create_dir_all(&cache_dir).map_err(|e| {
        AppError::DirCreate {
            path: cache_dir.clone(),
            source: e.to_string(),
        }
        .to_string()
    })?;
    let cache_file = cache_dir.join("index_v1.json");
    let etag_file = cache_dir.join("index_v1.etag");

    let urls = [
        format!("{}/index.json", GITLAB_RAW_MAIN),
        format!("{}/index.json", GITLAB_RAW_MASTER),
    ];

    let client = reqwest::Client::new();
    let mut etag: Option<String> = std::fs::read_to_string(&etag_file).ok();

    for url in urls {
        let mut req = client.get(&url);
        if let Some(ref v) = etag {
            req = req.header(IF_NONE_MATCH, v);
        }
        match req.send().await {
            Ok(resp) => {
                if resp.status().as_u16() == 304 {
                    if let Ok(f) = std::fs::File::open(&cache_file) {
                        if let Ok(parsed) = serde_json::from_reader::<_, IndexFileV1>(f) {
                            return Ok(parsed.mods);
                        }
                    }
                    // No cache to use; try next URL without etag
                    etag = None;
                    continue;
                }
                if resp.status().is_success() {
                    let new_etag = resp
                        .headers()
                        .get(ETAG)
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string());
                    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
                    let parsed: IndexFileV1 = serde_json::from_slice(&bytes)
                        .map_err(|e| format!("Failed to parse index.json: {}", e))?;
                    // Save cache and etag
                    if let Ok(f) = std::fs::File::create(&cache_file) {
                        let _ = serde_json::to_writer_pretty(f, &parsed);
                    }
                    if let Some(et) = new_etag {
                        let _ = std::fs::write(&etag_file, et);
                    }
                    return Ok(parsed.mods);
                }
            }
            Err(_) => continue,
        }
    }

    // Fallback: meta-only (fast, multi-request) then archive (single, heavy)
    match fetch_gitlab_mods_meta_only().await {
        Ok(items) if !items.is_empty() => Ok(items),
        _ => fetch_gitlab_mods_archive().await,
    }
}

#[tauri::command]
pub async fn fetch_gitlab_mods_meta_only() -> Result<Vec<ArchiveModItem>, String> {
    use std::time::Instant;
    let t0 = Instant::now();
    // 1) List mod directories via GitLab API (main, then master)
    #[derive(Deserialize)]
    struct GitLabTreeEntry { name: String, r#type: String }

    let client = reqwest::Client::new();
    let project = urlencoding::encode(GITLAB_PROJECT);
    let mut branch_used: Option<&'static str> = None;
    let mut mod_dirs: Vec<String> = Vec::new();
    for (i, branch) in ["main", "master"].iter().enumerate() {
        let url = format!(
            "https://gitlab.com/api/v4/projects/{}/repository/tree?path=mods&ref={}&per_page=500",
            project, branch
        );
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let entries: Vec<GitLabTreeEntry> = resp.json().await.map_err(|e| e.to_string())?;
                mod_dirs = entries.into_iter().filter(|e| e.r#type == "tree").map(|e| e.name).collect();
                branch_used = Some(if i == 0 { "main" } else { "master" });
                break;
            }
            _ => continue,
        }
    }
    if mod_dirs.is_empty() {
        return Err("No mod directories found from GitLab API".into());
    }
    let image_branch = branch_used.unwrap_or("main");

    // 2) Concurrently fetch only meta.json for each mod (description is deferred)
    use futures::{stream, StreamExt};
    let concurrency = 12usize;
    let results = stream::iter(mod_dirs.into_iter())
        .map(|dir| {
            let client = client.clone();
            let dir_clone = dir.clone();
            async move {
                // Try main then master for meta.json
                let paths = [
                    format!("{}/mods/{}/meta.json", GITLAB_RAW_MAIN, urlencoding::encode(&dir_clone)),
                    format!("{}/mods/{}/meta.json", GITLAB_RAW_MASTER, urlencoding::encode(&dir_clone)),
                ];
                for url in &paths {
                    if let Ok(resp) = client.get(url).send().await {
                        if resp.status().is_success() {
                            if let Ok(text) = resp.text().await {
                                if let Ok(meta) = serde_json::from_str::<ModMeta>(&text) {
                                    let image_url = format!(
                                        "{}/mods/{}/thumbnail.jpg",
                                        if image_branch == "main" { GITLAB_RAW_MAIN } else { GITLAB_RAW_MASTER },
                                        urlencoding::encode(&dir_clone)
                                    );
                                    return Some(ArchiveModItem {
                                        dir_name: dir_clone,
                                        meta,
                                        description: String::new(), // defer heavy descriptions
                                        image_url,
                                    });
                                }
                            }
                        }
                    }
                }
                None
            }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;

    let mut items: Vec<ArchiveModItem> = results.into_iter().flatten().collect();
    items.sort_by(|a, b| a.meta.title.to_lowercase().cmp(&b.meta.title.to_lowercase()));
    log::info!(
        "Fetched meta for {} mods via API in {} ms",
        items.len(),
        t0.elapsed().as_millis()
    );
    Ok(items)
}

fn safe_slug(input: &str) -> String {
    let mut s = input.trim().to_lowercase();
    s = s
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    while s.contains("--") {
        s = s.replace("--", "-");
    }
    s.trim_matches('-').to_string()
}

fn ensure_assets_dirs() -> Result<(std::path::PathBuf, std::path::PathBuf), String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::DirNotFound(std::path::PathBuf::from("config directory")).to_string())?;
    let base = config_dir.join("Balatro").join("mod_assets");
    let thumbs = base.join("thumbnails");
    let descs = base.join("descriptions");
    std::fs::create_dir_all(&thumbs).map_err(|e| AppError::DirCreate { path: thumbs.clone(), source: e.to_string() }.to_string())?;
    std::fs::create_dir_all(&descs).map_err(|e| AppError::DirCreate { path: descs.clone(), source: e.to_string() }.to_string())?;
    Ok((thumbs, descs))
}

#[tauri::command]
pub async fn get_cached_installed_thumbnail(
    title: String,
    dir_name: String,
    state: tauri::State<'_, crate::state::AppState>,
) -> Result<Option<String>, String> {
    let installed = {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.get_installed_mods()
            .map_err(|e| e.to_string())?
            .into_iter()
            .any(|m| m.name.eq_ignore_ascii_case(&title))
    };
    if !installed {
        return Ok(None);
    }

    let (thumbs_dir, _) = ensure_assets_dirs()?;
    let slug = safe_slug(&title);
    let path = thumbs_dir.join(format!("{slug}.jpg"));
    if path.exists() {
        let data = std::fs::read(&path).map_err(|e| AppError::FileRead { path: path.clone(), source: e.to_string() }.to_string())?;
        let b64 = STANDARD.encode(data);
        return Ok(Some(format!("data:image/jpeg;base64,{b64}")));
    }

    // Not cached yet: try to download from GitLab raw and store.
    let client = reqwest::Client::new();
    let enc = urlencoding::encode(&dir_name);
    let candidates = [
        format!("{}/mods/{}/thumbnail.jpg", GITLAB_RAW_MAIN, enc),
        format!("{}/mods/{}/thumbnail.jpg", GITLAB_RAW_MASTER, enc),
    ];
    for url in &candidates {
        if let Ok(resp) = client.get(url).send().await {
            if resp.status().is_success() {
                if let Ok(bytes) = resp.bytes().await {
                    // Persist and return
                    std::fs::write(&path, &bytes).map_err(|e| AppError::FileWrite { path: path.clone(), source: e.to_string() }.to_string())?;
                    let b64 = STANDARD.encode(&bytes);
                    return Ok(Some(format!("data:image/jpeg;base64,{b64}")));
                }
            }
        }
    }
    Ok(None)
}

#[tauri::command]
pub async fn get_description_cached_or_remote(
    title: String,
    dir_name: String,
    state: tauri::State<'_, crate::state::AppState>,
) -> Result<String, String> {
    let installed = {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        db.get_installed_mods()
            .map_err(|e| e.to_string())?
            .into_iter()
            .any(|m| m.name.eq_ignore_ascii_case(&title))
    };

    let (_, descs_dir) = ensure_assets_dirs()?;
    let slug = safe_slug(&title);
    let path = descs_dir.join(format!("{slug}.md"));

    if installed && path.exists() {
        return std::fs::read_to_string(&path).map_err(|e| AppError::FileRead { path, source: e.to_string() }.to_string());
    }

    // Fetch from GitLab raw (main then master)
    let client = reqwest::Client::new();
    let enc = urlencoding::encode(&dir_name);
    let candidates = [
        format!("{}/mods/{}/description.md", GITLAB_RAW_MAIN, enc),
        format!("{}/mods/{}/description.md", GITLAB_RAW_MASTER, enc),
    ];
    for url in &candidates {
        if let Ok(resp) = client.get(url).send().await {
            if resp.status().is_success() {
                if let Ok(text) = resp.text().await {
                    if installed {
                        // Store for offline use
                        if let Err(e) = std::fs::write(&path, &text) {
                            log::warn!("Failed to cache description for {}: {}", title, e);
                        }
                    }
                    return Ok(text);
                }
            }
        }
    }
    Err(format!("Description not found for {}", dir_name))
}
