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
use miette::Result;
use sentry::issues::structs::SentryIssue;

use crate::{
    commands::fetch::{
        outcome::{
            FetchOutcome,
            WorkFileChange,
            record_fetch_outcome,
        },
        sentry::{
            errors::MissingSentryOrganization,
            formatting::format_sentry_issues,
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

const SENTRY_SECTION_TITLE: &str = "Sentry";

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct SentryArgs {
    /// Date to fetch Sentry activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch the Sentry issues that received events yesterday.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Fetch the Sentry issues that received events last Friday.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

impl SentryArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(DateFlags {
            date: self.date,
            use_yesterday: self.use_yesterday,
            use_last_friday: self.use_last_friday,
        });

        fetch_sentry_issues(environment, date_selection).await
    }
}

pub fn describe_missing_sentry_configuration(swelog_config: &SwelogConfig) -> Option<&'static str> {
    if swelog_config
        .get_sentry_organization()
        .is_some()
    {
        return None;
    }

    Some("sentryOrganization is not configured")
}

pub async fn fetch_sentry_issues(
    environment: &Environment,
    date_selection: DateSelection,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::Sentry.print_fetching_notice();

    let fetch_outcome = collect_sentry_issues(environment, &swelog_config, date_selection).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_sentry_issues(
    environment: &Environment,
    swelog_config: &SwelogConfig,
    date_selection: DateSelection,
) -> Result<FetchOutcome> {
    let sentry_organization = swelog_config
        .get_sentry_organization()
        .ok_or(MissingSentryOrganization)?;

    let sentry_client = environment.build_sentry_client()?;

    let activity_date = resolve_selected_date(date_selection, environment.today)?;

    let issues = match activity_date {
        Some(activity_date) => {
            sentry_client
                .get_assigned_issues_on_date(sentry_organization, &activity_date)
                .await?
        }

        None => {
            sentry_client
                .get_current_assigned_issues(sentry_organization)
                .await?
        }
    };

    let sentry_fetch_outcome = get_sentry_fetch_outcome(&issues, activity_date.as_ref());

    Ok(sentry_fetch_outcome)
}

fn get_sentry_fetch_outcome(
    issues: &[SentryIssue],
    activity_date: Option<&NaiveDate>,
) -> FetchOutcome {
    if issues.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection { section_title: SENTRY_SECTION_TITLE },
            summary: format_empty_message(activity_date),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            section_title: SENTRY_SECTION_TITLE,
            content: format_sentry_issues(issues),
        },
        summary: format_recorded_message(issues.len(), activity_date),
    }
}

fn format_empty_message(activity_date: Option<&NaiveDate>) -> String {
    activity_date.map_or_else(
        || "No unresolved Sentry issues found.".to_string(),
        |activity_date| format!("No Sentry activity found for {}.", format_date(activity_date)),
    )
}

fn format_recorded_message(issue_count: usize, activity_date: Option<&NaiveDate>) -> String {
    activity_date.map_or_else(
        || format!("Added {issue_count} unresolved Sentry issues to your work file."),
        |activity_date| {
            format!(
                "Added {issue_count} Sentry issues from {} to your work file.",
                format_date(activity_date)
            )
        },
    )
}
