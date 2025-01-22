use anyhow::{Context, Result};

use dirs;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use tokio::fs as tokio_fs;
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    tag_name: String,
    name: String,
    assets: Vec<ReleaseAsset>,
    published_at: String,
    html_url: String,
    prerelease: bool,
    zipball_url: String, // Add this field
}

#[derive(Debug, Clone)]
pub enum ModType {
    Steamodded,
    Talisman,
}

impl std::fmt::Display for ModType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModType::Steamodded => write!(f, "Steamodded"),
            ModType::Talisman => write!(f, "Talisman"),
        }
    }
}

impl ModType {
    fn get_repo_url(&self) -> &str {
        match self {
            ModType::Steamodded => "Steamodded/smods",
            ModType::Talisman => "MathIsFun0/Talisman",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}
pub struct ModInstaller {
    client: reqwest::Client,
    mod_type: ModType,
}

impl ModInstaller {
    pub fn new(mod_type: ModType) -> Self {
        Self {
            client: reqwest::Client::new(),
            mod_type,
        }
    }

    fn get_installation_path(&self) -> Result<PathBuf> {
        let game_paths = crate::finder::get_balatro_paths();
        let game_path = game_paths
            .first()
            .context("Failed to find Balatro installation path. Is it installed?")?
            .to_path_buf();

        // Construct the steamodded-mods path
        let mod_path = dirs::config_dir()
            .context("Failed to get config directory")?
            .join(&game_path)
            .join("steamodded-mods");

        // dbg!(&mod_path);

        Ok(mod_path)
    }

    pub async fn get_available_versions(&self) -> Result<Vec<String>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("Balatro-Mod-Manager/1.0"),
        );
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );

        let url = format!(
            "https://api.github.com/repos/{}/releases",
            self.mod_type.get_repo_url()
        );

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .context("Failed to fetch Steamodded releases")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                status,
                error_text
            ));
        }

        let releases: Vec<Release> = response
            .json()
            .await
            .context("Failed to decode Steamodded releases")?;

        let mut versions: Vec<String> = releases
            .into_iter()
            .filter(|r| !r.prerelease) // Filter out pre-releases
            .map(|r| r.tag_name)
            .collect();

        versions.sort_by(|a, b| b.cmp(a));

        Ok(versions)
    }

    pub async fn install_version(&self, version: &str) -> Result<String> {
        let installation_path = self.get_installation_path()?;

        match self.mod_type {
            ModType::Steamodded => {
                info!(
                    "Installing Steamodded version {} to {:?}",
                    version, installation_path
                );

                let mut headers = HeaderMap::new();
                headers.insert(
                    USER_AGENT,
                    HeaderValue::from_static("Balatro-Mod-Manager/1.0"),
                );

                let url = format!(
                    "https://api.github.com/repos/{}/releases/tags/{}",
                    self.mod_type.get_repo_url(),
                    version
                );
                // Get the specific release
                let release: Release = self
                    .client
                    .get(url)
                    .headers(headers.clone())
                    .send()
                    .await?
                    .json()
                    .await?;

                info!("Downloading from {}", release.zipball_url);

                // Download the zip file directly using zipball_url
                let response = self
                    .client
                    .get(&release.zipball_url)
                    .headers(headers)
                    .send()
                    .await?;

                let bytes = response.bytes().await?;

                // Create installation directory if it doesn't exist
                tokio_fs::create_dir_all(&installation_path).await?;

                // Extract the zip
                let cursor = Cursor::new(bytes);
                let mut archive = ZipArchive::new(cursor)?;

                // Create a temporary directory for extraction
                // let temp_dir = installation_path.join("temp_steamodded");
                // if temp_dir.exists() {
                //     fs::remove_dir_all(&temp_dir)?;
                // }
                // fs::create_dir_all(&temp_dir)?;

                info!("Extracting files to temporary directory");

                // Extract all files
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let outpath = installation_path.join(file.name());

                    if file.name().ends_with('/') {
                        fs::create_dir_all(&outpath)?;
                    } else {
                        if let Some(p) = outpath.parent() {
                            fs::create_dir_all(p)?;
                        }
                        let mut outfile = fs::File::create(&outpath)?;
                        std::io::copy(&mut file, &mut outfile)?;
                    }
                }

                // Move files to final location
                // let steamodded_dir = installation_path.join("steamodded");
                // if steamodded_dir.exists() {
                //     fs::remove_dir_all(&steamodded_dir)?;
                // }
                // fs::rename(&temp_dir, installation_path)?;

                info!("Successfully installed Steamodded version {}", version);
                // dbg!(&installation_path);
            }
            ModType::Talisman => {
                let url = format!(
                    "https://github.com/MathIsFun0/Talisman/releases/download/{}/Talisman.zip",
                    version
                );

                info!("Downloading Talisman.zip from {}", url);

                // Download and extract zip logic here
                let response = self.client.get(&url).send().await?;
                let bytes = response.bytes().await?;

                // Create installation directory
                tokio_fs::create_dir_all(&installation_path).await?;

                // Extract directly to installation path
                let cursor = Cursor::new(bytes);
                let mut archive = ZipArchive::new(cursor)?;

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let outpath = installation_path.join(file.name());

                    if file.name().ends_with('/') {
                        fs::create_dir_all(&outpath)?;
                    } else {
                        if let Some(p) = outpath.parent() {
                            fs::create_dir_all(p)?;
                        }
                        let mut outfile = fs::File::create(&outpath)?;
                        std::io::copy(&mut file, &mut outfile)?;
                    }
                }
            }
        }
        Ok(installation_path.to_string_lossy().to_string())
    }
    pub async fn uninstall(&self) -> Result<()> {
        let installation_path = self.get_installation_path()?;
        let steamodded_dir = installation_path.join("steamodded");
        if steamodded_dir.exists() {
            info!("Uninstalling Steamodded from {:?}", steamodded_dir);
            tokio_fs::remove_dir_all(steamodded_dir).await?;
            info!("Successfully uninstalled Steamodded");
        } else {
            info!("Steamodded is not installed");
        }
        Ok(())
    }

    pub fn is_installed(&self) -> bool {
        match self.get_installation_path() {
            Ok(path) => path.join("steamodded").exists(),
            Err(_) => false,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_get_available_versions() -> Result<()> {
//         let installer = SteamoddedInstaller::new();
//         let versions = installer.get_available_versions().await?;
//         assert!(!versions.is_empty());
//         Ok(())
//     }
// }
