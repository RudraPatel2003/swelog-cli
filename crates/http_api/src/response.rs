use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Response,
    StatusCode,
};

use crate::{
    api_service::ApiService,
    errors::{
        ApiAuthorizationFailed,
        UnsuccessfulApiResponse,
    },
};

pub async fn read_successful_response_body(
    api_service: &ApiService,
    response: Response,
) -> Result<String> {
    let status_code = response.status();

    let response_text = response
        .text()
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {} response body", api_service.label))?;

    get_successful_response_text(api_service, status_code, response_text)
}

/// Instruct user to clear their credential if response is unauthorized or forbidden
fn get_successful_response_text(
    api_service: &ApiService,
    status_code: StatusCode,
    response_text: String,
) -> Result<String> {
    if let Some(credential) = &api_service.credential
        && matches!(status_code, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN)
    {
        let api_authorization_failed_error =
            ApiAuthorizationFailed::new(api_service.label, credential, status_code);

        return Err(api_authorization_failed_error.into());
    }

    if !status_code.is_success() {
        let unsuccessful_api_response_error = UnsuccessfulApiResponse {
            service_label: api_service.label,
            status_code,
            response_text,
        };

        return Err(unsuccessful_api_response_error.into());
    }

    Ok(response_text)
}

#[cfg(test)]
#[path = "response_tests.rs"]
mod tests;
