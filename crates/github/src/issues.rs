mod structs;

use chrono::NaiveDate;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use structs::SearchIssuesResponse;
pub use structs::{
    Issue,
    PullRequest,
};

use crate::client::GitHubClient;

const SEARCH_ISSUES_ENDPOINT_PATH: &str = "search/issues";

impl GitHubClient {
    pub async fn get_merged_prs(
        &self,
        github_username: &str,
        activity_date: &NaiveDate,
    ) -> Result<Vec<Issue>> {
        let search_query = get_merged_prs_search_query(github_username, *activity_date);

        self.search_issues(&search_query).await
    }

    pub async fn get_opened_prs(
        &self,
        github_username: &str,
        activity_date: &NaiveDate,
    ) -> Result<Vec<Issue>> {
        let search_query = get_opened_prs_search_query(github_username, *activity_date);

        self.search_issues(&search_query).await
    }

    pub async fn get_closed_prs(
        &self,
        github_username: &str,
        activity_date: &NaiveDate,
    ) -> Result<Vec<Issue>> {
        let search_query = get_closed_prs_search_query(github_username, *activity_date);

        self.search_issues(&search_query).await
    }

    pub(crate) async fn search_issues(&self, search_query: &str) -> Result<Vec<Issue>> {
        let query_parameters = get_search_query_parameters(search_query);

        let response_text =
            self.get_json_text(SEARCH_ISSUES_ENDPOINT_PATH, &query_parameters).await?;

        parse_search_issues_response_text(&response_text)
    }
}

const fn get_search_query_parameters(search_query: &str) -> [(&'static str, &str); 4] {
    [("q", search_query), ("sort", "updated"), ("order", "desc"), ("per_page", "100")]
}

fn get_merged_prs_search_query(github_username: &str, activity_date: NaiveDate) -> String {
    format!("author:{github_username} is:pr merged:{activity_date}")
}

fn get_opened_prs_search_query(github_username: &str, activity_date: NaiveDate) -> String {
    format!("author:{github_username} is:pr created:{activity_date}")
}

fn get_closed_prs_search_query(github_username: &str, activity_date: NaiveDate) -> String {
    format!("author:{github_username} is:pr is:unmerged closed:{activity_date}")
}

fn parse_search_issues_response_text(response_text: &str) -> Result<Vec<Issue>> {
    let search_issues_response: SearchIssuesResponse = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse GitHub search issues response")?;

    Ok(search_issues_response.items)
}

#[cfg(test)]
#[path = "issues_tests.rs"]
mod tests;
