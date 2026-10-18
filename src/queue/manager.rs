//! Download queue manager (src/queue/manager.rs in the original).
//!
//! Evidence from the binary:
//! - emits Tauri events: workshop://queue-updated, workshop://queue-started,
//!   workshop://queue-stopped, workshop://item-completed, workshop://progress
//! - startup recovery: "transitioned N interrupted items to 'Interrupted'"
//!   (items left in 'downloading' when the app closed get marked interrupted)
//! - drives steamcmd for the actual downloads

use serde::{Deserialize, Serialize};

/// Mirrors the `queue_items` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String, // uuid
    pub workshop_id: i64,
    pub app_id: Option<i64>,
    pub app_name: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub preview_url: Option<String>,
    pub item_type: String, // "workshop_item"
    pub file_size_bytes: Option<i64>,
    pub downloaded_bytes: i64,
    pub progress_percent: Option<f64>,
    pub status: QueueStatus,
    pub status_message: Option<String>,
    pub source_url: String,
    pub collection_id: Option<i64>,
    pub collection_title: Option<String>,
    pub output_path: Option<String>,
    pub position: i64,
    pub retry_count: i64,
    pub max_retries: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueueStatus {
    Queued,
    Downloading,
    Completed,
    Failed,
    Interrupted,
}

pub struct QueueManager {
    // db pool, steamcmd handle, event emitter...
}

impl QueueManager {
    /// On startup: UPDATE queue_items SET status='interrupted'
    /// WHERE status='downloading'.
    pub async fn recover_interrupted(&self) -> anyhow::Result<usize> {
        todo!()
    }

    /// Process the queue sequentially, emitting progress events.
    pub async fn run(&self) -> anyhow::Result<()> {
        todo!("for each queued item: steamcmd download, update progress, emit events")
    }
}
