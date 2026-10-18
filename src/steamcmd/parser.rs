//! SteamCMD output parser (src/steamcmd/parser.rs).
//!
//! Parses steamcmd stdout to track download progress, e.g. lines like:
//!   "Downloading item 12345 ..." / "Success. Downloaded item ..."

/// Progress update parsed from one stdout line.
pub struct ProgressUpdate {
    pub workshop_id: i64,
    pub downloaded_bytes: Option<u64>,
    pub done: bool,
}

pub fn parse_line(line: &str) -> Option<ProgressUpdate> {
    todo!("regex over steamcmd output")
}
