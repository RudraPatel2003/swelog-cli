mod errors;
mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::{
    config_file::read_config_file,
    swelog_config::SwelogConfig,
};
use dates::{
    date_format::DATE_VALUE_NAME,
    formatting::format_date,
    parsing::parse_date,
};
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
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
        resolve_selected_date,
    },
};

const LINEAR_SECTION_TITLE: &str = "Linear";

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct LinearArgs {
    /// Date to fetch Linear activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch the Linear issues you worked on yesterday.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Fetch the Linear issues you worked on last Friday.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

impl LinearArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(DateFlags {
            date: self.date,
            use_yesterday: self.use_yesterday,
            use_last_friday: self.use_last_friday,
        });

        fetch_linear_issues(environment, date_selection).await
    }
}

pub fn describe_missing_linear_configuration(swelog_config: &SwelogConfig) -> Option<&'static str> {
    if swelog_config.get_linear_username().is_some() {
        return None;
    }

    Some("linearUsername is not configured")
}

pub async fn fetch_linear_issues(
    environment: &Environment,
    date_selection: DateSelection,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::Linear.print_fetching_notice();

    let fetch_outcome = collect_linear_issues(environment, &swelog_config, date_selection).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_linear_issues(
    environment: &Environment,
    swelog_config: &SwelogConfig,
    date_selection: DateSelection,
) -> Result<FetchOutcome> {
    let linear_username = swelog_config.get_linear_username().ok_or(MissingLinearUsername)?;

    let linear_client = environment.build_linear_client();

    let activity_date = resolve_selected_date(date_selection, environment.today)?;

    let issues = match activity_date {
        Some(activity_date) => {
            linear_client.get_assigned_issues_on_date(linear_username, &activity_date).await?
        }

        None => {
            linear_client
                .get_current_active_assigned_issues(linear_username, &environment.today)
                .await?
        }
    };

    let linear_fetch_outcome = get_linear_fetch_outcome(&issues, activity_date.as_ref());

    Ok(linear_fetch_outcome)
}

fn get_linear_fetch_outcome(
    issues: &[LinearIssue],
    activity_date: Option<&NaiveDate>,
) -> FetchOutcome {
    if issues.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection { section_title: LINEAR_SECTION_TITLE },
            summary: format_empty_message(activity_date),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            section_title: LINEAR_SECTION_TITLE,
            content: format_linear_issues(issues),
        },
        summary: format_recorded_message(issues.len(), activity_date),
    }
}

fn format_empty_message(activity_date: Option<&NaiveDate>) -> String {
    activity_date.map_or_else(
        || "No active Linear issues found.".to_string(),
        |activity_date| format!("No Linear activity found for {}.", format_date(activity_date)),
    )
}

fn format_recorded_message(issue_count: usize, activity_date: Option<&NaiveDate>) -> String {
    activity_date.map_or_else(
        || format!("Added {issue_count} active Linear issues to your work file."),
        |activity_date| {
            format!(
                "Added {issue_count} Linear issues from {} to your work file.",
                format_date(activity_date)
            )
        },
    )
}
