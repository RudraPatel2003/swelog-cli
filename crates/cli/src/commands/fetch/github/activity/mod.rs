use chrono::NaiveDate;
use github::{
    client::GitHubClient,
    issues::Issue,
};
use miette::Result;

pub struct GitHubActivity {
    pub opened: Vec<Issue>,

    pub merged: Vec<Issue>,

    pub closed: Vec<Issue>,

    pub reviewed: Vec<Issue>,
}

impl GitHubActivity {
    pub fn is_empty(&self) -> bool {
        self.get_pull_request_groups().iter().all(|pull_requests| pull_requests.is_empty())
    }

    pub fn count_pull_requests(&self) -> usize {
        self.get_pull_request_groups().iter().map(|pull_requests| pull_requests.len()).sum()
    }

    fn get_pull_request_groups(&self) -> [&[Issue]; 4] {
        [&self.opened, &self.merged, &self.closed, &self.reviewed]
    }
}

pub async fn get_github_activity(
    github_client: &GitHubClient,
    github_username: &str,
    activity_date: &NaiveDate,
) -> Result<GitHubActivity> {
    let get_opened_prs_future = github_client.get_opened_prs(github_username, activity_date);

    let get_merged_prs_future = github_client.get_merged_prs(github_username, activity_date);

    let get_closed_prs_future = github_client.get_closed_prs(github_username, activity_date);

    let get_reviewed_prs_future = github_client.get_reviewed_prs(github_username, activity_date);

    let (opened, merged, closed, reviewed) = tokio::try_join!(
        get_opened_prs_future,
        get_merged_prs_future,
        get_closed_prs_future,
        get_reviewed_prs_future
    )?;

    Ok(GitHubActivity { opened, merged, closed, reviewed })
}
