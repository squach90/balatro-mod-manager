#[cfg(target_os = "macos")]
use std::env;
#[cfg(target_os = "macos")]
use std::fs;
#[cfg(target_os = "macos")]
use std::path::PathBuf;

fn main() {
    #[cfg(target_os = "macos")]
    {
        if env::var("SKIP_BUILD_SCRIPT").unwrap_or_else(|_| "0".to_string()) == "1" {
            return;
        }

        // Get the manifest directory for the current build script.
        // If it is running in a subcrate (e.g. "src-tauri/bmm-lib"), move one level up.
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let top_level = if manifest_dir.ends_with("bmm-lib") {
            // Assuming "src-tauri/bmm-lib", set top_level = "src-tauri"
            manifest_dir
                .parent()
                .expect("expected parent directory")
                .to_path_buf()
        } else {
            manifest_dir.clone()
        };

        // Destination directory for the dylib inside the Tauri project (src-tauri/bundled_dylibs)
        let dest_dir = top_level.join("bundled_dylibs");
        fs::create_dir_all(&dest_dir)
            .expect("Failed to create bundled_dylibs directory in project root (src-tauri)");

        // Path to the built dylib from lovely-mac (assumes lovely-mac is located at "../lovely-injector")
        let lovely_mac_dir = top_level.join("lovely-injector");
        let dylib_src = lovely_mac_dir.join("target/release/liblovely.dylib");
        if !dylib_src.exists() {
            panic!(
                "Dylib not found at {}. Did you build lovely-mac?",
                dylib_src.display()
            );
        }

        // Copy the dylib into the destination directory ("src-tauri/bundled_dylibs")
        let dylib_dest = dest_dir.join("liblovely.dylib");
        fs::copy(&dylib_src, &dylib_dest)
            .expect("Failed to copy liblovely.dylib to bundled_dylibs directory");

        println!(
            "cargo:info=Copied dylib from {} to {}",
            dylib_src.display(),
            dylib_dest.display()
        );

        // (Adjust this if necessary to match your runtime linking strategy.)
        // println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Resources/bundled_dylibs");

        // Trigger rebuild when the source dylib changes.
        println!("cargo:rerun-if-changed={}", dylib_src.display());
    }
}
