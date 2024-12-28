fn main() {
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        use std::path::PathBuf;

        // First, forward the DLL
        forward_dll::forward_dll("C:\\Windows\\System32\\version.dll")
            .expect("Failed to forward version.dll");

        // Get the output directory where version.dll will be generated
        let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("target")
            .join("release");

        // Set up the destination directory in the user's config folder
        let bins_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join("Balatro")
            .join("bins");

        // Create bins directory if it doesn't exist
        fs::create_dir_all(&bins_dir).expect("Failed to create bins directory");

        // Source DLL path
        let dll_path = out_dir.join("version.dll");

        // Copy the DLL to the bins directory
        fs::copy(&dll_path, bins_dir.join("version.dll"))
            .expect("Failed to copy version.dll to bins directory");

        // Tell cargo to rerun this script if the DLL changes
        println!("cargo:rerun-if-changed={}", dll_path.display());
    }
}
