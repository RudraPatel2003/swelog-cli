mod activity;
mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::config_file::read_config_file;
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
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
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
        resolve_selected_date,
    },
};

const GITHUB_SECTION_TITLE: &str = "GitHub";

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct GithubArgs {
    /// Date to fetch GitHub activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch GitHub activity for yesterday instead of today.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Fetch GitHub activity for last Friday instead of today.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

impl GithubArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(DateFlags {
            date: self.date,
            use_yesterday: self.use_yesterday,
            use_last_friday: self.use_last_friday,
        });

        fetch_github_activity(environment, date_selection).await
    }
}

pub async fn fetch_github_activity(
    environment: &Environment,
    date_selection: DateSelection,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::Github.print_fetching_notice();

    let fetch_outcome = collect_github_activity(environment, date_selection).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_github_activity(
    environment: &Environment,
    date_selection: DateSelection,
) -> Result<FetchOutcome> {
    let github_client = environment.build_github_client()?;

    let github_username = github_client.get_username().await?;

    let activity_date =
        resolve_selected_date(date_selection, environment.today)?.unwrap_or(environment.today);

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
