use base_url::base_url::BaseUrl;
use clap::Args;
use github::client::DEFAULT_GITHUB_API_BASE_URL;
use google_calendar::{
    client::DEFAULT_GOOGLE_CALENDAR_API_BASE_URL,
    oauth::DEFAULT_GOOGLE_TOKEN_BASE_URL,
};
use linear::client::DEFAULT_LINEAR_MCP_URL;
use llm::{
    anthropic_language_model::DEFAULT_ANTHROPIC_BASE_URL,
    language_model_endpoints::LanguageModelEndpoints,
    ollama_language_model::DEFAULT_OLLAMA_BASE_URL,
    open_ai_language_model::DEFAULT_OPEN_AI_BASE_URL,
    open_router_language_model::DEFAULT_OPEN_ROUTER_BASE_URL,
};
use sentry::client::DEFAULT_SENTRY_API_BASE_URL;
use updates::registry::DEFAULT_NPM_REGISTRY_BASE_URL;
use url::Url;

/// URLs for external services. Overridable for testing.
#[derive(Debug, Args)]
pub struct ServiceEndpoints {
    #[arg(
        long = "github-api-url",
        global = true,
        env = "SWELOG_GITHUB_API_URL",
        default_value = DEFAULT_GITHUB_API_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub github_api: BaseUrl,

    #[arg(
        long = "anthropic-api-url",
        global = true,
        env = "SWELOG_ANTHROPIC_API_URL",
        default_value = DEFAULT_ANTHROPIC_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub anthropic: BaseUrl,

    #[arg(
        long = "ollama-url",
        global = true,
        env = "SWELOG_OLLAMA_URL",
        default_value = DEFAULT_OLLAMA_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub ollama: BaseUrl,

    #[arg(
        long = "openai-api-url",
        global = true,
        env = "SWELOG_OPENAI_API_URL",
        default_value = DEFAULT_OPEN_AI_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub open_ai: BaseUrl,

    #[arg(
        long = "openrouter-api-url",
        global = true,
        env = "SWELOG_OPENROUTER_API_URL",
        default_value = DEFAULT_OPEN_ROUTER_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub open_router: BaseUrl,

    #[arg(
        long = "google-token-url",
        global = true,
        env = "SWELOG_GOOGLE_TOKEN_URL",
        default_value = DEFAULT_GOOGLE_TOKEN_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub google_token: BaseUrl,

    #[arg(
        long = "google-calendar-api-url",
        global = true,
        env = "SWELOG_GOOGLE_CALENDAR_API_URL",
        default_value = DEFAULT_GOOGLE_CALENDAR_API_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub google_calendar_api: BaseUrl,

    #[arg(
        long = "linear-mcp-url",
        global = true,
        env = "SWELOG_LINEAR_MCP_URL",
        default_value = DEFAULT_LINEAR_MCP_URL,
        value_name = "URL",
        hide = true
    )]
    pub linear_mcp: Url,

    #[arg(
        long = "sentry-api-url",
        global = true,
        env = "SWELOG_SENTRY_API_URL",
        default_value = DEFAULT_SENTRY_API_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub sentry_api: BaseUrl,

    #[arg(
        long = "npm-registry-url",
        global = true,
        env = "SWELOG_NPM_REGISTRY_URL",
        default_value = DEFAULT_NPM_REGISTRY_BASE_URL,
        value_name = "URL",
        hide = true
    )]
    pub npm_registry: BaseUrl,
}

impl ServiceEndpoints {
    #[must_use]
    pub fn language_model_endpoints(&self) -> LanguageModelEndpoints {
        LanguageModelEndpoints {
            anthropic_base_url: self
                .anthropic
                .clone(),
            ollama_base_url: self.ollama.clone(),
            open_ai_base_url: self.open_ai.clone(),
            open_router_base_url: self
                .open_router
                .clone(),
        }
    }
}
