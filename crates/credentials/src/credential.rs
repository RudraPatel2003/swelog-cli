use clap::ValueEnum;

/// A secret swelog stores in the operating system keyring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Credential {
    Github,
    OpenAi,
    OpenRouter,
    Anthropic,
    Linear,
    GoogleCalendar,
    Sentry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialKind {
    Secret { environment_variable: &'static str, prompt_instructions: &'static str },

    OAuth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CredentialSpec {
    pub label: &'static str,
    pub command_name: &'static str,
    pub(crate) storage_key: &'static str,
    pub kind: CredentialKind,
}

impl CredentialSpec {
    #[must_use]
    pub const fn environment_variable(&self) -> Option<&'static str> {
        match self.kind {
            CredentialKind::Secret { environment_variable, .. } => Some(environment_variable),

            CredentialKind::OAuth => None,
        }
    }
}

impl Credential {
    #[must_use]
    pub fn all() -> &'static [Self] {
        Self::value_variants()
    }

    #[must_use]
    pub const fn spec(self) -> CredentialSpec {
        match self {
            Self::Github => CredentialSpec {
                label: "GitHub token",
                command_name: "github",
                storage_key: "github-token",
                kind: CredentialKind::Secret {
                    environment_variable: "GITHUB_TOKEN",
                    prompt_instructions: "Create one at https://github.com/settings/tokens with the `repo` scope.",
                },
            },

            Self::OpenAi => CredentialSpec {
                label: "OpenAI API key",
                command_name: "open-ai",
                storage_key: "openai-api-key",
                kind: CredentialKind::Secret {
                    environment_variable: "OPENAI_API_KEY",
                    prompt_instructions: "Create one at https://platform.openai.com/api-keys.",
                },
            },

            Self::OpenRouter => CredentialSpec {
                label: "OpenRouter API key",
                command_name: "open-router",
                storage_key: "openrouter-api-key",
                kind: CredentialKind::Secret {
                    environment_variable: "OPENROUTER_API_KEY",
                    prompt_instructions: "Create one at https://openrouter.ai/keys.",
                },
            },

            Self::Anthropic => CredentialSpec {
                label: "Anthropic API key",
                command_name: "anthropic",
                storage_key: "anthropic-api-key",
                kind: CredentialKind::Secret {
                    environment_variable: "ANTHROPIC_API_KEY",
                    prompt_instructions: "Create one at https://console.anthropic.com/settings/keys.",
                },
            },

            Self::Linear => CredentialSpec {
                label: "Linear authorization",
                command_name: "linear",
                storage_key: "linear-oauth",
                kind: CredentialKind::OAuth,
            },

            Self::GoogleCalendar => CredentialSpec {
                label: "Google Calendar authorization",
                command_name: "google-calendar",
                storage_key: "google-calendar-oauth",
                kind: CredentialKind::OAuth,
            },

            Self::Sentry => CredentialSpec {
                label: "Sentry auth token",
                command_name: "sentry",
                storage_key: "sentry-auth-token",
                kind: CredentialKind::Secret {
                    environment_variable: "SENTRY_AUTH_TOKEN",
                    prompt_instructions: "Create one at https://sentry.io/settings/account/api/auth-tokens/ with the `event:read` scope.",
                },
            },
        }
    }
}

#[cfg(test)]
#[path = "credential_tests.rs"]
mod tests;
