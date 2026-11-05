use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use credentials::{
    credential::Credential,
    store::CredentialStore,
};
use miette::Result;
use serde::{
    Deserialize,
    Serialize,
};

use crate::errors::GoogleAuthorizationFailed;

// Refresh this many seconds before the access token expires, so a token cannot
// lapse between the expiry check and the request that uses it.
const REFRESH_BUFFER_SECONDS: u64 = 30;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,

    #[serde(default)]
    pub scopes: Vec<String>,
}

impl GoogleCredentials {
    pub const fn is_expiring(&self, now: u64) -> bool {
        self.expires_at
            .saturating_sub(now)
            < REFRESH_BUFFER_SECONDS
    }
}

pub fn read_google_credentials(
    credential_store: &CredentialStore,
) -> Result<Option<GoogleCredentials>> {
    let Some(google_credentials_json) = credential_store.read(Credential::GoogleCalendar)? else {
        return Ok(None);
    };

    let google_credentials = serde_json::from_str(&google_credentials_json).map_err(|error| {
        GoogleAuthorizationFailed {
            message: format!("stored Google credentials could not be read: {error}"),
        }
    })?;

    Ok(Some(google_credentials))
}

pub fn write_google_credentials(
    credential_store: &CredentialStore,
    google_credentials: &GoogleCredentials,
) -> Result<()> {
    let google_credentials_json =
        serde_json::to_string(google_credentials).map_err(|error| GoogleAuthorizationFailed {
            message: format!("Google credentials could not be stored: {error}"),
        })?;

    credential_store.write(Credential::GoogleCalendar, &google_credentials_json)
}

pub fn clear_google_credentials(credential_store: &CredentialStore) -> Result<()> {
    credential_store.clear(Credential::GoogleCalendar)?;

    Ok(())
}

pub fn get_current_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}
