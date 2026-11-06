use base_url::base_url::BaseUrl;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    RequestBuilder,
    Response,
    header::{
        ACCEPT,
        USER_AGENT,
    },
};
use serde::Serialize;

use crate::{
    api_service::ApiService,
    errors::FailedToSendApiRequest,
    response::read_successful_response_body,
    user_agent::SWELOG_USER_AGENT,
};

pub struct ApiClient {
    http_client: Client,
    api_base_url: BaseUrl,
    api_service: ApiService,
}

impl ApiClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, api_service: ApiService) -> Self {
        Self { http_client: Client::new(), api_base_url, api_service }
    }

    pub async fn send_get_request(
        &self,
        endpoint_path: &str,
        query_parameters: &(impl Serialize + Sync),
        bearer_token: &str,
    ) -> Result<Response> {
        let endpoint_url = self
            .api_base_url
            .join(endpoint_path)?;

        let request = self
            .http_client
            .get(endpoint_url)
            .query(query_parameters)
            .bearer_auth(bearer_token)
            .header(USER_AGENT, SWELOG_USER_AGENT);

        add_accept_header(
            request,
            self.api_service
                .accept_header,
        )
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendApiRequest {
            service_label: self
                .api_service
                .label,
        })
    }

    pub async fn read_successful_response_body(&self, response: Response) -> Result<String> {
        read_successful_response_body(&self.api_service, response).await
    }
}

fn add_accept_header(request: RequestBuilder, accept_header: Option<&str>) -> RequestBuilder {
    match accept_header {
        Some(accept_header) => request.header(ACCEPT, accept_header),

        None => request,
    }
}
