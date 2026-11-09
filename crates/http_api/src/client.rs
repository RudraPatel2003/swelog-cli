use std::time::Duration;

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
    header::USER_AGENT,
};
use serde::Serialize;

use crate::{
    api_service::ApiService,
    authentication::Authentication,
    errors::FailedToSendApiRequest,
    response::read_successful_response_body,
    user_agent::SWELOG_USER_AGENT,
};

pub struct ApiClient {
    http_client: Client,
    api_base_url: BaseUrl,
    api_service: ApiService,
    request_timeout: Option<Duration>,
}

impl ApiClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, api_service: ApiService) -> Self {
        Self { http_client: Client::new(), api_base_url, api_service, request_timeout: None }
    }

    #[must_use]
    pub fn with_request_timeout(self, request_timeout: Duration) -> Self {
        Self { request_timeout: Some(request_timeout), ..self }
    }

    pub async fn send_get_request(
        &self,
        endpoint_path: &str,
        query_parameters: &(impl Serialize + Sync),
        authentication: Authentication<'_>,
    ) -> Result<Response> {
        let endpoint_url = self
            .api_base_url
            .join(endpoint_path)?;

        let request = self
            .http_client
            .get(endpoint_url)
            .query(query_parameters);

        self.send_request(request, authentication)
            .await
    }

    pub async fn send_post_request(
        &self,
        endpoint_path: &str,
        json_body: &(impl Serialize + Sync),
        authentication: Authentication<'_>,
    ) -> Result<Response> {
        let endpoint_url = self
            .api_base_url
            .join(endpoint_path)?;

        let request = self
            .http_client
            .post(endpoint_url)
            .json(json_body);

        self.send_request(request, authentication)
            .await
    }

    pub async fn read_successful_response_body(&self, response: Response) -> Result<String> {
        read_successful_response_body(&self.api_service, response).await
    }

    async fn send_request(
        &self,
        request: RequestBuilder,
        authentication: Authentication<'_>,
    ) -> Result<Response> {
        let request = request.header(USER_AGENT, SWELOG_USER_AGENT);

        let request = add_default_headers(
            request,
            self.api_service
                .default_headers,
        );

        let request = add_authentication(request, authentication);

        let request = add_request_timeout(request, self.request_timeout);

        request
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| FailedToSendApiRequest {
                service_label: self
                    .api_service
                    .label,
            })
    }
}

fn add_default_headers(
    request: RequestBuilder,
    default_headers: &[(&'static str, &'static str)],
) -> RequestBuilder {
    default_headers
        .iter()
        .fold(request, |request, (header_name, header_value)| {
            request.header(*header_name, *header_value)
        })
}

fn add_authentication(request: RequestBuilder, authentication: Authentication) -> RequestBuilder {
    match authentication {
        Authentication::Anonymous => request,

        Authentication::BearerToken(bearer_token) => request.bearer_auth(bearer_token),

        Authentication::ApiKeyHeader { header_name, api_key } => {
            request.header(header_name, api_key)
        }
    }
}

fn add_request_timeout(
    request: RequestBuilder,
    request_timeout: Option<Duration>,
) -> RequestBuilder {
    match request_timeout {
        Some(request_timeout) => request.timeout(request_timeout),

        None => request,
    }
}

#[cfg(test)]
#[path = "client_tests.rs"]
mod tests;
