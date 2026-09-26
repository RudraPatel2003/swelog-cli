use config::swelog_config::{
    LanguageModelProvider,
    SwelogConfig,
};
use miette::Result;

use crate::errors::SummarizationNotConfigured;

pub struct SummarizationSettings {
    pub language_model_provider: LanguageModelProvider,
    pub language_model: String,
}

impl SummarizationSettings {
    pub fn from_config(swelog_config: &SwelogConfig) -> Result<Self> {
        let language_model_provider =
            swelog_config.language_model_provider.ok_or(SummarizationNotConfigured)?;

        let language_model =
            swelog_config.language_model.clone().ok_or(SummarizationNotConfigured)?;

        Ok(Self { language_model_provider, language_model })
    }
}
