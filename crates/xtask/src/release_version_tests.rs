use super::*;

#[test]
fn parse_release_version_accepts_a_semantic_version() {
    let release_version = parse_release_version("1.2.3").expect("version should parse");

    assert_eq!(release_version.to_string(), "1.2.3");
}

#[test]
fn parse_release_version_accepts_a_pre_release_version() {
    let release_version = parse_release_version("1.2.3-beta.1").expect("version should parse");

    assert_eq!(release_version.to_string(), "1.2.3-beta.1");
}

#[test]
fn parse_release_version_rejects_a_leading_v() {
    let error = parse_release_version("v1.2.3").expect_err("version should not parse");

    assert_eq!(error.to_string(), "`v1.2.3` is not a valid semantic version like 1.2.3");
}

#[test]
fn parse_release_version_rejects_a_partial_version() {
    let error = parse_release_version("1.2").expect_err("version should not parse");

    assert_eq!(error.to_string(), "`1.2` is not a valid semantic version like 1.2.3");
}

#[test]
fn parse_release_version_rejects_an_empty_version() {
    let error = parse_release_version("").expect_err("version should not parse");

    assert_eq!(error.to_string(), "`` is not a valid semantic version like 1.2.3");
}
