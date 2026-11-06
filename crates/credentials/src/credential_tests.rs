use std::collections::HashSet;

use super::*;

fn assert_unique(values: &[&str]) {
    let unique_values: HashSet<&str> = values
        .iter()
        .copied()
        .collect();

    assert_eq!(unique_values.len(), values.len(), "{values:?} should not repeat");
}

fn collect_spec_values(get_value: impl Fn(CredentialSpec) -> &'static str) -> Vec<&'static str> {
    Credential::all()
        .iter()
        .map(|credential| get_value(credential.spec()))
        .collect()
}

#[test]
fn every_credential_has_a_unique_label() {
    assert_unique(&collect_spec_values(|credential_spec| credential_spec.label));
}

#[test]
fn every_credential_is_stored_under_a_unique_key() {
    assert_unique(&collect_spec_values(|credential_spec| credential_spec.storage_key));
}

#[test]
fn every_command_name_is_accepted_on_the_command_line() {
    for credential in Credential::all() {
        let possible_value = credential
            .to_possible_value()
            .expect("credential should be a command line value");

        assert_eq!(
            credential
                .spec()
                .command_name,
            possible_value.get_name()
        );
    }
}

#[test]
fn only_secret_credentials_are_read_from_the_environment() {
    assert_eq!(
        Credential::Github
            .spec()
            .environment_variable(),
        Some("GITHUB_TOKEN")
    );

    assert_eq!(
        Credential::Linear
            .spec()
            .environment_variable(),
        None
    );
}
