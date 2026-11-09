use std::time::Duration;

use base_url::base_url::BaseUrl;
use http_api::{
    api_service::ApiService,
    authentication::Authentication,
    client::ApiClient,
};
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use serde::Deserialize;

use crate::errors::FailedToFetchLatestVersion;

pub const DEFAULT_NPM_REGISTRY_BASE_URL: &str = "https://registry.npmjs.org/";

const LATEST_VERSION_ENDPOINT_PATH: &str = "swelog-cli/latest";

const VERSION_CHECK_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

const NO_QUERY_PARAMETERS: [(&str, &str); 0] = [];

const NPM_REGISTRY_API_SERVICE: ApiService =
    ApiService { label: "npm registry", credential: None, default_headers: &[] };

#[derive(Debug, Deserialize)]
struct NpmPackageManifest {
    version: String,
}

pub async fn fetch_latest_version(npm_registry_base_url: &BaseUrl) -> Result<String> {
    let api_client = ApiClient::new(npm_registry_base_url.clone(), NPM_REGISTRY_API_SERVICE)
        .with_request_timeout(VERSION_CHECK_REQUEST_TIMEOUT);

    let response = api_client
        .send_get_request(
            LATEST_VERSION_ENDPOINT_PATH,
            &NO_QUERY_PARAMETERS,
            Authentication::Anonymous,
        )
        .await
        .wrap_err(FailedToFetchLatestVersion)?;

    let response_text = api_client
        .read_successful_response_body(response)
        .await
        .wrap_err(FailedToFetchLatestVersion)?;

    parse_npm_package_manifest(&response_text)
}

fn parse_npm_package_manifest(response_text: &str) -> Result<String> {
    let npm_package_manifest: NpmPackageManifest = serde_json::from_str(response_text)
        .into_diagnostic()
        .wrap_err("failed to parse the npm package manifest")?;

    let latest_version = npm_package_manifest.version;

    Ok(latest_version)
}

#[cfg(test)]
#[path = "registry_tests.rs"]
mod tests;
