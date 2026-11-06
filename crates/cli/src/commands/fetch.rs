mod all;
mod github;
mod google_calendar;
mod integration;
mod integration_args;
mod integrations;
mod linear;
mod outcome;
mod sentry;
mod status;
mod window;

use all::AllArgs;
use clap::{
    Args,
    Subcommand,
};
use github::GithubIntegration;
use google_calendar::GoogleCalendarIntegration;
use integration_args::IntegrationArgs;
use linear::LinearIntegration;
use miette::Result;
use sentry::SentryIntegration;
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
    Github(IntegrationArgs),

    /// Fetch the Linear issues assigned to your configured Linear username
    Linear(IntegrationArgs),

    /// Fetch the meetings on your Google Calendar for a date
    GoogleCalendar(IntegrationArgs),

    /// Fetch the Sentry issues assigned to you
    Sentry(IntegrationArgs),

    /// Show which fetch commands `swelog fetch all` will run
    Status(StatusArgs),
}

impl FetchArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        match self.command {
            FetchCommands::All(all_args) => {
                all_args
                    .run(environment)
                    .await
            }

            FetchCommands::Github(integration_args) => {
                integration_args
                    .run(&GithubIntegration, environment)
                    .await
            }

            FetchCommands::Linear(integration_args) => {
                integration_args
                    .run(&LinearIntegration, environment)
                    .await
            }

            FetchCommands::GoogleCalendar(integration_args) => {
                integration_args
                    .run(&GoogleCalendarIntegration, environment)
                    .await
            }

            FetchCommands::Sentry(integration_args) => {
                integration_args
                    .run(&SentryIntegration, environment)
                    .await
            }

            FetchCommands::Status(status_args) => status_args.run(environment),
        }
    }
}
