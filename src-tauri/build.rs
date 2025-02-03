fn main() {
    tauri_build::build();
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Resources/bundled_dylibs");
}
