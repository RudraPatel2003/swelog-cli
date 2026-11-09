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

pub const DEFAULT_GITHUB_API_BASE_URL: &str = "https://api.github.com/";

const GITHUB_API_SERVICE: ApiService = ApiService {
    label: "GitHub",
    credential: Some(CredentialDescription {
        name: "token",
        command_name: "github",
        authorization_hint: "check that your token has the `repo` scope",
    }),
    default_headers: &[("accept", "application/vnd.github.v3+json")],
};

pub struct GitHubClient {
    api_client: ApiClient,
    token: String,
}

impl GitHubClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, token: String) -> Self {
        Self { api_client: ApiClient::new(api_base_url, GITHUB_API_SERVICE), token }
    }

    pub(crate) async fn get_json_text(
        &self,
        endpoint_path: &str,
        query_parameters: &(impl Serialize + Sync),
    ) -> Result<String> {
        let response = self
            .api_client
            .send_get_request(
                endpoint_path,
                query_parameters,
                Authentication::BearerToken(&self.token),
            )
            .await?;

        self.api_client
            .read_successful_response_body(response)
            .await
    }
}
