//! SteamCMD process runner (src/steamcmd/process.rs).
//!
//! Runs: steamcmd +login anonymous +workshop_download_item <app_id> <workshop_id> +quit
//! Streams stdout -> parser -> progress events.
//! Handles SteamGuardRequired (needs real login, not anonymous).

pub async fn download_item(
    steamcmd_path: &str,
    app_id: i64,
    workshop_id: i64,
    on_progress: impl Fn(u64),
) -> anyhow::Result<String> {
    todo!("tokio::process::Command, parse stdout lines")
}
