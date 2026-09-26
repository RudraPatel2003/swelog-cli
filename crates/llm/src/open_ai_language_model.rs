pub mod errors;
mod structs;

use async_trait::async_trait;
use base_url::base_url::BaseUrl;
use errors::{
    OpenAiAuthorizationFailed,
    OpenAiRequestFailed,
    OpenAiResponseMissingText,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Client,
    StatusCode,
};
use structs::{
    OpenAiResponse,
    OpenAiResponseRequest,
};

use crate::language_model::LanguageModel;

pub const DEFAULT_OPEN_AI_BASE_URL: &str = "https://api.openai.com/";

const RESPONSES_ENDPOINT_PATH: &str = "v1/responses";

pub struct OpenAiLanguageModel {
    client: Client,
    base_url: BaseUrl,
    model: String,
    api_key: String,
}

impl OpenAiLanguageModel {
    #[must_use]
    pub fn new(base_url: BaseUrl, model: String, api_key: String) -> Self {
        Self { client: Client::new(), base_url, model, api_key }
    }
}

#[async_trait]
impl LanguageModel for OpenAiLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request =
            OpenAiResponseRequest { model: self.model.clone(), input: String::from(prompt) };

        let endpoint_url = self.base_url.join(RESPONSES_ENDPOINT_PATH)?;

        let response = self
            .client
            .post(endpoint_url)
            .bearer_auth(self.api_key.clone())
            .json(&request)
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to send OpenAI request for model {}", self.model))?;

        let status = response.status();

        if matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
            let open_ai_authorization_failed_error = OpenAiAuthorizationFailed { status };

            return Err(open_ai_authorization_failed_error.into());
        }

        if !status.is_success() {
            let open_ai_request_failed_error =
                OpenAiRequestFailed { model: self.model.clone(), status };

            return Err(open_ai_request_failed_error.into());
        }

        let response_body = response
            .text()
            .await
            .into_diagnostic()
            .wrap_err("failed to read OpenAI response body")?;

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
        .flat_map(|output| output.content.iter())
        .filter(|content| content.content_type == "output_text")
        .filter_map(|content| content.text.as_deref())
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
