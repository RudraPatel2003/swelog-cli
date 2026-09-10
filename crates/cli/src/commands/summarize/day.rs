use clap::Args;
use config::{
    config_file::read_config_file,
    context_file::get_context_file_content,
    setup::swelog_paths::SwelogPaths,
};
use daily_log::file::get_daily_log_file_path;
use highlight::stdout::{
    highlight_cyan,
    path_link,
};
use llm::summarization_settings::SummarizationSettings;
use miette::Result;
use summary::day::summarize_daily_work_from_config;

use crate::{
    environment::Environment,
    shared::{
        daily_log_args::DailyLogArgs,
        summarization_notice::{
            SummarizationPeriod,
            format_summarization_notice,
        },
    },
};

#[derive(Debug, Args)]
pub struct DailySummaryArgs {
    #[command(flatten)]
    daily_log_args: DailyLogArgs,
}

impl DailySummaryArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let swelog_config = read_config_file(&environment.config_file_path)?;

        let summarization_settings = SummarizationSettings::from_config(&swelog_config)?;

        let language_model = environment.build_language_model(&summarization_settings)?;

        let log_date = self.daily_log_args.resolve_log_date(environment.today)?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        let context_file_content = get_context_file_content(&swelog_paths.context_file)?;

        print!(
            "{}",
            format_summarization_notice(
                SummarizationPeriod::Day,
                &summarization_settings,
                context_file_content.as_deref(),
            )
        );

        summarize_daily_work_from_config(
            &swelog_config,
            &environment.cache_directory,
            language_model.as_ref(),
            &log_date,
            context_file_content.as_deref(),
            self.daily_log_args.overwrite(),
            self.daily_log_args.keep_work_file(),
        )
        .await?;

        let daily_log_file = get_daily_log_file_path(&swelog_paths, &log_date);

        println!(
            "Successfully summarized your daily work into {}",
            highlight_cyan(path_link(&daily_log_file))
        );

        Ok(())
    }
}
