use clap::Args;
use config::{
    config_file::read_config_file,
    setup::swelog_paths::SwelogPaths,
};
use daily_log::{
    file::get_daily_log_file_path,
    write::write_daily_log_from_config,
};
use highlight::stdout::{
    highlight_cyan,
    path_link,
};
use miette::Result;

use crate::{
    environment::Environment,
    shared::daily_log_args::DailyLogArgs,
};

#[derive(Debug, Args)]
pub struct LogArgs {
    #[command(flatten)]
    daily_log_args: DailyLogArgs,
}

impl LogArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let swelog_config = read_config_file(&environment.config_file_path)?;

        let log_date = self.daily_log_args.resolve_log_date(environment.today)?;

        write_daily_log_from_config(
            &swelog_config,
            &environment.cache_directory,
            &log_date,
            self.daily_log_args.overwrite(),
            self.daily_log_args.keep_work_file(),
        )?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        let daily_log_file = get_daily_log_file_path(&swelog_paths, &log_date);

        println!("Logged your work into {}", highlight_cyan(path_link(&daily_log_file)));

        Ok(())
    }
}
