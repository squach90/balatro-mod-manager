use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThumbItem {
    pub title: String,
    pub url: String,
}

#[tauri::command]
pub async fn enqueue_thumbnails(
    items: Vec<ThumbItem>,
    state: tauri::State<'_, crate::state::AppState>,
) -> Result<u32, String> {
    let count = items.len() as u32;
    let pairs = items.into_iter().map(|i| (i.title, i.url));
    state.thumbs.enqueue_many(pairs);
    Ok(count)
}

#[tauri::command]
pub async fn enqueue_thumbnail(
    title: String,
    url: String,
    state: tauri::State<'_, crate::state::AppState>,
) -> Result<(), String> {
    state.thumbs.enqueue(title, url);
    Ok(())
}

