use bmm_lib::errors::AppError;

/// Map library `AppError` to a string for Tauri command results.
pub fn map_error<T>(result: Result<T, AppError>) -> Result<T, String> {
    result.map_err(|e| e.to_string())
}
