use sentry::issues::structs::{
    SentryIssue,
    SentryIssueSubstatus,
};

use crate::commands::fetch::sentry::activity::SentryActivity;

const RESOLVED_LABEL: &str = "Resolved";

pub fn format_sentry_activity(sentry_activity: &SentryActivity) -> String {
    let unresolved_groups = format_unresolved_issue_groups(&sentry_activity.unresolved);

    let resolved_group = format_resolved_issue_group(&sentry_activity.resolved);

    unresolved_groups
        .into_iter()
        .chain(resolved_group)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_unresolved_issue_groups(issues: &[SentryIssue]) -> Vec<String> {
    let mut sorted_issues = issues
        .iter()
        .collect::<Vec<_>>();

    sorted_issues.sort_by_key(|issue| substatus_sort_key(issue.substatus));

    sorted_issues
        .chunk_by(|left, right| substatus_label(left.substatus) == substatus_label(right.substatus))
        .map(format_substatus_group)
        .collect()
}

fn format_resolved_issue_group(issues: &[SentryIssue]) -> Option<String> {
    if issues.is_empty() {
        return None;
    }

    let resolved_issues = issues
        .iter()
        .collect::<Vec<_>>();

    Some(format_issue_group(RESOLVED_LABEL, &resolved_issues))
}

fn format_substatus_group(issues: &[&SentryIssue]) -> String {
    let substatus_label = issues
        .first()
        .map_or("", |issue| substatus_label(issue.substatus));

    format_issue_group(substatus_label, issues)
}

fn format_issue_group(label: &str, issues: &[&SentryIssue]) -> String {
    let issue_lines = issues
        .iter()
        .map(|issue| format_sentry_issue(issue))
        .collect::<Vec<_>>()
        .join("\n");

    format!("### {label}\n\n{issue_lines}")
}

fn format_sentry_issue(issue: &SentryIssue) -> String {
    let short_id = &issue.short_id;

    let title = escape_markdown_link_text(&collapse_whitespace(&issue.title));

    format!("- [{short_id}]({}) {title}", issue.permalink)
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn escape_markdown_link_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

const fn substatus_label(substatus: Option<SentryIssueSubstatus>) -> &'static str {
    match substatus {
        Some(SentryIssueSubstatus::Escalating) => "Escalating",

        Some(SentryIssueSubstatus::Regressed) => "Regressed",

        Some(SentryIssueSubstatus::New) => "New",

        Some(SentryIssueSubstatus::Ongoing) => "Ongoing",

        Some(SentryIssueSubstatus::Other) | None => "Other",
    }
}

const fn substatus_sort_key(substatus: Option<SentryIssueSubstatus>) -> u8 {
    match substatus {
        Some(SentryIssueSubstatus::Escalating) => 0,

        Some(SentryIssueSubstatus::Regressed) => 1,

        Some(SentryIssueSubstatus::New) => 2,

        Some(SentryIssueSubstatus::Ongoing) => 3,

        Some(SentryIssueSubstatus::Other) | None => 4,
    }
}

#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;
