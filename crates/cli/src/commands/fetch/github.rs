mod activity;
mod formatting;

use async_trait::async_trait;
use config::swelog_config::SwelogConfig;
use credentials::credential::Credential;
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
        integration::{
            Integration,
            IntegrationSpec,
        },
        outcome::{
            FetchOutcome,
            WorkFileChange,
        },
        window::FetchWindow,
    },
    environment::Environment,
};

const GITHUB_SPEC: IntegrationSpec = IntegrationSpec {
    label: "GitHub",
    fetching_notice: "Fetching GitHub PRs...",
    section_title: "GitHub",
    credential: Credential::Github,
};

pub struct GithubIntegration;

#[async_trait]
impl Integration for GithubIntegration {
    fn spec(&self) -> &'static IntegrationSpec {
        &GITHUB_SPEC
    }

    async fn collect_fetch_outcome(
        &self,
        environment: &Environment,
        _swelog_config: &SwelogConfig,
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
}

fn get_github_fetch_outcome(github_activity: &GitHubActivity) -> FetchOutcome {
    if github_activity.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection,
            summary: "No GitHub activity found.".to_string(),
        };
    }

    let pull_request_count = github_activity.count_pull_requests();

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            content: format_github_activity(github_activity),
        },
        summary: format!("Recorded {pull_request_count} GitHub PRs in your work file."),
    }
}
