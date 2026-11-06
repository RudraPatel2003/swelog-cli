use miette::Diagnostic;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Diagnostic, Error, PartialEq, Eq)]
#[error("{setting_label} is not configured")]
#[diagnostic(
    code(swelog::fetch::missing_integration_configuration),
    help("set {config_key} in your swelog config before running `swelog fetch {command_name}`")
)]
pub struct MissingIntegrationConfiguration {
    pub setting_label: &'static str,
    pub config_key: &'static str,
    pub command_name: &'static str,
}
