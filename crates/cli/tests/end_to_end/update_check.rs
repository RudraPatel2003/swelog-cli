use httpmock::{
    Method::GET,
    Mock,
    MockServer,
};
use predicates::{
    prelude::PredicateBooleanExt,
    str::contains,
};
use updates::cache::get_version_cache_file_path;

use crate::support::sandbox::SwelogSandbox;

const NEWER_VERSION: &str = "99.0.0";

fn mock_npm_registry(server: &MockServer) -> Mock<'_> {
    server.mock(|when, then| {
        when.method(GET)
            .path("/swelog-cli/latest");

        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({ "name": "swelog-cli", "version": NEWER_VERSION }));
    })
}

#[test]
fn update_check_off_never_contacts_the_registry() {
    let sandbox = SwelogSandbox::new();

    let registry = MockServer::start();

    let latest_version = mock_npm_registry(&registry);

    sandbox
        .swelog()
        .env("SWELOG_NPM_REGISTRY_URL", registry.base_url())
        .arg("config")
        .assert()
        .success()
        .stderr(contains("A new version of swelog is available").not());

    assert_eq!(latest_version.calls(), 0);

    assert!(!get_version_cache_file_path(&sandbox.cache_directory()).exists());
}

#[test]
fn update_check_caches_the_latest_version_and_prints_a_notice_on_the_next_run() {
    let sandbox = SwelogSandbox::new();

    let registry = MockServer::start();

    let latest_version = mock_npm_registry(&registry);

    sandbox
        .swelog()
        .env("SWELOG_UPDATE_CHECK", "on")
        .env("SWELOG_NPM_REGISTRY_URL", registry.base_url())
        .arg("config")
        .assert()
        .success()
        .stderr(contains("A new version of swelog is available").not());

    latest_version.assert();

    assert!(get_version_cache_file_path(&sandbox.cache_directory()).is_file());

    sandbox
        .swelog()
        .env("SWELOG_UPDATE_CHECK", "on")
        .env("SWELOG_NPM_REGISTRY_URL", registry.base_url())
        .arg("config")
        .assert()
        .success()
        .stderr(contains("A new version of swelog is available"))
        .stderr(contains(NEWER_VERSION))
        .stderr(contains("npm update -g swelog-cli"));

    assert_eq!(latest_version.calls(), 1);
}
