use base_url::base_url::BaseUrl;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::{
        ACCEPT,
        USER_AGENT,
    },
};
use serde::Serialize;

use crate::{
    errors::FailedToSendGitHubRequest,
    repository_name::{
        GITHUB_ACCEPT_HEADER,
        SWELOG_USER_AGENT,
    },
    response::read_successful_response_body,
};

pub const DEFAULT_GITHUB_API_BASE_URL: &str = "https://api.github.com/";

pub struct GitHubClient {
    http_client: Client,
    api_base_url: BaseUrl,
    token: String,
}

impl GitHubClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, token: String) -> Self {
        Self { http_client: Client::new(), api_base_url, token }
    }

    pub(crate) async fn get_json_text(
        &self,
        endpoint_path: &str,
        query_parameters: &(impl Serialize + Sync),
    ) -> Result<String> {
        let endpoint_url = self
            .api_base_url
            .join(endpoint_path)?;

        let response = self
            .http_client
            .get(endpoint_url)
            .query(query_parameters)
            .bearer_auth(&self.token)
            .header(ACCEPT, GITHUB_ACCEPT_HEADER)
            .header(USER_AGENT, SWELOG_USER_AGENT)
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| FailedToSendGitHubRequest)?;

        read_successful_response_body(response).await
    }
}
