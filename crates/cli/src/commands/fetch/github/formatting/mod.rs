use github::{
    issues::Issue,
    repository_name::get_repository_name_from_repository_url,
};

use crate::commands::fetch::github::activity::GitHubActivity;

pub fn format_github_activity(github_activity: &GitHubActivity) -> String {
    let pull_requests_by_action = [
        ("Opened", &github_activity.opened),
        ("Merged", &github_activity.merged),
        ("Closed", &github_activity.closed),
        ("Reviewed", &github_activity.reviewed),
    ];

    let sections: Vec<String> = pull_requests_by_action
        .into_iter()
        .filter(|(_, pull_requests)| !pull_requests.is_empty())
        .map(|(action, pull_requests)| format_pull_request_section(action, pull_requests))
        .collect();

    sections.join("\n\n")
}

fn format_pull_request_section(action: &str, pull_requests: &[Issue]) -> String {
    let pull_request_lines: Vec<String> =
        pull_requests.iter().map(format_pull_request_line).collect();

    format!("### {action}\n\n{}", pull_request_lines.join("\n"))
}

fn format_pull_request_line(pull_request: &Issue) -> String {
    let repository_name = get_repository_name_from_repository_url(&pull_request.repository_url);

    let repository_link = format!("[{repository_name}](https://github.com/{repository_name})");

    let pull_request_link =
        format!("[#{}]({})", pull_request.number, pull_request.pull_request.html_url);

    format!(r#"- "{}" ({pull_request_link}) in {repository_link}"#, pull_request.title)
}

#[cfg(test)]
mod tests;
