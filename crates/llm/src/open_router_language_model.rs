pub mod errors;
mod structs;

use async_trait::async_trait;
use base_url::base_url::BaseUrl;
use errors::OpenRouterResponseMissingText;
use http_api::{
    api_service::{
        ApiService,
        CredentialDescription,
    },
    authentication::Authentication,
    client::ApiClient,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use structs::{
    OpenRouterResponse,
    OpenRouterResponseRequest,
};

use crate::language_model::LanguageModel;

pub const DEFAULT_OPEN_ROUTER_BASE_URL: &str = "https://openrouter.ai/api/";

const RESPONSES_ENDPOINT_PATH: &str = "v1/responses";

const OPEN_ROUTER_API_SERVICE: ApiService = ApiService {
    label: "OpenRouter",
    credential: Some(CredentialDescription {
        name: "API key",
        command_name: "open-router",
        authorization_hint: "check that the key is active in your OpenRouter settings",
    }),
    default_headers: &[],
};

pub struct OpenRouterLanguageModel {
    api_client: ApiClient,
    model: String,
    api_key: String,
}

impl OpenRouterLanguageModel {
    #[must_use]
    pub fn new(base_url: BaseUrl, model: String, api_key: String) -> Self {
        Self { api_client: ApiClient::new(base_url, OPEN_ROUTER_API_SERVICE), model, api_key }
    }
}

#[async_trait]
impl LanguageModel for OpenRouterLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request =
            OpenRouterResponseRequest { model: self.model.clone(), input: String::from(prompt) };

        let response = self
            .api_client
            .send_post_request(
                RESPONSES_ENDPOINT_PATH,
                &request,
                Authentication::BearerToken(&self.api_key),
            )
            .await?;

        let response_body = self
            .api_client
            .read_successful_response_body(response)
            .await?;

        parse_open_router_response_text(&response_body, &self.model)
    }
}

fn parse_open_router_response_text(response_body: &str, model: &str) -> Result<String> {
    let open_router_response: OpenRouterResponse = serde_json::from_str(response_body)
        .into_diagnostic()
        .wrap_err("failed to parse OpenRouter response")?;

    let text_parts: Vec<&str> = open_router_response
        .output
        .iter()
        .flat_map(|output| {
            output
                .content
                .iter()
        })
        .filter(|content| content.content_type == "output_text")
        .filter_map(|content| {
            content
                .text
                .as_deref()
        })
        .collect();

    if text_parts.is_empty() {
        let open_router_response_missing_text_error =
            OpenRouterResponseMissingText { model: model.to_string() };

        return Err(open_router_response_missing_text_error.into());
    }

    Ok(text_parts.join(""))
}

#[cfg(test)]
#[path = "open_router_language_model_tests.rs"]
mod tests;
