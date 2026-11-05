use sentry::issues::structs::{
    SentryIssue,
    SentryIssueSubstatus,
};

pub fn format_sentry_issues(issues: &[SentryIssue]) -> String {
    let mut sorted_issues = issues
        .iter()
        .collect::<Vec<_>>();

    sorted_issues.sort_by_key(|issue| substatus_sort_key(issue.substatus));

    sorted_issues
        .chunk_by(|left, right| substatus_label(left.substatus) == substatus_label(right.substatus))
        .map(format_substatus_group)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_substatus_group(issues: &[&SentryIssue]) -> String {
    let substatus_label = issues
        .first()
        .map_or("", |issue| substatus_label(issue.substatus));

    let issue_lines = issues
        .iter()
        .map(|issue| format_sentry_issue(issue))
        .collect::<Vec<_>>()
        .join("\n");

    format!("### {substatus_label}\n\n{issue_lines}")
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
