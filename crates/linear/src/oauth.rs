mod credential_store;

use std::time::Duration;

use credentials::{
    credential::Credential,
    store::CredentialStore,
};
use miette::Result;
use oauth::callback_server::CallbackServer;
use rmcp::transport::{
    AuthorizationManager,
    AuthorizationRequest,
    AuthorizationSession,
};
use tokio::time::timeout;

use crate::{
    errors::{
        LinearAuthorizationFailed,
        LinearAuthorizationTimedOut,
    },
    oauth::credential_store::SwelogCredentialStore,
};

const AUTHORIZATION_TIMEOUT: Duration = Duration::from_secs(300);

const CALLBACK_COMPLETION_MESSAGE: &str =
    "Linear authorization complete. You can close this window and return to swelog.";

pub async fn get_authorization_manager(
    mcp_url: &str,
    credential_store: &CredentialStore,
) -> Result<AuthorizationManager> {
    let mut authorization_manager = AuthorizationManager::new(mcp_url)
        .await
        .map_err(|error| LinearAuthorizationFailed { message: error.to_string() })?;

    authorization_manager
        .set_credential_store(SwelogCredentialStore::new(credential_store.clone()));

    if authorization_manager
        .initialize_from_store()
        .await
        .map_err(|error| LinearAuthorizationFailed { message: error.to_string() })?
    {
        return Ok(authorization_manager);
    }

    authorize_interactively(authorization_manager).await
}

pub fn clear_linear_authorization(credential_store: &CredentialStore) -> Result<()> {
    credential_store.clear(Credential::Linear)?;

    Ok(())
}

async fn authorize_interactively(
    mut authorization_manager: AuthorizationManager,
) -> Result<AuthorizationManager> {
    let callback_server = CallbackServer::bind(CALLBACK_COMPLETION_MESSAGE).await?;

    let metadata = authorization_manager
        .resolve_metadata()
        .await
        .map_err(|error| LinearAuthorizationFailed { message: error.to_string() })?
        .metadata;

    authorization_manager.set_metadata(metadata);

    let authorization_request =
        AuthorizationRequest::new(callback_server.redirect_uri()).with_client_name("swelog");

    let authorization_session =
        AuthorizationSession::new(authorization_manager, authorization_request)
            .await
            .map_err(|(_, error)| LinearAuthorizationFailed { message: error.to_string() })?;

    let authorization_url = authorization_session.get_authorization_url();

    println!("Authorize swelog with Linear using this URL:\n{authorization_url}\n");

    if webbrowser::open(authorization_url).is_err() {
        println!("Unable to open a browser automatically. Open the URL above manually.");
    }

    let callback_url = timeout(AUTHORIZATION_TIMEOUT, callback_server.receive_callback_url())
        .await
        .map_err(|_| LinearAuthorizationTimedOut)??;

    authorization_session
        .handle_callback_url(&callback_url)
        .await
        .map_err(|error| LinearAuthorizationFailed { message: error.to_string() })?;

    Ok(authorization_session.auth_manager)
}
