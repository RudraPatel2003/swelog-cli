use std::time::Duration;

use base_url::base_url::BaseUrl;
use miette::Result;
use oauth::callback_server::CallbackServer;
use oauth2::{
    CsrfToken,
    PkceCodeChallenge,
};
use tokio::time::timeout;
use url::Url;

use crate::{
    errors::{
        GoogleAuthorizationFailed,
        GoogleAuthorizationTimedOut,
    },
    oauth::{
        application::GoogleOAuthApplication,
        credentials::GoogleCredentials,
        redirect::parse_authorization_code,
        token::exchange_authorization_code,
    },
};

const GOOGLE_AUTHORIZATION_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";

// The narrowest scope that reads calendar events, so swelog can never change
// or delete one.
const CALENDAR_SCOPE: &str = "https://www.googleapis.com/auth/calendar.events.readonly";

const AUTHORIZATION_TIMEOUT: Duration = Duration::from_secs(300);

const CALLBACK_COMPLETION_MESSAGE: &str =
    "Google authorization complete. You can close this window and return to swelog.";

pub async fn authorize_in_browser(
    application: &GoogleOAuthApplication,
    token_base_url: &BaseUrl,
) -> Result<GoogleCredentials> {
    let callback_server = CallbackServer::bind(CALLBACK_COMPLETION_MESSAGE).await?;

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let csrf_token = CsrfToken::new_random();

    let redirect_uri = callback_server.redirect_uri().to_string();

    let authorization_url = build_authorization_url(
        application,
        &redirect_uri,
        pkce_challenge.as_str(),
        csrf_token.secret(),
    )?;

    print_and_open_authorization_url(authorization_url.as_str());

    let callback_url = timeout(AUTHORIZATION_TIMEOUT, callback_server.receive_callback_url())
        .await
        .map_err(|_| GoogleAuthorizationTimedOut)??;

    let authorization_code = parse_authorization_code(&callback_url, csrf_token.secret())?;

    exchange_authorization_code(
        token_base_url,
        application,
        &authorization_code,
        &pkce_verifier,
        &redirect_uri,
    )
    .await
}

fn build_authorization_url(
    application: &GoogleOAuthApplication,
    redirect_uri: &str,
    pkce_challenge: &str,
    state: &str,
) -> Result<Url> {
    let mut authorization_url =
        Url::parse(GOOGLE_AUTHORIZATION_URL).map_err(|error| GoogleAuthorizationFailed {
            message: format!("the Google authorization URL could not be built: {error}"),
        })?;

    authorization_url
        .query_pairs_mut()
        .append_pair("client_id", &application.client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", CALENDAR_SCOPE)
        .append_pair("state", state)
        .append_pair("code_challenge", pkce_challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent");

    Ok(authorization_url)
}

fn print_and_open_authorization_url(authorization_url: &str) {
    println!("Authorize swelog with Google using this URL:\n{authorization_url}\n");

    if webbrowser::open(authorization_url).is_err() {
        println!("Unable to open a browser automatically. Open the URL above manually.");
    }
}

#[cfg(test)]
#[path = "flow_tests.rs"]
mod tests;
