use base_url::base_url::BaseUrl;
use http_api::{
    api_service::{
        ApiService,
        CredentialDescription,
    },
    authentication::Authentication,
    client::ApiClient,
};
use miette::Result;
use serde::Serialize;

use crate::response::{
    ResponsePage,
    read_next_cursor,
};

pub const DEFAULT_SENTRY_API_BASE_URL: &str = "https://sentry.io/api/0/";

const SENTRY_API_SERVICE: ApiService = ApiService {
    label: "Sentry",
    credential: Some(CredentialDescription {
        name: "auth token",
        command_name: "sentry",
        authorization_hint: "check that your token has the `event:read` scope",
    }),
    default_headers: &[],
};

pub struct SentryClient {
    api_client: ApiClient,
    token: String,
}

impl SentryClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, token: String) -> Self {
        Self { api_client: ApiClient::new(api_base_url, SENTRY_API_SERVICE), token }
    }

    pub(crate) async fn get_json_page(
        &self,
        endpoint_path: &str,
        query_parameters: &(impl Serialize + Sync),
    ) -> Result<ResponsePage> {
        let response = self
            .api_client
            .send_get_request(
                endpoint_path,
                query_parameters,
                Authentication::BearerToken(&self.token),
            )
            .await?;

        let next_cursor = read_next_cursor(&response);

        let body = self
            .api_client
            .read_successful_response_body(response)
            .await?;

        Ok(ResponsePage { body, next_cursor })
    }
}
