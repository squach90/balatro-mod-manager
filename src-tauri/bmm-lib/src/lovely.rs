use crate::errors::AppError;
use std::fs::{self, File};
#[cfg(target_os = "macos")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_os = "macos")]
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub const EMBEDDED_DLL: &[u8] = include_bytes!("../../resources/version.dll");

fn detect_architecture() -> Result<&'static str, AppError> {
    use libc::{c_void, size_t, sysctl};

    let mut size: size_t = 0;
    let mut mib = [libc::CTL_HW, libc::HW_MACHINE];

    // First call to get buffer size
    unsafe {
        if sysctl(
            mib.as_mut_ptr(),
            2,
            std::ptr::null_mut(),
            &mut size,
            std::ptr::null_mut(),
            0,
        ) != 0
        {
            return Err(AppError::SystemDetection(
                "Failed to get architecture buffer size".into(),
            ));
        }
    }

    let mut buf = vec![0u8; size];

    // Second call to get actual value
    unsafe {
        if sysctl(
            mib.as_mut_ptr(),
            2,
            buf.as_mut_ptr() as *mut c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        ) != 0
        {
            return Err(AppError::SystemDetection(
                "Failed to retrieve architecture".into(),
            ));
        }
    }

    // Convert buffer to string and match architecture
    match String::from_utf8_lossy(&buf).trim_end_matches('\0') {
        "arm64" => Ok("aarch64"),
        "x86_64" => Ok("x86_64"),
        other => Err(AppError::UnsupportedArchitecture(other.into())),
    }
}

pub async fn ensure_lovely_exists() -> Result<PathBuf, AppError> {
    #[cfg(target_os = "macos")]
    {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::DirNotFound(PathBuf::from("config directory")))?;

        let bins_dir = config_dir.join("Balatro/bins");
        fs::create_dir_all(&bins_dir).map_err(|e| AppError::DirCreate {
            path: bins_dir.clone(),
            source: e.to_string(),
        })?;

        let lovely_path = bins_dir.join("liblovely.dylib");

        if !lovely_path.exists() {
            download_and_install_lovely(&lovely_path).await?;
        }

        Ok(lovely_path)
    }

    #[cfg(target_os = "windows")]
    {
        let balatro_paths = crate::finder::get_balatro_paths();
        if balatro_paths.is_empty() {
            return Err(AppError::DirNotFound(PathBuf::from("Balatro installation")));
        }
        Ok(balatro_paths[0].join("Balatro.exe"))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(AppError::InvalidState(
            "Lovely injection is not supported on this platform.".into(),
        ))
    }
}

#[cfg(target_os = "macos")]
async fn download_and_install_lovely(target_path: &Path) -> Result<(), AppError> {
    let temp_dir = tempfile::tempdir().map_err(|e| AppError::FileWrite {
        path: PathBuf::from("temp directory"),
        source: e.to_string(),
    })?;

    let arch = detect_architecture()?;
    let url = format!(
        "https://github.com/ethangreen-dev/lovely-injector/releases/latest/download/\
    lovely-{}-apple-darwin.tar.gz",
        arch
    );

    // Download latest release
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;

    // Save to temp file
    let temp_tar_gz = temp_dir.path().join("lovely.tar.gz");
    let mut file = File::create(&temp_tar_gz).map_err(|e| AppError::FileWrite {
        path: temp_tar_gz.clone(),
        source: e.to_string(),
    })?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| AppError::Network(e.to_string()))?;
    std::io::copy(&mut bytes.as_ref(), &mut file).map_err(|e| AppError::FileWrite {
        path: temp_tar_gz.clone(),
        source: e.to_string(),
    })?;

    // Extract and install
    let tar_gz = File::open(&temp_tar_gz).map_err(|e| AppError::FileRead {
        path: temp_tar_gz.clone(),
        source: e.to_string(),
    })?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);

    archive.unpack(&temp_dir).map_err(|e| AppError::FileRead {
        path: temp_tar_gz.clone(),
        source: e.to_string(),
    })?;

    // Find the library in extracted files
    let extracted_lib = temp_dir.path().join("liblovely.dylib");
    fs::copy(&extracted_lib, target_path).map_err(|e| AppError::FileCopy {
        source: extracted_lib.display().to_string(),
        dest: target_path.display().to_string(),
        source_error: e.to_string(),
    })?;

    // Set permissions
    std::fs::set_permissions(target_path, std::fs::Permissions::from_mode(0o755))?;

    Ok(())
}
