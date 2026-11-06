use keyring::{
    Entry,
    Error as KeyringError,
};
use miette::Result;

use crate::{
    credential::Credential,
    errors::KeyringUnavailable,
};

const KEYRING_SERVICE: &str = "swelog";

pub fn read_keyring_credential(credential: Credential) -> Result<Option<String>> {
    match keyring_entry(credential)?.get_password() {
        Ok(secret) => Ok(Some(secret)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(error) => Err(keyring_unavailable(credential, &error)),
    }
}

pub fn write_keyring_credential(credential: Credential, secret: &str) -> Result<()> {
    keyring_entry(credential)?
        .set_password(secret)
        .map_err(|error| keyring_unavailable(credential, &error))
}

pub fn clear_keyring_credential(credential: Credential) -> Result<bool> {
    match keyring_entry(credential)?.delete_credential() {
        Ok(()) => Ok(true),
        Err(KeyringError::NoEntry) => Ok(false),
        Err(error) => Err(keyring_unavailable(credential, &error)),
    }
}

fn keyring_entry(credential: Credential) -> Result<Entry> {
    Entry::new(
        KEYRING_SERVICE,
        credential
            .spec()
            .storage_key,
    )
    .map_err(|error| keyring_unavailable(credential, &error))
}

fn keyring_unavailable(credential: Credential, error: &KeyringError) -> miette::Report {
    KeyringUnavailable {
        label: credential
            .spec()
            .label,
        message: error.to_string(),
    }
    .into()
}
