mod all;
mod github;
mod google_calendar;
mod linear;
mod outcome;
mod sources;
mod status;

use all::AllArgs;
use clap::{
    Args,
    Subcommand,
};
use github::GithubArgs;
use google_calendar::GoogleCalendarArgs;
use linear::LinearArgs;
use miette::Result;
use status::StatusArgs;

use crate::environment::Environment;

#[derive(Debug, Args)]
pub struct FetchArgs {
    #[command(subcommand)]
    command: FetchCommands,
}

#[derive(Debug, Subcommand)]
enum FetchCommands {
    /// Run every fetch command you have configured a credential for
    All(AllArgs),

    /// Fetch the PRs you opened, merged, closed, and reviewed on a date in GitHub
    Github(GithubArgs),

    /// Fetch the Linear issues assigned to your configured Linear username
    Linear(LinearArgs),

    /// Fetch the meetings on your Google Calendar for a date
    GoogleCalendar(GoogleCalendarArgs),

    /// Show which fetch commands `swelog fetch all` will run
    Status(StatusArgs),
}

impl FetchArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        match self.command {
            FetchCommands::All(all_args) => all_args.run(environment).await,

            FetchCommands::Github(github_args) => github_args.run(environment).await,

            FetchCommands::Linear(linear_args) => linear_args.run(environment).await,

            FetchCommands::GoogleCalendar(google_calendar_args) => {
                google_calendar_args.run(environment).await
            }

            FetchCommands::Status(status_args) => status_args.run(environment),
        }
    }
}
