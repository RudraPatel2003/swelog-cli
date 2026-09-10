use std::path::PathBuf;

use highlight::stderr::path_link;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("{label} is not available")]
#[diagnostic(
    code(swelog::credentials::missing_credential),
    help(
        "run swelog from a terminal to enter it, or set the {environment_variable} environment variable. If a stored credential is stale, run `swelog auth clear {command_name}`."
    )
)]
pub struct MissingCredential {
    pub label: &'static str,
    pub environment_variable: &'static str,
    pub command_name: &'static str,
}

#[derive(Debug, Diagnostic, Error)]
#[error("{label} is not available")]
#[diagnostic(
    code(swelog::credentials::missing_authorization),
    help(
        "run `swelog auth clear {command_name}` and then `swelog fetch {command_name}` to authorize again."
    )
)]
pub struct MissingAuthorization {
    pub label: &'static str,
    pub command_name: &'static str,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to access the {label} credential in your operating system keyring")]
#[diagnostic(
    code(swelog::credentials::keyring_unavailable),
    help("unlock your keyring and try again: {message}")
)]
pub struct KeyringUnavailable {
    pub label: &'static str,
    pub message: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to access the credential file at {}", path_link(.credential_file))]
#[diagnostic(
    code(swelog::credentials::credential_file_unavailable),
    help("check that the file is readable JSON and try again: {message}")
)]
pub struct CredentialFileUnavailable {
    pub credential_file: PathBuf,
    pub message: String,
}

#[derive(Debug, Diagnostic, Error)]
#[error("`{value}` is not a credential store")]
#[diagnostic(
    code(swelog::credentials::invalid_credential_store),
    help("use `keyring` or `file:<path>`")
)]
pub struct InvalidCredentialStore {
    pub value: String,
}
