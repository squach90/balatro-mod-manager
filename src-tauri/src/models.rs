use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Payload {
    pub args: Vec<String>,
    pub cwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModMeta {
    #[serde(rename = "requires-steamodded")]
    pub requires_steamodded: bool,
    #[serde(rename = "requires-talisman")]
    pub requires_talisman: bool,
    pub categories: Vec<String>,
    pub author: String,
    pub repo: String,
    pub title: String,
    #[serde(rename = "downloadURL")]
    pub download_url: Option<String>,
    #[serde(rename = "folderName", default)]
    pub folder_name: String,
    #[serde(default)]
    pub version: String,
    #[serde(rename = "automatic-version-check", default)]
    pub automatic_version_check: bool,
    #[serde(rename = "last-updated", default)]
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModCacheInfo {
    pub path: String,
    pub last_commit: i64,
}
