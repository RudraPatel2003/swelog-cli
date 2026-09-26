use super::*;

const RELEASE_VERSION: &str = "0.19.0";

const NPM_PACKAGE_JSON: &str = r#"{
  "name": "swelog-cli",
  "version": "0.18.0",
  "bin": {
    "swelog": "bin/swelog.js"
  }
}
"#;

const PACKAGE_JSON_WITHOUT_A_VERSION: &str = r#"{ "name": "swelog-cli" }"#;

#[test]
fn parse_package_json_version_reads_the_version() {
    let version = parse_package_json_version(NPM_PACKAGE_JSON).expect("version should parse");

    assert_eq!(version, "0.18.0");
}

#[test]
fn parse_package_json_version_fails_when_the_version_is_missing() {
    let error = parse_package_json_version(PACKAGE_JSON_WITHOUT_A_VERSION)
        .expect_err("version should not parse");

    assert_eq!(error.to_string(), "no version found");
}

const PACKAGE_JSON_WITH_A_NUMERIC_VERSION: &str = r#"{ "version": 18 }"#;

#[test]
fn parse_package_json_version_fails_when_the_version_is_not_a_string() {
    let error = parse_package_json_version(PACKAGE_JSON_WITH_A_NUMERIC_VERSION)
        .expect_err("version should not parse");

    assert_eq!(error.to_string(), "no version found");
}

const UNTERMINATED_PACKAGE_JSON: &str = "{";

#[test]
fn parse_package_json_version_fails_for_invalid_json() {
    let error = parse_package_json_version(UNTERMINATED_PACKAGE_JSON)
        .expect_err("version should not parse");

    assert_eq!(error.to_string(), "failed to parse");
}

const NPM_PACKAGE_JSON_AT_THE_RELEASE_VERSION: &str = r#"{
  "name": "swelog-cli",
  "version": "0.19.0",
  "bin": {
    "swelog": "bin/swelog.js"
  }
}
"#;

#[test]
fn replace_package_json_version_keeps_the_original_key_order() {
    let updated_package_json = replace_package_json_version(NPM_PACKAGE_JSON, RELEASE_VERSION)
        .expect("version should be replaced");

    assert_eq!(updated_package_json, NPM_PACKAGE_JSON_AT_THE_RELEASE_VERSION);
}

#[test]
fn replace_package_json_version_adds_a_missing_version() {
    let updated_package_json =
        replace_package_json_version(PACKAGE_JSON_WITHOUT_A_VERSION, RELEASE_VERSION)
            .expect("version should be replaced");

    let version = parse_package_json_version(&updated_package_json).expect("version should parse");

    assert_eq!(version, RELEASE_VERSION);
}

const PACKAGE_JSON_ARRAY: &str = "[]";

#[test]
fn replace_package_json_version_fails_when_the_document_is_not_an_object() {
    let error = replace_package_json_version(PACKAGE_JSON_ARRAY, RELEASE_VERSION)
        .expect_err("version should not replace");

    assert_eq!(error.to_string(), "expected a JSON object");
}
