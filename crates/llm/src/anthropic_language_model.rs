pub mod errors;
mod structs;

use async_trait::async_trait;
use base_url::base_url::BaseUrl;
use errors::{
    AnthropicAuthorizationFailed,
    AnthropicRequestFailed,
    AnthropicResponseMissingText,
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
    AnthropicMessagesRequest,
    AnthropicRequestMessage,
    AnthropicResponse,
};

use crate::language_model::LanguageModel;

pub const DEFAULT_ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com/";

const MESSAGES_ENDPOINT_PATH: &str = "v1/messages";

const ANTHROPIC_VERSION_HEADER: &str = "anthropic-version";

const ANTHROPIC_VERSION: &str = "2023-06-01";

const ANTHROPIC_API_KEY_HEADER: &str = "x-api-key";

const USER_ROLE: &str = "user";

// The Messages API requires max_tokens on every request and has no default.
const MAX_RESPONSE_TOKENS: u32 = 8192;

pub struct AnthropicLanguageModel {
    client: Client,
    base_url: BaseUrl,
    model: String,
    api_key: String,
}

impl AnthropicLanguageModel {
    #[must_use]
    pub fn new(base_url: BaseUrl, model: String, api_key: String) -> Self {
        Self { client: Client::new(), base_url, model, api_key }
    }
}

#[async_trait]
impl LanguageModel for AnthropicLanguageModel {
    async fn generate_response(&self, prompt: &str) -> Result<String> {
        let request = AnthropicMessagesRequest {
            model: self.model.clone(),
            max_tokens: MAX_RESPONSE_TOKENS,
            messages: vec![AnthropicRequestMessage {
                role: USER_ROLE,
                content: String::from(prompt),
            }],
        };

        let endpoint_url = self.base_url.join(MESSAGES_ENDPOINT_PATH)?;

        let response = self
            .client
            .post(endpoint_url)
            .header(ANTHROPIC_API_KEY_HEADER, self.api_key.as_str())
            .header(ANTHROPIC_VERSION_HEADER, ANTHROPIC_VERSION)
            .json(&request)
            .send()
            .await
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("failed to send Anthropic request for model {}", self.model)
            })?;

        let status = response.status();

        if matches!(status, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
            let anthropic_authorization_failed_error = AnthropicAuthorizationFailed { status };

            return Err(anthropic_authorization_failed_error.into());
        }

        if !status.is_success() {
            let anthropic_request_failed_error =
                AnthropicRequestFailed { model: self.model.clone(), status };

            return Err(anthropic_request_failed_error.into());
        }

        let response_body = response
            .text()
            .await
            .into_diagnostic()
            .wrap_err("failed to read Anthropic response body")?;

        parse_anthropic_response_text(&response_body, &self.model)
    }
}

fn parse_anthropic_response_text(response_body: &str, model: &str) -> Result<String> {
    let anthropic_response: AnthropicResponse = serde_json::from_str(response_body)
        .into_diagnostic()
        .wrap_err("failed to parse Anthropic response")?;

    let text_parts: Vec<&str> = anthropic_response
        .content
        .iter()
        .filter(|content| content.content_type == "text")
        .filter_map(|content| content.text.as_deref())
        .collect();

    if text_parts.is_empty() {
        let anthropic_response_missing_text_error =
            AnthropicResponseMissingText { model: model.to_string() };

        return Err(anthropic_response_missing_text_error.into());
    }

    Ok(text_parts.join(""))
}

#[cfg(test)]
#[path = "anthropic_language_model_tests.rs"]
mod tests;
