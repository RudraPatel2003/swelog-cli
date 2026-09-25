mod structs;

use chrono::NaiveDate;
use futures::future::try_join_all;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use structs::Review;

use crate::{
    client::GitHubClient,
    issues::Issue,
    repository_name::get_repository_name_from_repository_url,
};

const PULL_REQUEST_REVIEWS_QUERY_PARAMETERS: [(&str, &str); 1] = [("per_page", "100")];

impl GitHubClient {
    pub async fn get_reviewed_prs(
        &self,
        github_username: &str,
        activity_date: &NaiveDate,
    ) -> Result<Vec<Issue>> {
        let search_query = get_reviewed_pr_candidates_search_query(github_username, *activity_date);

        let candidate_prs = self.search_issues(&search_query).await?;

        let candidate_pr_reviews = try_join_all(
            candidate_prs.iter().map(|candidate_pr| self.get_pull_request_reviews(candidate_pr)),
        )
        .await?;

        let candidate_prs_with_reviews = candidate_prs.into_iter().zip(candidate_pr_reviews);

        let reviewed_prs =
            keep_prs_reviewed_on(candidate_prs_with_reviews, github_username, *activity_date);

        Ok(reviewed_prs)
    }

    async fn get_pull_request_reviews(&self, pull_request: &Issue) -> Result<Vec<Review>> {
        let endpoint_path = get_pull_request_reviews_endpoint_path(pull_request);

        let response_text =
            self.get_json_text(&endpoint_path, &PULL_REQUEST_REVIEWS_QUERY_PARAMETERS).await?;

        parse_pull_request_reviews_response_text(&response_text)
    }
}

/// GitHub search cannot filter by when a review was submitted, and submitting a review updates the
/// pull request, so this finds every pull request that could have been reviewed on the date.
fn get_reviewed_pr_candidates_search_query(
    github_username: &str,
    activity_date: NaiveDate,
) -> String {
    format!(
        "reviewed-by:{github_username} -author:{github_username} is:pr updated:>={activity_date}"
    )
}

fn get_pull_request_reviews_endpoint_path(pull_request: &Issue) -> String {
    let repository_name = get_repository_name_from_repository_url(&pull_request.repository_url);

    format!("repos/{repository_name}/pulls/{}/reviews", pull_request.number)
}

fn parse_pull_request_reviews_response_text(response_text: &str) -> Result<Vec<Review>> {
    serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse GitHub pull request reviews response")
}

fn keep_prs_reviewed_on(
    candidate_prs_with_reviews: impl IntoIterator<Item = (Issue, Vec<Review>)>,
    github_username: &str,
    activity_date: NaiveDate,
) -> Vec<Issue> {
    candidate_prs_with_reviews
        .into_iter()
        .filter(|(_, reviews)| has_review_submitted_on(reviews, github_username, activity_date))
        .map(|(candidate_pr, _)| candidate_pr)
        .collect()
}

fn has_review_submitted_on(
    reviews: &[Review],
    github_username: &str,
    activity_date: NaiveDate,
) -> bool {
    reviews.iter().any(|review| {
        review.is_submitted_by(github_username) && review.is_submitted_on(activity_date)
    })
}

#[cfg(test)]
mod tests;
