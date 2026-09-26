use super::*;

#[test]
fn npm_package_manifest_is_parsed_from_a_registry_response() {
    let response_text = r#"{
        "name": "swelog-cli",
        "version": "0.11.0",
        "dist": { "tarball": "https://registry.npmjs.org/swelog-cli/-/swelog-cli-0.11.0.tgz" }
    }"#;

    let latest_version =
        parse_npm_package_manifest(response_text).expect("manifest should be parsed");

    assert_eq!(latest_version, "0.11.0");
}

#[test]
fn parsing_an_npm_package_manifest_fails_when_the_version_is_missing() {
    let response_text = r#"{ "name": "swelog-cli" }"#;

    let result = parse_npm_package_manifest(response_text);

    assert!(result.is_err());
}
