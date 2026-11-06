use std::collections::HashSet;

use config::integrations_config::{
    IntegrationsConfig,
    LinearConfig,
    SentryConfig,
};

use super::*;

fn get_config_with_linear_username(linear_username: Option<&str>) -> SwelogConfig {
    let linear_config =
        linear_username.map(|username| LinearConfig { username: username.to_string() });

    SwelogConfig {
        integrations: IntegrationsConfig { linear: linear_config, ..IntegrationsConfig::default() },
        ..SwelogConfig::get_default_config()
    }
}

fn get_config_with_sentry_organization(sentry_organization: Option<&str>) -> SwelogConfig {
    let sentry_config = sentry_organization
        .map(|organization| SentryConfig { organization: organization.to_string() });

    SwelogConfig {
        integrations: IntegrationsConfig { sentry: sentry_config, ..IntegrationsConfig::default() },
        ..SwelogConfig::get_default_config()
    }
}

fn get_missing_config_key(availability: IntegrationAvailability) -> Option<&'static str> {
    match availability {
        IntegrationAvailability::MissingConfiguration(missing_configuration) => {
            Some(missing_configuration.config_key)
        }

        IntegrationAvailability::Included | IntegrationAvailability::MissingAuthorization => None,
    }
}

fn assert_unique(values: &[&str]) {
    let unique_values: HashSet<&&str> = values
        .iter()
        .collect();

    assert_eq!(unique_values.len(), values.len(), "{values:?} should not repeat");
}

#[test]
fn every_integration_has_a_unique_label() {
    let labels: Vec<&str> = ALL_INTEGRATIONS
        .iter()
        .map(|integration| {
            integration
                .spec()
                .label
        })
        .collect();

    assert_unique(&labels);
}

#[test]
fn every_integration_writes_a_unique_work_file_section() {
    let section_titles: Vec<&str> = ALL_INTEGRATIONS
        .iter()
        .map(|integration| {
            integration
                .spec()
                .section_title
        })
        .collect();

    assert_unique(&section_titles);
}

#[test]
fn every_integration_uses_a_unique_credential() {
    let credential_labels: Vec<&str> = ALL_INTEGRATIONS
        .iter()
        .map(|integration| {
            integration
                .spec()
                .credential
                .label()
        })
        .collect();

    assert_unique(&credential_labels);
}

#[test]
fn github_is_included_when_authorized() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_integration_availability(
        &GithubIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, IntegrationAvailability::Included);
}

#[test]
fn an_integration_without_authorization_is_excluded() {
    let swelog_config = get_config_with_linear_username(Some("rudra"));

    let availability = get_integration_availability(
        &LinearIntegration,
        &swelog_config,
        AuthorizationPresence::Missing,
    );

    assert_eq!(availability, IntegrationAvailability::MissingAuthorization);
}

#[test]
fn linear_without_a_username_is_missing_its_configuration() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_integration_availability(
        &LinearIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(get_missing_config_key(availability), Some("integrations.linear.username"));
}

#[test]
fn linear_with_a_blank_username_is_missing_its_configuration() {
    let swelog_config = get_config_with_linear_username(Some("   "));

    let availability = get_integration_availability(
        &LinearIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(get_missing_config_key(availability), Some("integrations.linear.username"));
}

#[test]
fn integrations_without_settings_never_miss_configuration() {
    let swelog_config = get_config_with_linear_username(None);

    assert_eq!(GithubIntegration.find_missing_configuration(&swelog_config), None);

    assert_eq!(GoogleCalendarIntegration.find_missing_configuration(&swelog_config), None);
}

#[test]
fn linear_with_a_username_is_included() {
    let swelog_config = get_config_with_linear_username(Some("rudra"));

    let availability = get_integration_availability(
        &LinearIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, IntegrationAvailability::Included);
}

#[test]
fn google_calendar_ignores_the_linear_username() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_integration_availability(
        &GoogleCalendarIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, IntegrationAvailability::Included);
}

#[test]
fn sentry_with_an_organization_is_included() {
    let swelog_config = get_config_with_sentry_organization(Some("acme"));

    let availability = get_integration_availability(
        &SentryIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, IntegrationAvailability::Included);
}

#[test]
fn sentry_without_an_organization_is_missing_its_configuration() {
    let swelog_config = get_config_with_sentry_organization(None);

    let availability = get_integration_availability(
        &SentryIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(get_missing_config_key(availability), Some("integrations.sentry.organization"));
}

#[test]
fn sentry_with_a_blank_organization_is_missing_its_configuration() {
    let swelog_config = get_config_with_sentry_organization(Some("   "));

    let availability = get_integration_availability(
        &SentryIntegration,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(get_missing_config_key(availability), Some("integrations.sentry.organization"));
}
