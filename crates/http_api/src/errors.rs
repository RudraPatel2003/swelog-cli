use miette::Diagnostic;
use reqwest::StatusCode;
use thiserror::Error;

use crate::api_service::CredentialDescription;

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send {service_label} request")]
#[diagnostic(
    code(swelog::http_api::failed_to_send_request),
    help("check your network connection and try again")
)]
pub struct FailedToSendApiRequest {
    pub service_label: &'static str,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Unsuccessful {service_label} response with status {status_code} and body {response_text}")]
#[diagnostic(code(swelog::http_api::unsuccessful_response))]
pub struct UnsuccessfulApiResponse {
    pub service_label: &'static str,
    pub status_code: StatusCode,
    pub response_text: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("{service_label} rejected your {credential_name} with status {status_code}")]
#[diagnostic(
    code(swelog::http_api::authorization_failed),
    help(
        "run `swelog auth clear {credential_command_name}` and run the command again to replace your {credential_name}, or {authorization_hint}"
    )
)]
pub struct ApiAuthorizationFailed {
    pub service_label: &'static str,
    pub credential_name: &'static str,
    pub credential_command_name: &'static str,
    pub authorization_hint: &'static str,
    pub status_code: StatusCode,
}

impl ApiAuthorizationFailed {
    #[must_use]
    pub const fn new(
        service_label: &'static str,
        credential: &CredentialDescription,
        status_code: StatusCode,
    ) -> Self {
        Self {
            service_label,
            credential_name: credential.name,
            credential_command_name: credential.command_name,
            authorization_hint: credential.authorization_hint,
            status_code,
        }
    }
}
