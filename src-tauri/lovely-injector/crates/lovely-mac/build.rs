fn main() {
    println!("cargo:rustc-cdylib-link-arg=-Wl,-install_name,@rpath/liblovely.dylib");
    println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,@loader_path/../Frameworks");
}
