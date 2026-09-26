pub mod application;
mod credentials;
mod flow;
mod redirect;
mod token;

use ::credentials::store::CredentialStore;
use base_url::base_url::BaseUrl;
use miette::Result;

use crate::oauth::{
    application::GoogleOAuthApplication,
    credentials::{
        clear_google_credentials,
        get_current_epoch_seconds,
        read_google_credentials,
        write_google_credentials,
    },
    flow::authorize_in_browser,
    token::{
        refresh_access_token,
        structs::RefreshOutcome,
    },
};

pub const DEFAULT_GOOGLE_TOKEN_BASE_URL: &str = "https://oauth2.googleapis.com/";

pub struct GoogleAuthorization {
    application: GoogleOAuthApplication,
    token_base_url: BaseUrl,
    credential_store: CredentialStore,
}

impl GoogleAuthorization {
    #[must_use]
    pub const fn new(
        application: GoogleOAuthApplication,
        token_base_url: BaseUrl,
        credential_store: CredentialStore,
    ) -> Self {
        Self { application, token_base_url, credential_store }
    }

    pub async fn get_access_token_authorizing_if_needed(&self) -> Result<String> {
        let Some(google_credentials) = read_google_credentials(&self.credential_store)? else {
            let authorized_credentials =
                authorize_in_browser(&self.application, &self.token_base_url).await?;

            write_google_credentials(&self.credential_store, &authorized_credentials)?;

            return Ok(authorized_credentials.access_token);
        };

        let current_epoch_seconds = get_current_epoch_seconds();

        if !google_credentials.is_expiring(current_epoch_seconds) {
            return Ok(google_credentials.access_token);
        }

        let refresh_outcome = refresh_access_token(
            &self.token_base_url,
            &self.application,
            &google_credentials.refresh_token,
        )
        .await?;

        let refreshed_credentials = match refresh_outcome {
            RefreshOutcome::Refreshed(refreshed_credentials) => refreshed_credentials,
            RefreshOutcome::ReauthorizationRequired => {
                authorize_in_browser(&self.application, &self.token_base_url).await?
            }
        };

        write_google_credentials(&self.credential_store, &refreshed_credentials)?;

        Ok(refreshed_credentials.access_token)
    }

    pub fn clear(&self) -> Result<()> {
        clear_google_credentials(&self.credential_store)
    }
}
