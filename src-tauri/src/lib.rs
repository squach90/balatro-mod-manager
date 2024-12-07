use bmm_lib::balamod::find_balatros;
use tauri::Manager;

#[tauri::command]
async fn find_steam_balatro() -> Result<Vec<String>, String> {
    let balatros = find_balatros();
    Ok(balatros
        .iter()
        .map(|b| b.path.to_string_lossy().into_owned())
        .collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    pretty_env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![find_steam_balatro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
