mod activity;
mod errors;
mod formatting;

use clap::Args;
use config::{
    config_file::read_config_file,
    swelog_config::SwelogConfig,
};
use dates::formatting::format_date;
use miette::Result;

use crate::{
    commands::fetch::{
        outcome::{
            FetchOutcome,
            WorkFileChange,
            record_fetch_outcome,
        },
        sentry::{
            activity::{
                SentryActivity,
                get_sentry_activity,
            },
            errors::MissingSentryOrganization,
            formatting::format_sentry_activity,
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

const SENTRY_SECTION_TITLE: &str = "Sentry";

#[derive(Debug, Args)]
pub struct SentryArgs {
    #[command(flatten)]
    date_flags: DateFlags,
}

impl SentryArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let fetch_window = FetchWindow::from_date_selection(date_selection, environment.today)?;

        fetch_sentry_issues(environment, fetch_window).await
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
    fetch_window: FetchWindow,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::Sentry.print_fetching_notice();

    let fetch_outcome = collect_sentry_issues(environment, &swelog_config, fetch_window).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_sentry_issues(
    environment: &Environment,
    swelog_config: &SwelogConfig,
    fetch_window: FetchWindow,
) -> Result<FetchOutcome> {
    let sentry_organization = swelog_config
        .get_sentry_organization()
        .ok_or(MissingSentryOrganization)?;

    let sentry_client = environment.build_sentry_client()?;

    let sentry_activity =
        get_sentry_activity(&sentry_client, sentry_organization, fetch_window, environment.today)
            .await?;

    let sentry_fetch_outcome = get_sentry_fetch_outcome(&sentry_activity, fetch_window);

    Ok(sentry_fetch_outcome)
}

fn get_sentry_fetch_outcome(
    sentry_activity: &SentryActivity,
    fetch_window: FetchWindow,
) -> FetchOutcome {
    if sentry_activity.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection { section_title: SENTRY_SECTION_TITLE },
            summary: format_empty_message(fetch_window),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            section_title: SENTRY_SECTION_TITLE,
            content: format_sentry_activity(sentry_activity),
        },
        summary: format_recorded_message(sentry_activity.count_issues(), fetch_window),
    }
}

fn format_empty_message(fetch_window: FetchWindow) -> String {
    match fetch_window {
        FetchWindow::Date(activity_date) => {
            format!("No Sentry activity found for {}.", format_date(&activity_date))
        }

        FetchWindow::Current => "No Sentry issues found.".to_string(),
    }
}

fn format_recorded_message(issue_count: usize, fetch_window: FetchWindow) -> String {
    match fetch_window {
        FetchWindow::Date(activity_date) => format!(
            "Added {issue_count} Sentry issues from {} to your work file.",
            format_date(&activity_date)
        ),

        FetchWindow::Current => format!("Added {issue_count} Sentry issues to your work file."),
    }
}
