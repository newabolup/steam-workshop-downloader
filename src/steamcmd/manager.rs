//! SteamCMD management (src/steamcmd/manager.rs).
//!
//! - install_steamcmd: downloads https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip
//!   extracts it, stores the path in app_settings
//! - detect_steamcmd: finds an existing install (PATH, common locations,
//!   or the customPath argument)

pub const STEAMCMD_ZIP_URL: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";

pub async fn install_steamcmd() -> anyhow::Result<String> {
    todo!("download zip -> extract to app data dir -> return exe path")
}

pub async fn detect_steamcmd(custom_path: Option<String>) -> anyhow::Result<String> {
    todo!("custom_path.or(search PATH).or(common locations)")
}
