mod formatting;

use async_trait::async_trait;
use config::swelog_config::SwelogConfig;
use credentials::credential::Credential;
use dates::formatting::format_date;
use linear::client::structs::LinearIssue;
use miette::Result;

use crate::{
    commands::fetch::{
        integration::{
            Integration,
            IntegrationSpec,
            errors::MissingIntegrationConfiguration,
        },
        linear::formatting::format_linear_issues,
        outcome::{
            FetchOutcome,
            WorkFileChange,
        },
        window::FetchWindow,
    },
    environment::Environment,
};

const LINEAR_SPEC: IntegrationSpec = IntegrationSpec {
    label: "Linear",
    fetching_notice: "Fetching Linear issues...",
    section_title: "Linear",
    credential: Credential::Linear,
};

const MISSING_LINEAR_USERNAME: MissingIntegrationConfiguration = MissingIntegrationConfiguration {
    setting_label: "Linear username",
    config_key: "integrations.linear.username",
    command_name: "linear",
};

pub struct LinearIntegration;

#[async_trait]
impl Integration for LinearIntegration {
    fn spec(&self) -> &'static IntegrationSpec {
        &LINEAR_SPEC
    }

    fn find_missing_configuration(
        &self,
        swelog_config: &SwelogConfig,
    ) -> Option<MissingIntegrationConfiguration> {
        swelog_config
            .integrations
            .get_linear_username()
            .is_none()
            .then_some(MISSING_LINEAR_USERNAME)
    }

    async fn collect_fetch_outcome(
        &self,
        environment: &Environment,
        swelog_config: &SwelogConfig,
        fetch_window: FetchWindow,
    ) -> Result<FetchOutcome> {
        let linear_username = swelog_config
            .integrations
            .get_linear_username()
            .ok_or(MISSING_LINEAR_USERNAME)?;

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
}

fn get_linear_fetch_outcome(issues: &[LinearIssue], fetch_window: FetchWindow) -> FetchOutcome {
    if issues.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection,
            summary: format_empty_message(fetch_window),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection { content: format_linear_issues(issues) },
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
