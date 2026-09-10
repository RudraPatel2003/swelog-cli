use clap::Args;
use config::{
    config_file::read_config_file,
    overwrite::Overwrite,
    setup::setup_swelog_files_from_config,
};
use highlight::stdout::path_link;
use miette::Result;

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct SetupArgs {
    /// Overwrite existing swelog files.
    #[arg(long = "force")]
    overwrite_existing_files: bool,
}

impl SetupArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let swelog_config = read_config_file(&environment.config_file_path)?;

        setup_swelog_files_from_config(
            &swelog_config,
            &environment.cache_directory,
            Overwrite::from_force_flag(self.overwrite_existing_files),
        )?;

        println!(
            "Created swelog files in your Obsidian vault at {}",
            path_link(&swelog_config.obsidian_vault_path)
        );

        Ok(())
    }
}
