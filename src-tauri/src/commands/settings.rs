use crate::state::AppState;
use crate::util::map_error;
use bmm_lib::errors::AppError;

#[tauri::command]
pub async fn get_lovely_console_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.is_lovely_console_enabled())
}

#[tauri::command]
pub async fn set_lovely_console_status(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;
    map_error(db.set_lovely_console_status(enabled))
}

#[tauri::command]
pub async fn get_discord_rpc_status(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.is_discord_rpc_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_discord_rpc_status(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|_| AppError::LockPoisoned("Database lock poisoned".to_string()))?;

    db.set_discord_rpc_enabled(enabled)
        .map_err(|e| e.to_string())?;

    // update the runtime status so changes take effect immediately
    let discord_rpc = state
        .discord_rpc
        .lock()
        .map_err(|_| AppError::LockPoisoned("Discord RPC lock poisoned".to_string()))?;
    discord_rpc.set_enabled(enabled);
    Ok(())
}

#[tauri::command]
pub async fn get_background_state(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.get_background_enabled())
}

#[tauri::command]
pub async fn set_background_state(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.set_background_enabled(enabled))
}

#[tauri::command]
pub async fn is_security_warning_acknowledged(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.is_security_warning_acknowledged())
}

#[tauri::command]
pub async fn set_security_warning_acknowledged(
    state: tauri::State<'_, AppState>,
    acknowledged: bool,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    map_error(db.set_security_warning_acknowledged(acknowledged))
}
