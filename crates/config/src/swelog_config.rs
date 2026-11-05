use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};

const DEFAULT_OBSIDIAN_VAULT_PATH: &str = "";

pub const DEFAULT_SWELOG_FOLDER_NAME: &str = "swelog";

pub const DEFAULT_DAILY_LOG_FOLDER_NAME: &str = "Daily";

pub const DEFAULT_WEEKLY_LOG_FOLDER_NAME: &str = "Weekly";

pub const DEFAULT_WORK_FILE_NAME: &str = "WORK.md";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LanguageModelProvider {
    Anthropic,
    Ollama,
    OpenAi,
    OpenRouter,
}

impl LanguageModelProvider {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Anthropic => "Anthropic",
            Self::Ollama => "Ollama",
            Self::OpenAi => "OpenAI",
            Self::OpenRouter => "OpenRouter",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder_name: String,
    pub work_file_name: String,
    pub daily_log_folder_name: String,
    pub weekly_log_folder_name: String,

    #[serde(default, rename = "llm", skip_serializing_if = "Option::is_none")]
    pub language_model_provider: Option<LanguageModelProvider>,

    #[serde(default, rename = "llmModel", skip_serializing_if = "Option::is_none")]
    pub language_model: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_username: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sentry_organization: Option<String>,
}

impl SwelogConfig {
    #[must_use]
    pub fn get_default_config() -> Self {
        Self {
            obsidian_vault_path: PathBuf::from(DEFAULT_OBSIDIAN_VAULT_PATH),
            swelog_folder_name: String::from(DEFAULT_SWELOG_FOLDER_NAME),
            work_file_name: String::from(DEFAULT_WORK_FILE_NAME),
            daily_log_folder_name: String::from(DEFAULT_DAILY_LOG_FOLDER_NAME),
            weekly_log_folder_name: String::from(DEFAULT_WEEKLY_LOG_FOLDER_NAME),
            language_model_provider: None,
            language_model: None,
            linear_username: None,
            sentry_organization: None,
        }
    }

    #[must_use]
    pub fn get_linear_username(&self) -> Option<&str> {
        self.linear_username
            .as_deref()
            .map(str::trim)
            .filter(|linear_username| !linear_username.is_empty())
    }

    #[must_use]
    pub fn get_sentry_organization(&self) -> Option<&str> {
        self.sentry_organization
            .as_deref()
            .map(str::trim)
            .filter(|sentry_organization| !sentry_organization.is_empty())
    }
}
