pub mod structs;

use base_url::base_url::BaseUrl;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use oauth2::PkceCodeVerifier;
use reqwest::{
    Client,
    Url,
};

use crate::{
    errors::{
        FailedToSendGoogleCalendarRequest,
        GoogleAuthorizationFailed,
    },
    oauth::{
        application::GoogleOAuthApplication,
        credentials::{
            GoogleCredentials,
            get_current_epoch_seconds,
        },
        token::structs::{
            RefreshOutcome,
            TokenErrorResponse,
            TokenRequestOutcome,
            TokenResponse,
        },
    },
};

const TOKEN_ENDPOINT_PATH: &str = "token";

// The error Google returns when a refresh token has been revoked or expired.
const INVALID_GRANT_ERROR: &str = "invalid_grant";

pub async fn exchange_authorization_code(
    token_base_url: &BaseUrl,
    application: &GoogleOAuthApplication,
    authorization_code: &str,
    pkce_verifier: &PkceCodeVerifier,
    redirect_uri: &str,
) -> Result<GoogleCredentials> {
    let mut form = get_client_form_fields(application);

    form.push(("grant_type", "authorization_code".to_string()));

    form.push(("code", authorization_code.to_string()));

    form.push(("code_verifier", pkce_verifier.secret().clone()));

    form.push(("redirect_uri", redirect_uri.to_string()));

    let TokenRequestOutcome::Succeeded(response_text) =
        post_token_request(token_base_url, &form).await?
    else {
        let google_authorization_failed_error = GoogleAuthorizationFailed {
            message: "Google rejected the authorization code".to_string(),
        };

        return Err(google_authorization_failed_error.into());
    };

    let token_response = parse_token_response(&response_text)?;

    let refresh_token = token_response.refresh_token.clone().ok_or_else(|| {
        GoogleAuthorizationFailed { message: "Google did not return a refresh token".to_string() }
    })?;

    Ok(token_response.into_credentials(refresh_token, get_current_epoch_seconds()))
}

pub async fn refresh_access_token(
    token_base_url: &BaseUrl,
    application: &GoogleOAuthApplication,
    refresh_token: &str,
) -> Result<RefreshOutcome> {
    let mut form = get_client_form_fields(application);

    form.push(("grant_type", "refresh_token".to_string()));

    form.push(("refresh_token", refresh_token.to_string()));

    let TokenRequestOutcome::Succeeded(response_text) =
        post_token_request(token_base_url, &form).await?
    else {
        return Ok(RefreshOutcome::ReauthorizationRequired);
    };

    let token_response = parse_token_response(&response_text)?;

    // Google omits the refresh token when refreshing, so the existing one has to
    // be carried forward rather than dropped.
    let refresh_token_to_store = get_refresh_token_to_store(&token_response, refresh_token);

    let google_credentials =
        token_response.into_credentials(refresh_token_to_store, get_current_epoch_seconds());

    Ok(RefreshOutcome::Refreshed(google_credentials))
}

fn get_refresh_token_to_store(token_response: &TokenResponse, refresh_token: &str) -> String {
    token_response.refresh_token.clone().unwrap_or_else(|| refresh_token.to_string())
}

fn get_client_form_fields(application: &GoogleOAuthApplication) -> Vec<(&'static str, String)> {
    vec![
        ("client_id", application.client_id.clone()),
        ("client_secret", application.client_secret.clone()),
    ]
}

async fn post_token_request(
    token_base_url: &BaseUrl,
    form: &[(&'static str, String)],
) -> Result<TokenRequestOutcome> {
    let token_url: Url = token_base_url.join(TOKEN_ENDPOINT_PATH)?;

    let response = Client::new()
        .post(token_url)
        .form(form)
        .send()
        .await
        .into_diagnostic()
        .wrap_err_with(|| FailedToSendGoogleCalendarRequest)?;

    let status_code = response.status();

    let response_text = response.text().await.map_err(|error| GoogleAuthorizationFailed {
        message: format!("failed to read the Google token response: {error}"),
    })?;

    if status_code.is_success() {
        return Ok(TokenRequestOutcome::Succeeded(response_text));
    }

    let token_error = parse_token_error_response(&response_text);

    if token_error.as_ref().is_some_and(|token_error| token_error.error == INVALID_GRANT_ERROR) {
        return Ok(TokenRequestOutcome::InvalidGrant);
    }

    let google_authorization_failed_error = GoogleAuthorizationFailed {
        message: describe_token_error(token_error.as_ref(), &response_text),
    };

    Err(google_authorization_failed_error.into())
}

fn describe_token_error(token_error: Option<&TokenErrorResponse>, response_text: &str) -> String {
    let Some(token_error) = token_error else {
        return response_text.to_string();
    };

    token_error.error_description.as_ref().map_or_else(
        || token_error.error.clone(),
        |error_description| format!("{} ({error_description})", token_error.error),
    )
}

fn parse_token_error_response(response_text: &str) -> Option<TokenErrorResponse> {
    serde_json::from_str(response_text).ok()
}

fn parse_token_response(response_text: &str) -> Result<TokenResponse> {
    serde_json::from_str(response_text).map_err(|error| {
        GoogleAuthorizationFailed {
            message: format!("the Google token response did not match the expected shape: {error}"),
        }
        .into()
    })
}

#[cfg(test)]
#[path = "token_tests.rs"]
mod tests;
