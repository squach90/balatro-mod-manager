pub mod lovely;
pub mod cache;
pub mod settings;
pub mod repo;
pub mod system;
pub mod paths;
pub mod mods;
pub mod detection;
pub mod import;
pub mod install;

// Re-export commands so `generate_handler!` can take flat names if desired.
pub use lovely::{check_lovely_update, is_lovely_installed, update_lovely_to_latest};
pub use repo::{clone_repo, get_repo_path};
pub use system::{check_balatro_running, check_steam_running};
