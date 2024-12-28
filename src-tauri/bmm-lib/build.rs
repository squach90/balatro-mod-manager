// use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    if std::env::var("SKIP_BUILD_SCRIPT").unwrap_or("0".to_string()) == "1" {
        return;
    }
    #[cfg(target_os = "macos")]
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lovely_mac_dir = PathBuf::from(&manifest_dir).join("../lovely-injector");
        let lovely_bin_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join("Balatro")
            .join("bins");
        //
        // // // Build lovely-mac first
        // let status = Command::new("cargo")
        //     .args(["build", "--release"])
        //     .current_dir(lovely_mac_dir.join("crates/lovely-mac"))
        //     .status()
        //     .expect("Failed to build lovely-mac");
        //
        // if !status.success() {
        //     panic!("Failed to build lovely-mac");
        // }
        //
        // Get the correct dylib path
        let dylib_path = lovely_mac_dir.join("target/release/liblovely.dylib");

        if !dylib_path.exists() {
            panic!("Dylib not found at expected path: {}", dylib_path.display());
        }
        //
        // // Generate the Rust code with the dylib path
        // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        // let code = format!(
        //     "pub const LOVELY_DYLIB: &[u8] = include_bytes!({:?});",
        //     dylib_path
        // );
        //
        // std::fs::write(out_dir.join("lovely_dylib.rs"), code)
        //     .expect("Failed to write generated code");

        // move liblove.dylib to the correct location (config/Balatro/bins)
        std::fs::copy(&dylib_path, lovely_bin_dir.join("liblovely.dylib"))
            .expect("Failed to copy liblovely.dylib to bins directory");

        println!("cargo:rerun-if-changed={}", dylib_path.display());
    }

    #[cfg(target_os = "windows")]
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        // Adjust the path to your Windows-specific injector code if desired
        let injector_dir = PathBuf::from(&manifest_dir).join("../lovely-injector");

        // Where we'll place the compiled DLL (in this case named version.dll)
        let output_bin_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join("Balatro")
            .join("bins");

        // Optionally build the crate that will produce version.dll
        // Make sure your Cargo.toml for that crate has [lib] name = "version"
        // and crate-type = ["cdylib"] or ["dylib"] so it compiles to version.dll.
        /*
        let status = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(injector_dir.join("crates/version-crate"))
            .status()
            .expect("Failed to build version-crate");

        if !status.success() {
            panic!("Failed to build version-crate");
        }
        */

        // Point to the resulting version.dll
        let dll_path = injector_dir.join("target").join("release").join("version.dll");

        if !dll_path.exists() {
            panic!("DLL not found at the expected path: {}", dll_path.display());
        }

        // Copy version.dll to your bins directory
        std::fs::copy(&dll_path, output_bin_dir.join("version.dll"))
            .expect("Failed to copy version.dll to bins directory");

        println!("cargo:rerun-if-changed={}", dll_path.display());
    }
}
