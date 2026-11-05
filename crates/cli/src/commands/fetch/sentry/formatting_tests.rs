use markdown::formatting::format_markdown;

use super::*;

fn get_mock_issue(
    short_id: &str,
    title: &str,
    substatus: Option<SentryIssueSubstatus>,
) -> SentryIssue {
    SentryIssue {
        short_id: short_id.to_string(),
        title: title.to_string(),
        permalink: format!("https://acme.sentry.io/issues/{short_id}/"),
        substatus,
    }
}

const SUBSTATUSES_GROUPED_AND_ORDERED: &str = r"### Escalating

- [BACKEND-3](https://acme.sentry.io/issues/BACKEND-3/) Escalating issue

### Regressed

- [BACKEND-1](https://acme.sentry.io/issues/BACKEND-1/) Regressed issue

### New

- [BACKEND-4](https://acme.sentry.io/issues/BACKEND-4/) New issue

### Ongoing

- [BACKEND-2](https://acme.sentry.io/issues/BACKEND-2/) Ongoing issue";

#[test]
fn format_sentry_issues_groups_and_orders_substatuses() {
    let regressed_issue =
        get_mock_issue("BACKEND-1", "Regressed issue", Some(SentryIssueSubstatus::Regressed));

    let ongoing_issue =
        get_mock_issue("BACKEND-2", "Ongoing issue", Some(SentryIssueSubstatus::Ongoing));

    let escalating_issue =
        get_mock_issue("BACKEND-3", "Escalating issue", Some(SentryIssueSubstatus::Escalating));

    let new_issue = get_mock_issue("BACKEND-4", "New issue", Some(SentryIssueSubstatus::New));

    let issues = vec![regressed_issue, ongoing_issue, escalating_issue, new_issue];

    let markdown = format_sentry_issues(&issues);

    assert_eq!(markdown, SUBSTATUSES_GROUPED_AND_ORDERED);
}

const ISSUES_SHARING_A_SUBSTATUS: &str = r"### Ongoing

- [BACKEND-1](https://acme.sentry.io/issues/BACKEND-1/) First
- [BACKEND-2](https://acme.sentry.io/issues/BACKEND-2/) Second";

#[test]
fn format_sentry_issues_groups_issues_sharing_a_substatus() {
    let first_issue = get_mock_issue("BACKEND-1", "First", Some(SentryIssueSubstatus::Ongoing));

    let second_issue = get_mock_issue("BACKEND-2", "Second", Some(SentryIssueSubstatus::Ongoing));

    let issues = vec![first_issue, second_issue];

    let markdown = format_sentry_issues(&issues);

    assert_eq!(markdown, ISSUES_SHARING_A_SUBSTATUS);
}

#[test]
fn format_sentry_issues_keeps_the_order_within_a_substatus() {
    let first_issue = get_mock_issue("BACKEND-2", "Second", Some(SentryIssueSubstatus::Ongoing));

    let second_issue = get_mock_issue("BACKEND-1", "First", Some(SentryIssueSubstatus::Ongoing));

    let issues = vec![first_issue, second_issue];

    let markdown = format_sentry_issues(&issues);

    assert!(markdown.contains("Second\n- [BACKEND-1]"));
}

#[test]
fn format_sentry_issues_collapses_whitespace_and_escapes_link_text() {
    let mock_issue = get_mock_issue(
        "BACKEND-1",
        "TypeError: [object]\nis undefined",
        Some(SentryIssueSubstatus::Ongoing),
    );

    let issues = vec![mock_issue];

    let markdown = format_sentry_issues(&issues);

    assert!(markdown.contains("TypeError: \\[object\\] is undefined"));
}

#[test]
fn format_sentry_issues_labels_an_unknown_or_missing_substatus_as_other() {
    let unknown_issue = get_mock_issue("BACKEND-1", "Mystery", Some(SentryIssueSubstatus::Other));

    let missing_issue = get_mock_issue("BACKEND-2", "Blank", None);

    let issues = vec![unknown_issue, missing_issue];

    let markdown = format_sentry_issues(&issues);

    assert_eq!(markdown.matches("### Other\n").count(), 1);
}

#[test]
fn format_sentry_issues_is_already_formatted_markdown() {
    let escalating_issue =
        get_mock_issue("BACKEND-1", "Escalating issue", Some(SentryIssueSubstatus::Escalating));

    let ongoing_issue =
        get_mock_issue("BACKEND-2", "Ongoing issue", Some(SentryIssueSubstatus::Ongoing));

    let issues = vec![escalating_issue, ongoing_issue];

    let markdown = format!("{}\n", format_sentry_issues(&issues));

    assert_eq!(format_markdown(&markdown), markdown);
}
