use linear::client::structs::{
    LinearIssue,
    LinearStatusType,
};

pub fn format_linear_issues(issues: &[LinearIssue]) -> String {
    let mut sorted_issues = issues.iter().collect::<Vec<_>>();

    sorted_issues.sort_by(|left, right| {
        status_sort_key(left.status_type)
            .cmp(&status_sort_key(right.status_type))
            .then_with(|| left.status_name.cmp(&right.status_name))
    });

    sorted_issues
        .chunk_by(|left, right| left.status_name == right.status_name)
        .map(format_status_group)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_status_group(issues: &[&LinearIssue]) -> String {
    let status_name = issues.first().map_or("", |issue| issue.status_name.as_str());

    let issue_lines =
        issues.iter().map(|issue| format_linear_issue(issue)).collect::<Vec<_>>().join("\n");

    format!("### {status_name}\n\n{issue_lines}")
}

fn format_linear_issue(issue: &LinearIssue) -> String {
    let identifier = &issue.identifier;

    let title = escape_markdown_link_text(&collapse_whitespace(&issue.title));

    format!("- [{identifier}]({}) {title}", issue.url)
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn escape_markdown_link_text(text: &str) -> String {
    text.replace('\\', "\\\\").replace('[', "\\[").replace(']', "\\]")
}

const fn status_sort_key(status_type: LinearStatusType) -> u8 {
    match status_type {
        LinearStatusType::Started => 0,
        LinearStatusType::Unstarted => 1,
        LinearStatusType::Backlog => 2,
        LinearStatusType::Other => 3,
        LinearStatusType::Completed => 4,
        LinearStatusType::Canceled => 5,
    }
}

#[cfg(test)]
#[path = "formatting_tests.rs"]
mod tests;
