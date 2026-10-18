//! Tauri commands — the full backend API surface, recovered from the
//! command-name string table inside the binary.
//!
//! Argument names were recovered alongside the commands; types are inferred
//! from usage (SQL schema, JSON shapes). Adjust as needed.

use tauri::State;

// ---------------------------------------------------------------------------
// Queue
// ---------------------------------------------------------------------------

/// Add items to the download queue.
/// `items`: list of { workshop_id, app_id?, source_url?, ... }
#[tauri::command]
pub async fn add_to_queue(items: Vec<serde_json::Value>) -> Result<(), String> {
    todo!("insert into queue_items; emit workshop://queue-updated")
}

/// Remove items from the queue by their ids.
#[tauri::command]
pub async fn remove_from_queue(item_ids: Vec<String>) -> Result<(), String> {
    todo!("DELETE FROM queue_items WHERE id IN (...); emit workshop://queue-updated")
}

/// Reorder the queue.
#[tauri::command]
pub async fn reorder_queue(ordered_ids: Vec<String>) -> Result<(), String> {
    todo!("UPDATE queue_items SET position = ? WHERE id = ?")
}

/// Retry a failed/interrupted item.
#[tauri::command]
pub async fn retry_item(item_id: String) -> Result<(), String> {
    todo!("reset status to 'queued', retry_count+1")
}

/// Clear finished items from the queue list.
#[tauri::command]
pub async fn clear_completed_queue() -> Result<(), String> {
    todo!("DELETE FROM queue_items WHERE status = 'completed'")
}

/// Export queue URLs (recovered name: export_queue_urls).
#[tauri::command]
pub async fn export_queue_urls() -> Result<String, String> {
    todo!("return newline-separated source_url list")
}

// ---------------------------------------------------------------------------
// Search / history
// ---------------------------------------------------------------------------

/// Search workshop items (arg `offset` recovered; `query` inferred).
#[tauri::command]
pub async fn search(query: String, offset: Option<i64>) -> Result<serde_json::Value, String> {
    todo!("Steam GetPublishedFileDetails API")
}

/// Download history list.
#[tauri::command]
pub async fn get_history() -> Result<serde_json::Value, String> {
    todo!("SELECT FROM download_history ORDER BY completed_at DESC")
}

// ---------------------------------------------------------------------------
// SteamCMD
// ---------------------------------------------------------------------------

/// Download + install SteamCMD (from https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip).
#[tauri::command]
pub async fn install_steamcmd() -> Result<(), String> {
    todo!("download zip, extract, save path in settings")
}

/// Locate an existing SteamCMD installation.
/// (`customPath` argument name recovered — likely Option<String>.)
#[tauri::command]
pub async fn detect_steamcmd(custom_path: Option<String>) -> Result<String, String> {
    todo!("return steamcmd executable path or SteamCmdNotFound error")
}

// ---------------------------------------------------------------------------
// Settings & folders
// ---------------------------------------------------------------------------

/// Persist settings map into app_settings table.
#[tauri::command]
pub async fn save_settings(settings: serde_json::Value) -> Result<(), String> {
    todo!("INSERT OR REPLACE INTO app_settings (key, value)")
}

/// Open a folder in the OS file explorer.
#[tauri::command]
pub async fn open_folder(path: String) -> Result<(), String> {
    todo!("open::that(path)")
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

/// Resolve a Steam collection URL into its item list
/// (uses GetCollectionDetails/v1).
#[tauri::command]
pub async fn resolve_collection(url: String) -> Result<serde_json::Value, String> {
    todo!("ISteamRemoteStorage/GetCollectionDetails/v1")
}

// ---------------------------------------------------------------------------
// Diagnostics
// ---------------------------------------------------------------------------

/// Read diagnostic log rows (arg `limit` recovered).
#[tauri::command]
pub async fn get_diagnostic_logs(limit: Option<i64>) -> Result<serde_json::Value, String> {
    todo!("SELECT FROM diagnostic_logs ORDER BY id DESC LIMIT ?")
}

/// Clear diagnostic logs.
#[tauri::command]
pub async fn clear_diagnostic_logs() -> Result<(), String> {
    todo!("DELETE FROM diagnostic_logs")
}
