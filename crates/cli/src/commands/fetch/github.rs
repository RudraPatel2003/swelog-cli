mod activity;
mod formatting;

use clap::Args;
use config::config_file::read_config_file;
use miette::Result;

use crate::{
    commands::fetch::{
        github::{
            activity::{
                GitHubActivity,
                get_github_activity,
            },
            formatting::format_github_activity,
        },
        outcome::{
            FetchOutcome,
            WorkFileChange,
            record_fetch_outcome,
        },
        sources::FetchSource,
        window::FetchWindow,
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
    },
};

const GITHUB_SECTION_TITLE: &str = "GitHub";

#[derive(Debug, Args)]
pub struct GithubArgs {
    #[command(flatten)]
    date_flags: DateFlags,
}

impl GithubArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let fetch_window = FetchWindow::from_date_selection(date_selection, environment.today)?;

        fetch_github_activity(environment, fetch_window).await
    }
}

pub async fn fetch_github_activity(
    environment: &Environment,
    fetch_window: FetchWindow,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::Github.print_fetching_notice();

    let fetch_outcome = collect_github_activity(environment, fetch_window).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_github_activity(
    environment: &Environment,
    fetch_window: FetchWindow,
) -> Result<FetchOutcome> {
    let github_client = environment.build_github_client()?;

    let github_username = github_client
        .get_username()
        .await?;

    let activity_date = fetch_window.date_or_today(environment.today);

    let github_activity =
        get_github_activity(&github_client, &github_username, &activity_date).await?;

    let github_fetch_outcome = get_github_fetch_outcome(&github_activity);

    Ok(github_fetch_outcome)
}

fn get_github_fetch_outcome(github_activity: &GitHubActivity) -> FetchOutcome {
    if github_activity.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection { section_title: GITHUB_SECTION_TITLE },
            summary: "No GitHub activity found.".to_string(),
        };
    }

    let pull_request_count = github_activity.count_pull_requests();

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            section_title: GITHUB_SECTION_TITLE,
            content: format_github_activity(github_activity),
        },
        summary: format!("Recorded {pull_request_count} GitHub PRs in your work file."),
    }
}
