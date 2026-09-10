use clap::Args;
use config::{
    config_file::read_config_file,
    setup::swelog_paths::SwelogPaths,
    work_file::{
        create_or_reset_work_file,
        read_work_file,
    },
};
use highlight::stdout::path_link;
use miette::Result;
use undo::snapshot::{
    UndoSnapshot,
    get_undo_snapshot_file_path,
    write_undo_snapshot,
};

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct ResetArgs {}

impl ResetArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file(&environment.config_file_path)?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        let work_file_content = read_work_file(&swelog_paths)?;

        let undo_snapshot = UndoSnapshot { created_file: None, work_file_content };

        let undo_snapshot_file_path = get_undo_snapshot_file_path(&environment.cache_directory);

        write_undo_snapshot(&undo_snapshot_file_path, &undo_snapshot)?;

        create_or_reset_work_file(&swelog_config, &environment.cache_directory)?;

        println!("Reset work file at {}", path_link(&swelog_paths.work_file));

        Ok(())
    }
}
