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

#[test]
fn get_fetch_source_availability_includes_github_when_authorized() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_fetch_source_availability(
        FetchSource::Github,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}

#[test]
fn get_fetch_source_availability_excludes_source_without_authorization() {
    let swelog_config = get_config_with_linear_username(Some("rudra"));

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Missing,
    );

    assert_eq!(availability, FetchSourceAvailability::MissingAuthorization);
}

#[test]
fn get_fetch_source_availability_excludes_linear_without_a_username() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(
        availability,
        FetchSourceAvailability::MissingConfiguration {
            reason: "integrations.linear.username is not configured"
        }
    );
}

#[test]
fn get_fetch_source_availability_excludes_linear_when_the_username_is_blank() {
    let swelog_config = get_config_with_linear_username(Some("   "));

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(
        availability,
        FetchSourceAvailability::MissingConfiguration {
            reason: "integrations.linear.username is not configured"
        }
    );
}

#[test]
fn describe_missing_configuration_is_empty_for_a_source_with_no_configuration() {
    let swelog_config = get_config_with_linear_username(None);

    assert_eq!(FetchSource::Github.describe_missing_configuration(&swelog_config), None);

    assert_eq!(FetchSource::GoogleCalendar.describe_missing_configuration(&swelog_config), None);
}

#[test]
fn get_fetch_source_availability_includes_linear_with_a_username() {
    let swelog_config = get_config_with_linear_username(Some("rudra"));

    let availability = get_fetch_source_availability(
        FetchSource::Linear,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}

#[test]
fn get_fetch_source_availability_ignores_the_linear_username_for_google_calendar() {
    let swelog_config = get_config_with_linear_username(None);

    let availability = get_fetch_source_availability(
        FetchSource::GoogleCalendar,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}

fn get_config_with_sentry_organization(sentry_organization: Option<&str>) -> SwelogConfig {
    let sentry_config = sentry_organization
        .map(|organization| SentryConfig { organization: organization.to_string() });

    SwelogConfig {
        integrations: IntegrationsConfig { sentry: sentry_config, ..IntegrationsConfig::default() },
        ..SwelogConfig::get_default_config()
    }
}

#[test]
fn get_fetch_source_availability_includes_sentry_when_authorized_and_configured() {
    let swelog_config = get_config_with_sentry_organization(Some("acme"));

    let availability = get_fetch_source_availability(
        FetchSource::Sentry,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(availability, FetchSourceAvailability::Included);
}

#[test]
fn get_fetch_source_availability_excludes_sentry_without_an_organization() {
    let swelog_config = get_config_with_sentry_organization(None);

    let availability = get_fetch_source_availability(
        FetchSource::Sentry,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(
        availability,
        FetchSourceAvailability::MissingConfiguration {
            reason: "integrations.sentry.organization is not configured"
        }
    );
}

#[test]
fn get_fetch_source_availability_excludes_sentry_when_the_organization_is_blank() {
    let swelog_config = get_config_with_sentry_organization(Some("   "));

    let availability = get_fetch_source_availability(
        FetchSource::Sentry,
        &swelog_config,
        AuthorizationPresence::Present,
    );

    assert_eq!(
        availability,
        FetchSourceAvailability::MissingConfiguration {
            reason: "integrations.sentry.organization is not configured"
        }
    );
}
