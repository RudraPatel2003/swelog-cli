use chrono::NaiveDate;
use miette::Result;
use sentry::{
    client::SentryClient,
    issues::structs::SentryIssue,
};

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

/// Without a date, the unresolved issues cover the last 90 days and the resolved
/// issues cover today.
pub async fn get_sentry_activity(
    sentry_client: &SentryClient,
    sentry_organization: &str,
    activity_date: Option<&NaiveDate>,
    today: &NaiveDate,
) -> Result<SentryActivity> {
    let resolution_date = activity_date.unwrap_or(today);

    let get_unresolved_issues_future =
        get_unresolved_issues(sentry_client, sentry_organization, activity_date);

    let get_resolved_issues_future =
        sentry_client.get_assigned_issues_resolved_on_date(sentry_organization, resolution_date);

    let (unresolved, resolved) =
        tokio::try_join!(get_unresolved_issues_future, get_resolved_issues_future)?;

    Ok(SentryActivity { unresolved, resolved })
}

async fn get_unresolved_issues(
    sentry_client: &SentryClient,
    sentry_organization: &str,
    activity_date: Option<&NaiveDate>,
) -> Result<Vec<SentryIssue>> {
    match activity_date {
        Some(activity_date) => {
            sentry_client
                .get_assigned_issues_on_date(sentry_organization, activity_date)
                .await
        }

        None => {
            sentry_client
                .get_current_assigned_issues(sentry_organization)
                .await
        }
    }
}
