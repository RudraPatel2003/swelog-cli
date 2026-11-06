use std::{
    collections::BTreeMap,
    fs,
    io::ErrorKind,
    path::Path,
};

use miette::{
    IntoDiagnostic,
    Result,
};

use crate::{
    credential::Credential,
    errors::CredentialFileUnavailable,
};

type StoredSecrets = BTreeMap<String, String>;

pub fn read_file_credential(
    credential_file: &Path,
    credential: Credential,
) -> Result<Option<String>> {
    let stored_secrets = read_stored_secrets(credential_file)?;

    let storage_key = credential
        .spec()
        .storage_key;

    let secret = stored_secrets
        .get(storage_key)
        .cloned();

    Ok(secret)
}

pub fn write_file_credential(
    credential_file: &Path,
    credential: Credential,
    secret: &str,
) -> Result<()> {
    let mut stored_secrets = read_stored_secrets(credential_file)?;

    stored_secrets.insert(
        credential
            .spec()
            .storage_key
            .to_string(),
        secret.to_string(),
    );

    write_stored_secrets(credential_file, &stored_secrets)
}

pub fn clear_file_credential(credential_file: &Path, credential: Credential) -> Result<bool> {
    let mut stored_secrets = read_stored_secrets(credential_file)?;

    let was_stored = stored_secrets
        .remove(
            credential
                .spec()
                .storage_key,
        )
        .is_some();

    if was_stored {
        write_stored_secrets(credential_file, &stored_secrets)?;
    }

    Ok(was_stored)
}

fn read_stored_secrets(credential_file: &Path) -> Result<StoredSecrets> {
    let credential_file_contents = match fs::read_to_string(credential_file) {
        Ok(credential_file_contents) => credential_file_contents,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(StoredSecrets::new()),
        Err(error) => {
            return Err(CredentialFileUnavailable {
                credential_file: credential_file.to_path_buf(),
                message: error.to_string(),
            })
            .into_diagnostic();
        }
    };

    serde_json::from_str(&credential_file_contents)
        .map_err(|error| CredentialFileUnavailable {
            credential_file: credential_file.to_path_buf(),
            message: error.to_string(),
        })
        .into_diagnostic()
}

fn write_stored_secrets(credential_file: &Path, stored_secrets: &StoredSecrets) -> Result<()> {
    if let Some(parent) = credential_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| CredentialFileUnavailable {
                credential_file: credential_file.to_path_buf(),
                message: error.to_string(),
            })
            .into_diagnostic()?;
    }

    let json = serde_json::to_string_pretty(stored_secrets)
        .map_err(|error| CredentialFileUnavailable {
            credential_file: credential_file.to_path_buf(),
            message: error.to_string(),
        })
        .into_diagnostic()?;

    fs::write(credential_file, json)
        .map_err(|error| CredentialFileUnavailable {
            credential_file: credential_file.to_path_buf(),
            message: error.to_string(),
        })
        .into_diagnostic()
}
