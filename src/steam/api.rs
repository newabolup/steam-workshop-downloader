//! Steam Web API client (src/steam/api.rs in the original).
//!
//! Endpoints recovered from the binary:
//! - https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/
//! - https://api.steampowered.com/ISteamRemoteStorage/GetCollectionDetails/v1/
//! - https://steamcommunity.com/sharedfiles/filedetails/  (source URLs)
//!
//! Error variants recovered (likely an `enum ApiError`):
//!   ItemNotFound, AccessDenied, SteamApi, SteamCmdNotFound,
//!   SteamCmdExecution, SteamGuardRequired
//! Note: "not found or has been deleted from Steam Workshop (result 9)"
//!   -> result code 9 from GetPublishedFileDetails means deleted item.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedFileDetails {
    pub publishedfileid: String,
    pub creator_app_id: Option<i64>,
    pub consumer_app_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub creator: Option<String>,
    pub file_size: Option<i64>,
    pub preview_url: Option<String>,
    pub time_created: Option<i64>,
    pub time_updated: Option<i64>,
    // result: 1 = OK, 9 = deleted/not found
    pub result: i64,
}

#[derive(Debug)]
pub enum SteamError {
    /// "not found or has been deleted from Steam Workshop (result 9)"
    ItemNotFound,
    AccessDenied,
    SteamApi(String),
    /// "SteamCMD executable not found. Please install or configure it in Settings."
    SteamCmdNotFound,
    SteamCmdExecution(String),
    SteamGuardRequired,
}

impl std::fmt::Display for SteamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for SteamError {}

/// POST form: itemcount=1, publishedfileids[0]=<id>
pub async fn get_published_file_details(
    client: &reqwest::Client,
    workshop_id: i64,
) -> Result<PublishedFileDetails, SteamError> {
    todo!("POST https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/")
}
