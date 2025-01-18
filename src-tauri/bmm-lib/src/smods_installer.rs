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
}

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}
#[derive(Default)]
pub struct SteamoddedInstaller {
    client: reqwest::Client,
}

impl SteamoddedInstaller {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    fn get_installation_path(&self) -> Result<PathBuf> {
        // Get the Balatro installation path
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

        let response = self
            .client
            .get("https://api.github.com/repos/Steamodded/smods/releases")
            .headers(headers)
            .send()
            .await
            .context("Failed to fetch Steamodded releases")?;

        // Check status and clone the response for error handling
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

        let mut versions: Vec<String> = releases.into_iter().map(|r| r.tag_name).collect();

        versions.sort_by(|a, b| b.cmp(a));

        Ok(versions)
    }

    pub async fn install_version(&self, version: &str) -> Result<()> {
        let installation_path = self.get_installation_path()?;
        info!(
            "Installing Steamodded version {} to {:?}",
            version, installation_path
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("Balatro-Mod-Manager/1.0"),
        );

        // Get the specific release
        let release: Release = self
            .client
            .get(format!(
                "https://api.github.com/repos/Steamodded/smods/releases/tags/{}",
                version
            ))
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;

        // Find the .zip asset
        let zip_asset = release
            .assets
            .iter()
            .find(|asset| asset.name.ends_with(".zip"))
            .context("No zip file found in release assets")?;

        info!(
            "Downloading {} from {}",
            zip_asset.name, zip_asset.browser_download_url
        );

        // Download the zip file
        let response = self
            .client
            .get(&zip_asset.browser_download_url)
            .send()
            .await?;

        let bytes = response.bytes().await?;

        // Create installation directory if it doesn't exist
        tokio_fs::create_dir_all(&installation_path).await?;

        // Extract the zip
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)?;

        // Create a temporary directory for extraction
        let temp_dir = installation_path.join("temp_steamodded");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        info!("Extracting files to temporary directory");

        // Extract all files
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = temp_dir.join(file.name());

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
        let steamodded_dir = installation_path.join("steamodded");
        if steamodded_dir.exists() {
            fs::remove_dir_all(&steamodded_dir)?;
        }
        fs::rename(&temp_dir, &steamodded_dir)?;

        info!("Successfully installed Steamodded version {}", version);
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_available_versions() -> Result<()> {
        let installer = SteamoddedInstaller::new();
        let versions = installer.get_available_versions().await?;
        assert!(!versions.is_empty());
        Ok(())
    }
}
