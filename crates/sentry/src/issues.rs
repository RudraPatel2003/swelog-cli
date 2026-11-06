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
    issue_activity::was_resolved_during,
    issues::structs::SentryIssue,
};

const UNRESOLVED_AND_ASSIGNED_TO_ME_QUERY: &str = "assigned:me is:unresolved";

const RESOLVED_AND_ASSIGNED_TO_ME_QUERY: &str = "assigned:me is:resolved";

const ALL_ACCESSIBLE_PROJECTS: &str = "-1";

/// Sentry caps a page at 100 issues.
const PAGE_SIZE: &str = "100";

/// Leaves the hourly event counts out of every issue.
const COLLAPSED_FIELDS: &str = "stats";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IssueStatus {
    Unresolved,
    Resolved,
}

impl IssueStatus {
    const fn get_assigned_to_me_query(self) -> &'static str {
        match self {
            Self::Unresolved => UNRESOLVED_AND_ASSIGNED_TO_ME_QUERY,

            Self::Resolved => RESOLVED_AND_ASSIGNED_TO_ME_QUERY,
        }
    }
}

impl SentryClient {
    pub async fn get_assigned_issues_on_date(
        &self,
        organization: &str,
        date: &NaiveDate,
    ) -> Result<Vec<SentryIssue>> {
        let activity_window = get_activity_window(*date, &Local)?;

        self.fetch_all_assigned_issues(
            organization,
            IssueStatus::Unresolved,
            Some(&activity_window),
        )
        .await
    }

    /// Sentry searches the last 90 days of activity when no window is given.
    pub async fn get_current_assigned_issues(
        &self,
        organization: &str,
    ) -> Result<Vec<SentryIssue>> {
        self.fetch_all_assigned_issues(organization, IssueStatus::Unresolved, None)
            .await
    }

    /// A date window would only match issues that received events on the date,
    /// so the resolved issues are searched without one and kept when their
    /// activity shows a resolution on the date
    pub async fn get_assigned_issues_resolved_on_date(
        &self,
        organization: &str,
        date: &NaiveDate,
    ) -> Result<Vec<SentryIssue>> {
        let resolution_window = get_activity_window(*date, &Local)?;

        let resolved_issues = self
            .fetch_all_assigned_issues(organization, IssueStatus::Resolved, None)
            .await?;

        let resolved_issue_ids = resolved_issues
            .iter()
            .map(|issue| issue.id.as_str());

        let issue_activities = self
            .get_issue_activities(organization, resolved_issue_ids)
            .await?;

        let issues_resolved_on_date = resolved_issues
            .into_iter()
            .zip(issue_activities)
            .filter(|(_, issue_activity)| was_resolved_during(issue_activity, &resolution_window))
            .map(|(issue, _)| issue)
            .collect();

        Ok(issues_resolved_on_date)
    }

    async fn fetch_all_assigned_issues(
        &self,
        organization: &str,
        issue_status: IssueStatus,
        activity_window: Option<&ActivityWindow>,
    ) -> Result<Vec<SentryIssue>> {
        let endpoint_path = get_issues_endpoint_path(organization);

        let mut issues = Vec::new();

        let mut cursor: Option<String> = None;

        loop {
            let query_parameters =
                get_issues_query_parameters(issue_status, activity_window, cursor.as_deref());

            let page = self
                .get_json_page(&endpoint_path, &query_parameters)
                .await?;

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
    issue_status: IssueStatus,
    activity_window: Option<&ActivityWindow>,
    cursor: Option<&str>,
) -> Vec<(&'static str, String)> {
    let mut query_parameters = vec![
        (
            "query",
            issue_status
                .get_assigned_to_me_query()
                .to_string(),
        ),
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
