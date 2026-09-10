use clap::Args;
use config::{
    init::write_default_config,
    overwrite::Overwrite,
    swelog_config::SwelogConfig,
};
use highlight::stdout::{
    highlight_cyan,
    path_link,
};
use miette::Result;

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Overwrite an existing config file with defaults.
    #[arg(long = "force")]
    overwrite_existing_config: bool,
}

impl InitArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let default_config = SwelogConfig::get_default_config();

        write_default_config(
            &environment.config_file_path,
            &default_config,
            Overwrite::from_force_flag(self.overwrite_existing_config),
        )?;

        println!(
            "Created swelog config at {}",
            highlight_cyan(path_link(&environment.config_file_path))
        );

        Ok(())
    }
}
