mod file;
mod keyring;

use std::{
    path::PathBuf,
    str::FromStr,
};

use highlight::stdout::path_link;
use miette::Result;

use crate::{
    credential::Credential,
    errors::InvalidCredentialStore,
    store::{
        file::{
            clear_file_credential,
            read_file_credential,
            write_file_credential,
        },
        keyring::{
            clear_keyring_credential,
            read_keyring_credential,
            write_keyring_credential,
        },
    },
};

const KEYRING_SELECTION: &str = "keyring";

const FILE_SELECTION_PREFIX: &str = "file:";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CredentialStore {
    Keyring,
    File(PathBuf),
}

impl CredentialStore {
    pub fn read(&self, credential: Credential) -> Result<Option<String>> {
        match self {
            Self::Keyring => read_keyring_credential(credential),

            Self::File(credential_file) => read_file_credential(credential_file, credential),
        }
    }

    pub fn write(&self, credential: Credential, secret: &str) -> Result<()> {
        match self {
            Self::Keyring => write_keyring_credential(credential, secret),

            Self::File(credential_file) => {
                write_file_credential(credential_file, credential, secret)
            }
        }
    }

    pub fn clear(&self, credential: Credential) -> Result<bool> {
        match self {
            Self::Keyring => clear_keyring_credential(credential),

            Self::File(credential_file) => clear_file_credential(credential_file, credential),
        }
    }

    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Keyring => String::from("your operating system keyring"),

            Self::File(credential_file) => {
                format!("the credential file at {}", path_link(credential_file))
            }
        }
    }
}

impl FromStr for CredentialStore {
    type Err = InvalidCredentialStore;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == KEYRING_SELECTION {
            return Ok(Self::Keyring);
        }

        if let Some(credential_file) = value.strip_prefix(FILE_SELECTION_PREFIX)
            && !credential_file.is_empty()
        {
            return Ok(Self::File(PathBuf::from(credential_file)));
        }

        Err(InvalidCredentialStore { value: value.to_string() })
    }
}

#[cfg(test)]
mod tests;
