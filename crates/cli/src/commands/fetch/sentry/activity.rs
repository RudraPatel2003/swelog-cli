use chrono::NaiveDate;
use miette::Result;
use sentry::{
    client::SentryClient,
    issues::structs::SentryIssue,
};

use crate::commands::fetch::window::FetchWindow;

pub struct SentryActivity {
    pub unresolved: Vec<SentryIssue>,

    pub resolved: Vec<SentryIssue>,
}

impl SentryActivity {
    pub const fn is_empty(&self) -> bool {
        self.unresolved
            .is_empty()
            && self
                .resolved
                .is_empty()
    }

    pub fn count_issues(&self) -> usize {
        [&self.unresolved, &self.resolved]
            .iter()
            .map(|issues| issues.len())
            .sum()
    }
}

/// In the current window, the unresolved issues cover the last 90 days and the
/// resolved issues cover today.
pub async fn get_sentry_activity(
    sentry_client: &SentryClient,
    sentry_organization: &str,
    fetch_window: FetchWindow,
    today: NaiveDate,
) -> Result<SentryActivity> {
    let resolution_date = fetch_window.date_or_today(today);

    let get_unresolved_issues_future =
        get_unresolved_issues(sentry_client, sentry_organization, fetch_window);

    let get_resolved_issues_future =
        sentry_client.get_assigned_issues_resolved_on_date(sentry_organization, &resolution_date);

    let (unresolved, resolved) =
        tokio::try_join!(get_unresolved_issues_future, get_resolved_issues_future)?;

    Ok(SentryActivity { unresolved, resolved })
}

async fn get_unresolved_issues(
    sentry_client: &SentryClient,
    sentry_organization: &str,
    fetch_window: FetchWindow,
) -> Result<Vec<SentryIssue>> {
    match fetch_window {
        FetchWindow::Date(activity_date) => {
            sentry_client
                .get_assigned_issues_on_date(sentry_organization, &activity_date)
                .await
        }

        FetchWindow::Current => {
            sentry_client
                .get_current_assigned_issues(sentry_organization)
                .await
        }
    }
}
