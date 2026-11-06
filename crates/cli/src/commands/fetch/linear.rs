mod errors;
mod formatting;

use clap::Args;
use config::{
    config_file::read_config_file,
    swelog_config::SwelogConfig,
};
use dates::formatting::format_date;
use linear::client::structs::LinearIssue;
use miette::Result;

use crate::{
    commands::fetch::{
        linear::{
            errors::MissingLinearUsername,
            formatting::format_linear_issues,
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

const LINEAR_SECTION_TITLE: &str = "Linear";

#[derive(Debug, Args)]
pub struct LinearArgs {
    #[command(flatten)]
    date_flags: DateFlags,
}

impl LinearArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let fetch_window = FetchWindow::from_date_selection(date_selection, environment.today)?;

        fetch_linear_issues(environment, fetch_window).await
    }
}

pub fn describe_missing_linear_configuration(swelog_config: &SwelogConfig) -> Option<&'static str> {
    if swelog_config
        .integrations
        .get_linear_username()
        .is_some()
    {
        return None;
    }

    Some("integrations.linear.username is not configured")
}

pub async fn fetch_linear_issues(
    environment: &Environment,
    fetch_window: FetchWindow,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::Linear.print_fetching_notice();

    let fetch_outcome = collect_linear_issues(environment, &swelog_config, fetch_window).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_linear_issues(
    environment: &Environment,
    swelog_config: &SwelogConfig,
    fetch_window: FetchWindow,
) -> Result<FetchOutcome> {
    let linear_username = swelog_config
        .integrations
        .get_linear_username()
        .ok_or(MissingLinearUsername)?;

    let linear_client = environment.build_linear_client();

    let issues = match fetch_window {
        FetchWindow::Date(activity_date) => {
            linear_client
                .get_assigned_issues_on_date(linear_username, &activity_date)
                .await?
        }

        FetchWindow::Current => {
            linear_client
                .get_current_active_assigned_issues(linear_username, &environment.today)
                .await?
        }
    };

    let linear_fetch_outcome = get_linear_fetch_outcome(&issues, fetch_window);

    Ok(linear_fetch_outcome)
}

fn get_linear_fetch_outcome(issues: &[LinearIssue], fetch_window: FetchWindow) -> FetchOutcome {
    if issues.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection { section_title: LINEAR_SECTION_TITLE },
            summary: format_empty_message(fetch_window),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            section_title: LINEAR_SECTION_TITLE,
            content: format_linear_issues(issues),
        },
        summary: format_recorded_message(issues.len(), fetch_window),
    }
}

fn format_empty_message(fetch_window: FetchWindow) -> String {
    match fetch_window {
        FetchWindow::Date(activity_date) => {
            format!("No Linear activity found for {}.", format_date(&activity_date))
        }

        FetchWindow::Current => "No active Linear issues found.".to_string(),
    }
}

fn format_recorded_message(issue_count: usize, fetch_window: FetchWindow) -> String {
    match fetch_window {
        FetchWindow::Date(activity_date) => format!(
            "Added {issue_count} Linear issues from {} to your work file.",
            format_date(&activity_date)
        ),

        FetchWindow::Current => {
            format!("Added {issue_count} active Linear issues to your work file.")
        }
    }
}
