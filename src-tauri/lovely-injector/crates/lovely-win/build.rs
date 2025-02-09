#[cfg(target_os = "windows")]
use std::env;
#[cfg(target_os = "windows")]
use std::fs;
#[cfg(target_os = "windows")]
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "windows")]
    {
        if env::var("SKIP_BUILD_SCRIPT").unwrap_or_else(|_| "0".to_string()) == "1" {
            return;
        }
        // First, forward the DLL
        forward_dll::forward_dll("C:\\Windows\\System32\\version.dll")
            .expect("Failed to forward version.dll");

        // The DLL will be generated in the target directory
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let target_dir = PathBuf::from(&manifest_dir).join("target").join("release");

        // Set up the destination directory
        let bins_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join("Balatro")
            .join("bins");

        // Create all necessary directories
        fs::create_dir_all(&bins_dir).expect("Failed to create Balatro bins directory");

        // Source DLL path
        let dll_path = target_dir.join("version.dll");

        // Print some debug information
        println!("cargo:warning=DLL source path: {}", dll_path.display());
        println!("cargo:warning=Bins directory: {}", bins_dir.display());

        // Only try to copy if the DLL exists
        if dll_path.exists() {
            match fs::copy(&dll_path, bins_dir.join("version.dll")) {
                Ok(_) => {
                    println!("cargo:warning=Successfully copied version.dll to bins directory")
                }
                Err(e) => println!("cargo:warning=Failed to copy version.dll: {}", e),
            }
        } else {
            println!(
                "cargo:warning=version.dll not found at: {}",
                dll_path.display()
            );
            // Don't panic here as the DLL might not exist yet during the build process
        }

        // Tell cargo to rerun this script if the DLL changes
        println!("cargo:rerun-if-changed={}", dll_path.display());
    }
}
