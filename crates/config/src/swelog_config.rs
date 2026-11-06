use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

use crate::integrations_config::IntegrationsConfig;

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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SwelogConfig {
    pub obsidian_vault_path: PathBuf,
    pub swelog_folder_name: String,
    pub work_file_name: String,
    pub daily_log_folder_name: String,
    pub weekly_log_folder_name: String,

    #[serde(rename = "llm")]
    pub language_model_provider: Option<LanguageModelProvider>,

    #[serde(rename = "llmModel")]
    pub language_model: Option<String>,

    #[serde(default, skip_serializing_if = "IntegrationsConfig::is_empty")]
    pub integrations: IntegrationsConfig,
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
            integrations: IntegrationsConfig::default(),
        }
    }
}
