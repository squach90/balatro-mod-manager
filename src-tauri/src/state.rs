use std::sync::Mutex;

use bmm_lib::{database::Database, discord_rpc::DiscordRpcManager};
use crate::thumb_queue::ThumbnailManager;

/// Global application state shared with Tauri commands.
pub struct AppState {
    pub db: Mutex<Database>,
    pub discord_rpc: Mutex<DiscordRpcManager>,
    pub thumbs: ThumbnailManager,
}
