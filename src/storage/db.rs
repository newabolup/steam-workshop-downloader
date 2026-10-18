//! SQLite storage — full schema recovered VERBATIM from the binary.
//! Database file: workshop_downloader.sqlite3

pub const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS queue_items (
    id TEXT PRIMARY KEY,
    workshop_id INTEGER NOT NULL,
    app_id INTEGER,
    app_name TEXT,
    title TEXT NOT NULL,
    description TEXT,
    author TEXT,
    preview_url TEXT,
    item_type TEXT NOT NULL,          -- 'workshop_item'
    file_size_bytes INTEGER,
    downloaded_bytes INTEGER NOT NULL DEFAULT 0,
    progress_percent REAL,
    status TEXT NOT NULL,             -- queued/downloading/completed/failed/interrupted
    status_message TEXT,
    source_url TEXT NOT NULL,
    collection_id INTEGER,
    collection_title TEXT,
    output_path TEXT,
    position INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 0  -- (truncated in binary; inferred)
);

CREATE INDEX IF NOT EXISTS idx_queue_items_workshop_id ON queue_items(workshop_id);

CREATE TABLE IF NOT EXISTS download_history (
    id TEXT PRIMARY KEY,
    workshop_id INTEGER NOT NULL,
    app_id INTEGER,
    app_name TEXT,
    title TEXT NOT NULL,
    author TEXT,
    file_size_bytes INTEGER,
    output_path TEXT NOT NULL,
    completed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS diagnostic_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    level TEXT NOT NULL,
    category TEXT NOT NULL,
    message TEXT NOT NULL,
    details TEXT
);
"#;

/// Relevant SELECT/UPDATE statements also recovered (see RECOVERY_REPORT.md):
/// - SELECT id, workshop_id, app_id, app_name, title, description, author,
///         preview_url, 'workshop_item' as item_type, file_size_bytes, ...
///         FROM queue_items ...
/// - UPDATE queue_items SET position = ?1 WHERE id = ?2;
/// - INSERT INTO download_history (id, workshop_id, app_id, app_name, title,
///         author, file_size_bytes, output_path, ...) ...
/// - DELETE FROM queue_items WHERE status = 'completed';
