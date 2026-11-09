pub mod errors;
mod structs;

use async_trait::async_trait;
use base_url::base_url::BaseUrl;
use errors::OpenAiResponseMissingText;
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
    OpenAiResponse,
    OpenAiResponseRequest,
};

use crate::language_model::LanguageModel;

pub const DEFAULT_OPEN_AI_BASE_URL: &str = "https://api.openai.com/";

const RESPONSES_ENDPOINT_PATH: &str = "v1/responses";

const OPEN_AI_API_SERVICE: ApiService = ApiService {
    label: "OpenAI",
    credential: Some(CredentialDescription {
        name: "API key",
        command_name: "open-ai",
        authorization_hint: "check that the key is active in the OpenAI dashboard",
    }),
    default_headers: &[],
};

pub struct OpenAiLanguageModel {
    api_client: ApiClient,
    model: String,
    api_key: String,
}

impl OpenAiLanguageModel {
    #[must_use]
    pub fn new(base_url: BaseUrl, model: String, api_key: String) -> Self {
        Self { api_client: ApiClient::new(base_url, OPEN_AI_API_SERVICE), model, api_key }
    }
}

#[async_trait]
impl LanguageModel for OpenAiLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request =
            OpenAiResponseRequest { model: self.model.clone(), input: String::from(prompt) };

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

        parse_open_ai_response_text(&response_body, &self.model)
    }
}

fn parse_open_ai_response_text(response_body: &str, model: &str) -> Result<String> {
    let open_ai_response: OpenAiResponse = serde_json::from_str(response_body)
        .into_diagnostic()
        .wrap_err("failed to parse OpenAI response")?;

    let text_parts: Vec<&str> = open_ai_response
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
        let open_ai_response_missing_text_error =
            OpenAiResponseMissingText { model: model.to_string() };

        return Err(open_ai_response_missing_text_error.into());
    }

    Ok(text_parts.join(""))
}

#[cfg(test)]
#[path = "open_ai_language_model_tests.rs"]
mod tests;
