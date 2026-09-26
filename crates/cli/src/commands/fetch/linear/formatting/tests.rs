use linear::client::structs::LinearIssueTimestamps;
use markdown::formatting::format_markdown;

use super::*;

fn get_mock_issue(
    identifier: &str,
    title: &str,
    status_name: &str,
    status_type: LinearStatusType,
) -> LinearIssue {
    LinearIssue {
        identifier: identifier.to_string(),
        title: title.to_string(),
        url: format!("https://linear.app/issue/{identifier}"),
        status_name: status_name.to_string(),
        status_type,
        timestamps: LinearIssueTimestamps::default(),
    }
}

const ACTIVE_STATUSES_GROUPED_AND_ORDERED: &str = r"### In Progress

- [ENG-2](https://linear.app/issue/ENG-2) Started issue

### Todo

- [ENG-1](https://linear.app/issue/ENG-1) Todo issue

### Backlog

- [ENG-3](https://linear.app/issue/ENG-3) Backlog issue";

#[test]
fn format_linear_issues_groups_and_orders_active_statuses() {
    let todo_issue = get_mock_issue("ENG-1", "Todo issue", "Todo", LinearStatusType::Unstarted);

    let started_issue =
        get_mock_issue("ENG-2", "Started issue", "In Progress", LinearStatusType::Started);

    let backlog_issue =
        get_mock_issue("ENG-3", "Backlog issue", "Backlog", LinearStatusType::Backlog);

    let issues = vec![todo_issue, started_issue, backlog_issue];

    let markdown = format_linear_issues(&issues);

    assert_eq!(markdown, ACTIVE_STATUSES_GROUPED_AND_ORDERED);
}

const ISSUES_SHARING_A_STATUS: &str = r"### In Progress

- [ENG-1](https://linear.app/issue/ENG-1) First
- [ENG-2](https://linear.app/issue/ENG-2) Second";

#[test]
fn format_linear_issues_groups_issues_sharing_a_status() {
    let first_issue = get_mock_issue("ENG-1", "First", "In Progress", LinearStatusType::Started);

    let second_issue = get_mock_issue("ENG-2", "Second", "In Progress", LinearStatusType::Started);

    let issues = vec![first_issue, second_issue];

    let markdown = format_linear_issues(&issues);

    assert_eq!(markdown, ISSUES_SHARING_A_STATUS);
}

#[test]
fn format_linear_issues_collapses_whitespace_and_escapes_link_text() {
    let mock_issue =
        get_mock_issue("ENG-1", "Fix [OAuth]\ncallback", "In Progress", LinearStatusType::Started);

    let issues = vec![mock_issue];

    let markdown = format_linear_issues(&issues);

    assert!(markdown.contains("Fix \\[OAuth\\] callback"));
}

#[test]
fn format_linear_issues_is_already_formatted_markdown() {
    let todo_issue = get_mock_issue("ENG-1", "Todo issue", "Todo", LinearStatusType::Unstarted);

    let started_issue =
        get_mock_issue("ENG-2", "Started issue", "In Progress", LinearStatusType::Started);

    let issues = vec![todo_issue, started_issue];

    let markdown = format!("{}\n", format_linear_issues(&issues));

    assert_eq!(format_markdown(&markdown), markdown);
}
