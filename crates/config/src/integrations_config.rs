use serde::{
    Deserialize,
    Serialize,
};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IntegrationsConfig {
    pub linear: Option<LinearConfig>,

    pub sentry: Option<SentryConfig>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinearConfig {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SentryConfig {
    pub organization: String,
}

impl IntegrationsConfig {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.linear
            .is_none()
            && self
                .sentry
                .is_none()
    }

    #[must_use]
    pub fn get_linear_username(&self) -> Option<&str> {
        let linear_config = self
            .linear
            .as_ref()?;

        get_non_blank_value(&linear_config.username)
    }

    #[must_use]
    pub fn get_sentry_organization(&self) -> Option<&str> {
        let sentry_config = self
            .sentry
            .as_ref()?;

        get_non_blank_value(&sentry_config.organization)
    }
}

fn get_non_blank_value(value: &str) -> Option<&str> {
    let trimmed_value = value.trim();

    if trimmed_value.is_empty() {
        return None;
    }

    Some(trimmed_value)
}

#[cfg(test)]
#[path = "integrations_config_tests.rs"]
mod tests;
