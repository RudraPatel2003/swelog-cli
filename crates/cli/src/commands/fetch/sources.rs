use config::swelog_config::SwelogConfig;
use credentials::{
    credential::Credential,
    resolution::read_credential_from_environment,
    store::CredentialStore,
};
use miette::Result;

use crate::commands::fetch::linear::describe_missing_linear_configuration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FetchSource {
    Github,
    Linear,
    GoogleCalendar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorizationPresence {
    Present,
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FetchSourceAvailability {
    Included,
    MissingAuthorization,
    MissingConfiguration { reason: &'static str },
}

impl FetchSource {
    pub const ALL_FETCH_SOURCES: [Self; 3] = [Self::Github, Self::Linear, Self::GoogleCalendar];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Github => "GitHub",
            Self::Linear => "Linear",
            Self::GoogleCalendar => "Google Calendar",
        }
    }

    const fn fetching_notice(self) -> &'static str {
        match self {
            Self::Github => "Fetching GitHub PRs...",
            Self::Linear => "Fetching Linear issues...",
            Self::GoogleCalendar => "Fetching Google Calendar events...",
        }
    }

    #[must_use]
    pub const fn credential(self) -> Credential {
        match self {
            Self::Github => Credential::Github,
            Self::Linear => Credential::Linear,
            Self::GoogleCalendar => Credential::GoogleCalendar,
        }
    }

    fn describe_missing_configuration(self, swelog_config: &SwelogConfig) -> Option<&'static str> {
        match self {
            Self::Linear => describe_missing_linear_configuration(swelog_config),

            Self::Github | Self::GoogleCalendar => None,
        }
    }

    pub fn print_fetching_notice(self) {
        println!("{}", self.fetching_notice());
    }
}

pub fn collect_fetch_source_availabilities(
    swelog_config: &SwelogConfig,
    credential_store: &CredentialStore,
) -> Result<Vec<(FetchSource, FetchSourceAvailability)>> {
    FetchSource::ALL_FETCH_SOURCES
        .into_iter()
        .map(|fetch_source| {
            let availability =
                resolve_fetch_source_availability(fetch_source, swelog_config, credential_store)?;

            Ok((fetch_source, availability))
        })
        .collect()
}

pub fn collect_included_fetch_sources(
    swelog_config: &SwelogConfig,
    credential_store: &CredentialStore,
) -> Result<Vec<FetchSource>> {
    let included_fetch_sources =
        collect_fetch_source_availabilities(swelog_config, credential_store)?
            .into_iter()
            .filter(|(_, availability)| matches!(availability, FetchSourceAvailability::Included))
            .map(|(fetch_source, _)| fetch_source)
            .collect();

    Ok(included_fetch_sources)
}

fn resolve_fetch_source_availability(
    fetch_source: FetchSource,
    swelog_config: &SwelogConfig,
    credential_store: &CredentialStore,
) -> Result<FetchSourceAvailability> {
    let authorization = read_authorization_presence(credential_store, fetch_source.credential())?;

    Ok(get_fetch_source_availability(fetch_source, swelog_config, authorization))
}

fn read_authorization_presence(
    credential_store: &CredentialStore,
    credential: Credential,
) -> Result<AuthorizationPresence> {
    if read_credential_from_environment(credential).is_some() {
        return Ok(AuthorizationPresence::Present);
    }

    if credential_store.read(credential)?.is_some() {
        return Ok(AuthorizationPresence::Present);
    }

    Ok(AuthorizationPresence::Missing)
}

fn get_fetch_source_availability(
    fetch_source: FetchSource,
    swelog_config: &SwelogConfig,
    authorization: AuthorizationPresence,
) -> FetchSourceAvailability {
    if authorization == AuthorizationPresence::Missing {
        return FetchSourceAvailability::MissingAuthorization;
    }

    if let Some(reason) = fetch_source.describe_missing_configuration(swelog_config) {
        return FetchSourceAvailability::MissingConfiguration { reason };
    }

    FetchSourceAvailability::Included
}

#[cfg(test)]
#[path = "sources_tests.rs"]
mod tests;
