pub mod structs;

use futures::{
    StreamExt,
    TryStreamExt,
    stream,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::{
    activity::ActivityWindow,
    client::SentryClient,
    issue_activity::structs::{
        SentryActivityEntry,
        SentryIssueDetails,
    },
};

const CONCURRENT_ISSUE_ACTIVITY_REQUESTS: usize = 8;

/// Sentry documents collapsing these fields as a way to speed up the issue
/// details request, and only the activity is needed
const COLLAPSED_ISSUE_DETAILS: [(&str, &str); 3] =
    [("collapse", "release"), ("collapse", "stats"), ("collapse", "tags")];

impl SentryClient {
    /// Owned issue IDs keep the returned future `Send`. Borrowed IDs in the stream closure hit
    /// rust-lang/rust#102211
    pub(crate) async fn get_issue_activities(
        &self,
        organization: &str,
        issue_ids: Vec<String>,
    ) -> Result<Vec<Vec<SentryActivityEntry>>> {
        stream::iter(issue_ids)
            .map(|issue_id| self.get_issue_activity(organization, issue_id))
            .buffered(CONCURRENT_ISSUE_ACTIVITY_REQUESTS)
            .try_collect()
            .await
    }

    async fn get_issue_activity(
        &self,
        organization: &str,
        issue_id: String,
    ) -> Result<Vec<SentryActivityEntry>> {
        let endpoint_path = get_issue_details_endpoint_path(organization, &issue_id);

        let page = self
            .get_json_page(&endpoint_path, &COLLAPSED_ISSUE_DETAILS)
            .await?;

        parse_issue_activity_response_text(&page.body)
    }
}

pub(crate) fn was_resolved_during(
    activity_entries: &[SentryActivityEntry],
    activity_window: &ActivityWindow,
) -> bool {
    activity_entries
        .iter()
        .any(|activity_entry| {
            activity_entry
                .activity_type
                .is_resolution()
                && activity_window.contains(activity_entry.date_created)
        })
}

fn get_issue_details_endpoint_path(organization: &str, issue_id: &str) -> String {
    format!("organizations/{organization}/issues/{issue_id}/")
}

fn parse_issue_activity_response_text(response_text: &str) -> Result<Vec<SentryActivityEntry>> {
    let issue_details: SentryIssueDetails = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse Sentry issue details response")?;

    Ok(issue_details.activity)
}

#[cfg(test)]
#[path = "issue_activity_tests.rs"]
mod tests;
