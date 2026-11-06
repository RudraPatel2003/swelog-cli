use config::swelog_config::SwelogConfig;
use credentials::{
    credential::Credential,
    resolution::read_credential_from_environment,
    store::CredentialStore,
};
use miette::Result;

use crate::commands::fetch::{
    github::GithubIntegration,
    google_calendar::GoogleCalendarIntegration,
    integration::{
        Integration,
        errors::MissingIntegrationConfiguration,
    },
    linear::LinearIntegration,
    sentry::SentryIntegration,
};

pub const ALL_INTEGRATIONS: &[&dyn Integration] =
    &[&GithubIntegration, &LinearIntegration, &GoogleCalendarIntegration, &SentryIntegration];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorizationPresence {
    Present,
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegrationAvailability {
    Included,
    MissingAuthorization,
    MissingConfiguration(MissingIntegrationConfiguration),
}

pub fn collect_integration_availabilities(
    swelog_config: &SwelogConfig,
    credential_store: &CredentialStore,
) -> Result<Vec<(&'static dyn Integration, IntegrationAvailability)>> {
    ALL_INTEGRATIONS
        .iter()
        .map(|integration| {
            let availability =
                resolve_integration_availability(*integration, swelog_config, credential_store)?;

            Ok((*integration, availability))
        })
        .collect()
}

pub fn collect_included_integrations(
    swelog_config: &SwelogConfig,
    credential_store: &CredentialStore,
) -> Result<Vec<&'static dyn Integration>> {
    let included_integrations =
        collect_integration_availabilities(swelog_config, credential_store)?
            .into_iter()
            .filter(|(_, availability)| matches!(availability, IntegrationAvailability::Included))
            .map(|(integration, _)| integration)
            .collect();

    Ok(included_integrations)
}

fn resolve_integration_availability(
    integration: &dyn Integration,
    swelog_config: &SwelogConfig,
    credential_store: &CredentialStore,
) -> Result<IntegrationAvailability> {
    let authorization = read_authorization_presence(
        credential_store,
        integration
            .spec()
            .credential,
    )?;

    Ok(get_integration_availability(integration, swelog_config, authorization))
}

fn read_authorization_presence(
    credential_store: &CredentialStore,
    credential: Credential,
) -> Result<AuthorizationPresence> {
    if read_credential_from_environment(credential).is_some() {
        return Ok(AuthorizationPresence::Present);
    }

    if credential_store
        .read(credential)?
        .is_some()
    {
        return Ok(AuthorizationPresence::Present);
    }

    Ok(AuthorizationPresence::Missing)
}

fn get_integration_availability(
    integration: &dyn Integration,
    swelog_config: &SwelogConfig,
    authorization: AuthorizationPresence,
) -> IntegrationAvailability {
    if authorization == AuthorizationPresence::Missing {
        return IntegrationAvailability::MissingAuthorization;
    }

    if let Some(missing_configuration) = integration.find_missing_configuration(swelog_config) {
        return IntegrationAvailability::MissingConfiguration(missing_configuration);
    }

    IntegrationAvailability::Included
}

#[cfg(test)]
#[path = "integrations_tests.rs"]
mod tests;
