pub mod errors;

use async_trait::async_trait;
use config::swelog_config::SwelogConfig;
use credentials::credential::Credential;
use miette::Result;

use crate::{
    commands::fetch::{
        integration::errors::MissingIntegrationConfiguration,
        outcome::FetchOutcome,
        window::FetchWindow,
    },
    environment::Environment,
};

#[derive(Clone, Copy, Debug)]
pub struct IntegrationSpec {
    pub label: &'static str,
    pub fetching_notice: &'static str,
    pub section_title: &'static str,
    pub credential: Credential,
}

#[async_trait]
pub trait Integration: Sync {
    fn spec(&self) -> &'static IntegrationSpec;

    fn find_missing_configuration(
        &self,
        _swelog_config: &SwelogConfig,
    ) -> Option<MissingIntegrationConfiguration> {
        None
    }

    async fn collect_fetch_outcome(
        &self,
        environment: &Environment,
        swelog_config: &SwelogConfig,
        fetch_window: FetchWindow,
    ) -> Result<FetchOutcome>;
}
