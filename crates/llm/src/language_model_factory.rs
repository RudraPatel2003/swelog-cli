use config::swelog_config::LanguageModelProvider;
use credentials::{
    credential::Credential,
    resolution::get_or_prompt_for_credential,
    store::CredentialStore,
};
use miette::Result;

use crate::{
    anthropic_language_model::AnthropicLanguageModel,
    language_model::LanguageModel,
    language_model_endpoints::LanguageModelEndpoints,
    ollama_language_model::OllamaLanguageModel,
    open_ai_language_model::OpenAiLanguageModel,
    open_router_language_model::OpenRouterLanguageModel,
    summarization_settings::SummarizationSettings,
};

pub fn get_language_model(
    summarization_settings: &SummarizationSettings,
    endpoints: &LanguageModelEndpoints,
    credential_store: &CredentialStore,
) -> Result<Box<dyn LanguageModel>> {
    let language_model = summarization_settings
        .language_model
        .clone();

    match summarization_settings.language_model_provider {
        LanguageModelProvider::Anthropic => {
            let anthropic_api_key =
                get_or_prompt_for_credential(credential_store, Credential::Anthropic)?;

            let anthropic_language_model = AnthropicLanguageModel::new(
                endpoints
                    .anthropic_base_url
                    .clone(),
                language_model,
                anthropic_api_key,
            );

            Ok(Box::new(anthropic_language_model))
        }

        LanguageModelProvider::Ollama => {
            let ollama_language_model =
                OllamaLanguageModel::new(&endpoints.ollama_base_url, language_model)?;

            Ok(Box::new(ollama_language_model))
        }

        LanguageModelProvider::OpenAi => {
            let open_ai_api_key =
                get_or_prompt_for_credential(credential_store, Credential::OpenAi)?;

            let open_ai_language_model = OpenAiLanguageModel::new(
                endpoints
                    .open_ai_base_url
                    .clone(),
                language_model,
                open_ai_api_key,
            );

            Ok(Box::new(open_ai_language_model))
        }

        LanguageModelProvider::OpenRouter => {
            let open_router_api_key =
                get_or_prompt_for_credential(credential_store, Credential::OpenRouter)?;

            let open_router_language_model = OpenRouterLanguageModel::new(
                endpoints
                    .open_router_base_url
                    .clone(),
                language_model,
                open_router_api_key,
            );

            Ok(Box::new(open_router_language_model))
        }
    }
}
