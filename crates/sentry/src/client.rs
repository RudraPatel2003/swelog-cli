use base_url::base_url::BaseUrl;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    header::USER_AGENT,
};
use serde::Serialize;

use crate::{
    errors::FailedToSendSentryRequest,
    response::{
        ResponsePage,
        read_successful_response_page,
    },
};

pub const DEFAULT_SENTRY_API_BASE_URL: &str = "https://sentry.io/api/0/";

const SWELOG_USER_AGENT: &str = "RudraPatel2003/swelog-cli";

pub struct SentryClient {
    http_client: Client,
    api_base_url: BaseUrl,
    token: String,
}

impl SentryClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, token: String) -> Self {
        Self { http_client: Client::new(), api_base_url, token }
    }

    pub(crate) async fn get_json_page(
        &self,
        endpoint_path: &str,
        query_parameters: &(impl Serialize + Sync),
    ) -> Result<ResponsePage> {
        let endpoint_url = self
            .api_base_url
            .join(endpoint_path)?;

        let response = self
            .http_client
            .get(endpoint_url)
            .query(query_parameters)
            .bearer_auth(&self.token)
            .header(USER_AGENT, SWELOG_USER_AGENT)
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| FailedToSendSentryRequest)?;

        read_successful_response_page(response).await
    }
}
