use chrono::NaiveDate;
use clap::Args;
use config::{
    config_file::read_config_file,
    context_file::get_context_file_content,
    overwrite::Overwrite,
    setup::swelog_paths::SwelogPaths,
};
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_monday_date,
};
use highlight::stdout::{
    highlight_cyan,
    path_link,
};
use llm::summarization_settings::SummarizationSettings;
use miette::Result;
use summary::week::{
    get_weekly_log_file_path,
    summarize_weekly_work_from_config,
};

use crate::{
    environment::Environment,
    shared::{
        date_selection::{
            WeekSelection,
            resolve_monday_date,
        },
        summarization_notice::{
            SummarizationPeriod,
            format_summarization_notice,
        },
    },
};

#[derive(Debug, Args)]
pub struct WeeklySummaryArgs {
    /// The Monday of the week you want to summarize in the format MM-DD-YYYY.
    #[arg(long = "week-of", value_name = DATE_VALUE_NAME, value_parser = parse_monday_date)]
    monday_date: Option<NaiveDate>,

    /// Summarize the previous week instead of the week containing today.
    #[arg(long = "last-week", conflicts_with = "monday_date")]
    use_last_week: bool,

    /// Overwrite existing weekly log file.
    #[arg(long = "force")]
    overwrite_existing_weekly_log: bool,
}

impl WeeklySummaryArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let swelog_config = read_config_file(&environment.config_file_path)?;

        let week_selection = WeekSelection::from_week_flags(self.monday_date, self.use_last_week);

        let monday_date = resolve_monday_date(week_selection, environment.today)?;

        let summarization_settings = SummarizationSettings::from_config(&swelog_config)?;

        let language_model = environment.build_language_model(&summarization_settings)?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        let context_file_content = get_context_file_content(&swelog_paths.context_file)?;

        print!(
            "{}",
            format_summarization_notice(
                SummarizationPeriod::Week,
                &summarization_settings,
                context_file_content.as_deref(),
            )
        );

        summarize_weekly_work_from_config(
            &swelog_config,
            language_model.as_ref(),
            &monday_date,
            context_file_content.as_deref(),
            Overwrite::from_force_flag(self.overwrite_existing_weekly_log),
        )
        .await?;

        let weekly_log_file = get_weekly_log_file_path(&swelog_paths, &monday_date);

        println!(
            "Successfully summarized your weekly work into {}",
            highlight_cyan(path_link(&weekly_log_file))
        );

        Ok(())
    }
}
