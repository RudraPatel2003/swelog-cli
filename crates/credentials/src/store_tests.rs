use std::path::PathBuf;

use tempfile::{
    TempDir,
    tempdir,
};

use super::*;

const GITHUB_TOKEN: &str = "ghp_example";

const ANTHROPIC_API_KEY: &str = "sk-ant-example";

struct TestContext {
    temporary_directory: TempDir,
    credential_store: CredentialStore,
}

fn get_test_context() -> TestContext {
    let temporary_directory = tempdir().expect("temp directory should be created");

    let credential_file = temporary_directory.path().join("nested").join("credentials.json");

    TestContext { temporary_directory, credential_store: CredentialStore::File(credential_file) }
}

#[test]
fn file_store_reads_nothing_before_a_credential_is_written() {
    let TestContext { temporary_directory, credential_store } = get_test_context();

    let secret = credential_store.read(Credential::Github).expect("read should succeed");

    assert_eq!(secret, None);

    drop(temporary_directory);
}

#[test]
fn file_store_reads_back_what_it_wrote() {
    let TestContext { temporary_directory, credential_store } = get_test_context();

    credential_store.write(Credential::Github, GITHUB_TOKEN).expect("write should succeed");

    credential_store.write(Credential::Anthropic, ANTHROPIC_API_KEY).expect("write should succeed");

    let github_token = credential_store.read(Credential::Github).expect("read should succeed");

    let anthropic_api_key =
        credential_store.read(Credential::Anthropic).expect("read should succeed");

    assert_eq!(github_token.as_deref(), Some(GITHUB_TOKEN));

    assert_eq!(anthropic_api_key.as_deref(), Some(ANTHROPIC_API_KEY));

    drop(temporary_directory);
}

#[test]
fn file_store_clear_reports_whether_a_credential_was_stored() {
    let TestContext { temporary_directory, credential_store } = get_test_context();

    credential_store.write(Credential::Github, GITHUB_TOKEN).expect("write should succeed");

    let was_cleared = credential_store.clear(Credential::Github).expect("clear should succeed");

    let was_cleared_again =
        credential_store.clear(Credential::Github).expect("clear should succeed");

    assert!(was_cleared);

    assert!(!was_cleared_again);

    assert_eq!(credential_store.read(Credential::Github).expect("read should succeed"), None);

    drop(temporary_directory);
}

#[test]
fn credential_store_parses_the_keyring_selection() {
    let credential_store: CredentialStore = "keyring".parse().expect("keyring should parse");

    assert_eq!(credential_store, CredentialStore::Keyring);
}

#[test]
fn credential_store_parses_a_file_selection() {
    let credential_store: CredentialStore =
        "file:/tmp/credentials.json".parse().expect("file selection should parse");

    assert_eq!(credential_store, CredentialStore::File(PathBuf::from("/tmp/credentials.json")));
}

#[test]
fn credential_store_rejects_an_unknown_selection() {
    let error = "vault".parse::<CredentialStore>().expect_err("unknown selection should fail");

    assert_eq!(error.value, "vault");
}

#[test]
fn credential_store_rejects_a_file_selection_without_a_path() {
    let error = "file:".parse::<CredentialStore>().expect_err("empty path should fail");

    assert_eq!(error.value, "file:");
}
