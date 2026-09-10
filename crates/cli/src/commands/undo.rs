use clap::Args;
use config::{
    config_file::read_config_file,
    setup::swelog_paths::SwelogPaths,
};
use highlight::stdout::{
    highlight_cyan,
    path_link,
};
use miette::Result;
use undo::{
    restore::restore_undo_snapshot,
    snapshot::{
        UndoSnapshot,
        get_undo_snapshot_file_path,
        read_undo_snapshot,
        remove_undo_snapshot,
    },
};

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct UndoArgs {}

impl UndoArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file(&environment.config_file_path)?;

        let undo_snapshot_file = get_undo_snapshot_file_path(&environment.cache_directory);

        let undo_snapshot = read_undo_snapshot(&undo_snapshot_file)?;

        restore_undo_snapshot(&swelog_config, &undo_snapshot)?;

        remove_undo_snapshot(&undo_snapshot_file)?;

        print_undone_changes(&SwelogPaths::new(&swelog_config), &undo_snapshot);

        Ok(())
    }
}

fn print_undone_changes(swelog_paths: &SwelogPaths, undo_snapshot: &UndoSnapshot) {
    println!("Restored your work file at {}", highlight_cyan(path_link(&swelog_paths.work_file)));

    if let Some(created_file) = &undo_snapshot.created_file {
        println!("Deleted {}", highlight_cyan(path_link(created_file)));
    }
}
