mod day;
mod week;

use clap::{
    Args,
    Subcommand,
};
use day::DailySummaryArgs;
use miette::Result;
use week::WeeklySummaryArgs;

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct SummarizeArgs {
    #[command(flatten)]
    daily_summary_args: DailySummaryArgs,

    #[command(subcommand)]
    command: Option<SummarizeCommands>,
}

#[derive(Debug, Subcommand)]
enum SummarizeCommands {
    /// Summarize your configured work file and log it into the daily folder.
    Day(DailySummaryArgs),

    /// Summarize the past week of daily logs into a weekly log.
    Week(WeeklySummaryArgs),
}

impl SummarizeArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        match self.command {
            Some(SummarizeCommands::Day(daily_summary_args)) => {
                daily_summary_args.run(environment).await?;
            }

            Some(SummarizeCommands::Week(weekly_summary_args)) => {
                weekly_summary_args.run(environment).await?;
            }

            None => {
                self.daily_summary_args.run(environment).await?;
            }
        }

        Ok(())
    }
}
