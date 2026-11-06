mod activity;
mod formatting;

use async_trait::async_trait;
use config::swelog_config::SwelogConfig;
use credentials::credential::Credential;
use dates::formatting::format_date;
use miette::Result;

use crate::{
    commands::fetch::{
        integration::{
            Integration,
            IntegrationSpec,
            errors::MissingIntegrationConfiguration,
        },
        outcome::{
            FetchOutcome,
            WorkFileChange,
        },
        sentry::{
            activity::{
                SentryActivity,
                get_sentry_activity,
            },
            formatting::format_sentry_activity,
        },
        window::FetchWindow,
    },
    environment::Environment,
};

const SENTRY_SPEC: IntegrationSpec = IntegrationSpec {
    label: "Sentry",
    fetching_notice: "Fetching Sentry issues...",
    section_title: "Sentry",
    credential: Credential::Sentry,
};

const MISSING_SENTRY_ORGANIZATION: MissingIntegrationConfiguration =
    MissingIntegrationConfiguration {
        setting_label: "Sentry organization",
        config_key: "integrations.sentry.organization",
        command_name: "sentry",
    };

pub struct SentryIntegration;

#[async_trait]
impl Integration for SentryIntegration {
    fn spec(&self) -> &'static IntegrationSpec {
        &SENTRY_SPEC
    }

    fn find_missing_configuration(
        &self,
        swelog_config: &SwelogConfig,
    ) -> Option<MissingIntegrationConfiguration> {
        swelog_config
            .integrations
            .get_sentry_organization()
            .is_none()
            .then_some(MISSING_SENTRY_ORGANIZATION)
    }

    async fn collect_fetch_outcome(
        &self,
        environment: &Environment,
        swelog_config: &SwelogConfig,
        fetch_window: FetchWindow,
    ) -> Result<FetchOutcome> {
        let sentry_organization = swelog_config
            .integrations
            .get_sentry_organization()
            .ok_or(MISSING_SENTRY_ORGANIZATION)?;

        let sentry_client = environment.build_sentry_client()?;

        let sentry_activity = get_sentry_activity(
            &sentry_client,
            sentry_organization,
            fetch_window,
            environment.today,
        )
        .await?;

        let sentry_fetch_outcome = get_sentry_fetch_outcome(&sentry_activity, fetch_window);

        Ok(sentry_fetch_outcome)
    }
}

fn get_sentry_fetch_outcome(
    sentry_activity: &SentryActivity,
    fetch_window: FetchWindow,
) -> FetchOutcome {
    if sentry_activity.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection,
            summary: format_empty_message(fetch_window),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
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
