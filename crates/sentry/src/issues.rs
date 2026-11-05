pub mod structs;

use chrono::{
    Local,
    NaiveDate,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::{
    activity::{
        ActivityWindow,
        get_activity_window,
    },
    client::SentryClient,
    errors::RepeatedSentryPaginationCursor,
    issues::structs::SentryIssue,
};

const UNRESOLVED_AND_ASSIGNED_TO_ME_QUERY: &str = "assigned:me is:unresolved";

const ALL_ACCESSIBLE_PROJECTS: &str = "-1";

/// Sentry caps a page at 100 issues.
const PAGE_SIZE: &str = "100";

/// Leaves the hourly event counts out of every issue.
const COLLAPSED_FIELDS: &str = "stats";

impl SentryClient {
    pub async fn get_assigned_issues_on_date(
        &self,
        organization: &str,
        date: &NaiveDate,
    ) -> Result<Vec<SentryIssue>> {
        let activity_window = get_activity_window(*date, &Local)?;

        self.fetch_all_assigned_issues(organization, Some(&activity_window)).await
    }

    /// Sentry searches the last 90 days of activity when no window is given.
    pub async fn get_current_assigned_issues(
        &self,
        organization: &str,
    ) -> Result<Vec<SentryIssue>> {
        self.fetch_all_assigned_issues(organization, None).await
    }

    async fn fetch_all_assigned_issues(
        &self,
        organization: &str,
        activity_window: Option<&ActivityWindow>,
    ) -> Result<Vec<SentryIssue>> {
        let endpoint_path = get_issues_endpoint_path(organization);

        let mut issues = Vec::new();

        let mut cursor: Option<String> = None;

        loop {
            let query_parameters = get_issues_query_parameters(activity_window, cursor.as_deref());

            let page = self.get_json_page(&endpoint_path, &query_parameters).await?;

            issues.append(&mut parse_issues_response_text(&page.body)?);

            let Some(next_cursor) = page.next_cursor else {
                return Ok(issues);
            };

            ensure_cursor_advanced(cursor.as_deref(), &next_cursor)?;

            cursor = Some(next_cursor);
        }
    }
}

fn get_issues_endpoint_path(organization: &str) -> String {
    format!("organizations/{organization}/issues/")
}

fn get_issues_query_parameters(
    activity_window: Option<&ActivityWindow>,
    cursor: Option<&str>,
) -> Vec<(&'static str, String)> {
    let mut query_parameters = vec![
        ("query", UNRESOLVED_AND_ASSIGNED_TO_ME_QUERY.to_string()),
        ("project", ALL_ACCESSIBLE_PROJECTS.to_string()),
        ("limit", PAGE_SIZE.to_string()),
        ("collapse", COLLAPSED_FIELDS.to_string()),
    ];

    if let Some(activity_window) = activity_window {
        query_parameters.push(("start", activity_window.format_start()));

        query_parameters.push(("end", activity_window.format_end()));
    }

    if let Some(cursor) = cursor {
        query_parameters.push(("cursor", cursor.to_string()));
    }

    query_parameters
}

fn ensure_cursor_advanced(previous_cursor: Option<&str>, next_cursor: &str) -> Result<()> {
    if previous_cursor == Some(next_cursor) {
        let repeated_sentry_pagination_cursor_error = RepeatedSentryPaginationCursor;

        return Err(repeated_sentry_pagination_cursor_error.into());
    }

    Ok(())
}

fn parse_issues_response_text(response_text: &str) -> Result<Vec<SentryIssue>> {
    serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse Sentry issues response")
}

#[cfg(test)]
#[path = "issues_tests.rs"]
mod tests;
