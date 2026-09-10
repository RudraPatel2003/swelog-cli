use std::{
    fs,
    path::Path,
};

use config::{
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
};
use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::snapshot::UndoSnapshot;

pub fn restore_undo_snapshot(
    swelog_config: &SwelogConfig,
    undo_snapshot: &UndoSnapshot,
) -> Result<()> {
    let swelog_paths = SwelogPaths::new(swelog_config);

    fs::write(&swelog_paths.work_file, &undo_snapshot.work_file_content)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!("failed to write work file at {}", path_link(&swelog_paths.work_file))
        })?;

    delete_created_file(undo_snapshot.created_file.as_deref())
}

fn delete_created_file(created_file: Option<&Path>) -> Result<()> {
    let Some(created_file) = created_file else {
        return Ok(());
    };

    if !created_file.exists() {
        return Ok(());
    }

    fs::remove_file(created_file)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to delete {}", path_link(created_file)))
}

#[cfg(test)]
mod tests;
