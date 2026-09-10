use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    process,
};

use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::errors::NoUndoSnapshot;

const UNDO_SNAPSHOT_FILE_NAME: &str = "undo.json";

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoSnapshot {
    pub created_file: Option<PathBuf>,

    pub work_file_content: String,
}

#[must_use]
pub fn get_undo_snapshot_file_path(cache_directory: &Path) -> PathBuf {
    cache_directory.join(UNDO_SNAPSHOT_FILE_NAME)
}

pub fn read_undo_snapshot(undo_snapshot_file: &Path) -> Result<UndoSnapshot> {
    if !undo_snapshot_file.is_file() {
        let no_undo_snapshot_error = NoUndoSnapshot;

        return Err(no_undo_snapshot_error.into());
    }

    let undo_snapshot_contents =
        fs::read_to_string(undo_snapshot_file).into_diagnostic().wrap_err_with(|| {
            format!("failed to read the undo snapshot at {}", path_link(undo_snapshot_file))
        })?;

    let undo_snapshot = serde_json::from_str(&undo_snapshot_contents)
        .into_diagnostic()
        .wrap_err("failed to parse the undo snapshot")?;

    Ok(undo_snapshot)
}

pub fn write_undo_snapshot(undo_snapshot_file: &Path, undo_snapshot: &UndoSnapshot) -> Result<()> {
    if let Some(parent) = undo_snapshot_file.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create the swelog cache directory at {}", path_link(parent))
        })?;
    }

    let json = serde_json::to_string(undo_snapshot)
        .into_diagnostic()
        .wrap_err("failed to serialize the undo snapshot")?;

    let temporary_file_path = undo_snapshot_file.with_extension(format!("{}.tmp", process::id()));

    fs::write(&temporary_file_path, json).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the undo snapshot at {}", path_link(&temporary_file_path))
    })?;

    fs::rename(&temporary_file_path, undo_snapshot_file).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the undo snapshot at {}", path_link(undo_snapshot_file))
    })?;

    Ok(())
}

pub fn remove_undo_snapshot(undo_snapshot_file: &Path) -> Result<()> {
    if !undo_snapshot_file.exists() {
        return Ok(());
    }

    fs::remove_file(undo_snapshot_file).into_diagnostic().wrap_err_with(|| {
        format!("failed to remove the undo snapshot at {}", path_link(undo_snapshot_file))
    })
}

#[cfg(test)]
mod tests;
