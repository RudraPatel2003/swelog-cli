use super::*;

const RELEASE_VERSION: &str = "0.19.0";

const CLI_MANIFEST: &str = r#"[package]
name = "cli"
version = "0.18.0"
edition = { workspace = true }

[dependencies]
miette = { workspace = true }
"#;

const MANIFEST_WITHOUT_A_VERSION: &str = r#"[package]
name = "cli"
"#;

#[test]
fn parse_crate_version_reads_the_package_version() {
    let version = parse_crate_version(CLI_MANIFEST).expect("version should parse");

    assert_eq!(version, "0.18.0");
}

const MANIFEST_WITH_A_DEPENDENCY_VERSION: &str = r#"[package]
name = "cli"
version = "0.18.0"

[dependencies.miette]
version = "7.6.0"
"#;

#[test]
fn parse_crate_version_ignores_indented_dependency_versions() {
    let version =
        parse_crate_version(MANIFEST_WITH_A_DEPENDENCY_VERSION).expect("version should parse");

    assert_eq!(version, "0.18.0");
}

#[test]
fn parse_crate_version_fails_when_there_is_no_version_line() {
    let error =
        parse_crate_version(MANIFEST_WITHOUT_A_VERSION).expect_err("version should not parse");

    assert_eq!(error.to_string(), "no version line found");
}

const MANIFEST_WITH_AN_INHERITED_VERSION: &str = r"[package]
version = { workspace = true }
";

#[test]
fn parse_crate_version_fails_when_the_version_is_not_quoted() {
    let error = parse_crate_version(MANIFEST_WITH_AN_INHERITED_VERSION)
        .expect_err("version should not parse");

    assert_eq!(
        error.to_string(),
        "could not read a quoted version from `version = { workspace = true }`"
    );
}

const CLI_MANIFEST_AT_THE_RELEASE_VERSION: &str = r#"[package]
name = "cli"
version = "0.19.0"
edition = { workspace = true }

[dependencies]
miette = { workspace = true }
"#;

#[test]
fn replace_crate_version_rewrites_only_the_package_version() {
    let updated_manifest =
        replace_crate_version(CLI_MANIFEST, RELEASE_VERSION).expect("version should be replaced");

    assert_eq!(updated_manifest, CLI_MANIFEST_AT_THE_RELEASE_VERSION);
}

const MANIFEST_WITHOUT_A_TRAILING_NEWLINE: &str = "[package]\nversion = \"0.18.0\"";

#[test]
fn replace_crate_version_leaves_the_manifest_with_one_trailing_newline() {
    let updated_manifest =
        replace_crate_version(MANIFEST_WITHOUT_A_TRAILING_NEWLINE, RELEASE_VERSION)
            .expect("version should be replaced");

    assert_eq!(updated_manifest, "[package]\nversion = \"0.19.0\"\n");
}

#[test]
fn replace_crate_version_fails_when_there_is_no_version_line() {
    let error = replace_crate_version(MANIFEST_WITHOUT_A_VERSION, RELEASE_VERSION)
        .expect_err("version should not replace");

    assert_eq!(error.to_string(), "no version line found");
}
