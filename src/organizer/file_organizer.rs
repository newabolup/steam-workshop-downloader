//! File organizer (src/organizer/file_organizer.rs).
//!
//! Evidence from the binary:
//! - "Staged workshop content directory is empty at ..."
//! - "...iting existing files." (probably "...overwriting existing files.")
//! - "workshop_manifest.json"
//!
//! Flow (inferred): steamcmd downloads into a staging dir; the organizer
//! moves files to the final output dir, writes workshop_manifest.json
//! (id/title/author mapping), optionally overwrites existing files.

use std::path::Path;

pub async fn organize(
    staging_dir: &Path,
    output_dir: &Path,
    overwrite: bool,
) -> anyhow::Result<()> {
    todo!("move files, write workshop_manifest.json")
}
