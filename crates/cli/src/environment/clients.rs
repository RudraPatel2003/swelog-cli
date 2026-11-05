use credentials::{
    credential::Credential,
    resolution::get_or_prompt_for_credential,
};
use github::client::GitHubClient;
use google_calendar::{
    client::GoogleCalendarClient,
    oauth::{
        GoogleAuthorization,
        application::get_oauth_application,
    },
};
use linear::client::LinearClient;
use llm::{
    language_model::LanguageModel,
    language_model_factory::get_language_model,
    summarization_settings::SummarizationSettings,
};
use miette::Result;
use sentry::client::SentryClient;

use crate::environment::Environment;

impl Environment {
    pub fn build_github_client(&self) -> Result<GitHubClient> {
        let github_token =
            get_or_prompt_for_credential(&self.credential_store, Credential::Github)?;

        let github_api_endpoint = self
            .endpoints
            .github_api
            .clone();

        let github_client = GitHubClient::new(github_api_endpoint, github_token);

        Ok(github_client)
    }

    pub fn build_google_calendar_client(&self) -> Result<GoogleCalendarClient> {
        let application = get_oauth_application(&self.google_oauth_application_overrides)?;

        let google_token = self
            .endpoints
            .google_token
            .clone();

        let google_calendar_api_endpoint = self
            .endpoints
            .google_calendar_api
            .clone();

        let credential_store = self
            .credential_store
            .clone();

        let authorization = GoogleAuthorization::new(application, google_token, credential_store);

        let google_calendar_client =
            GoogleCalendarClient::new(google_calendar_api_endpoint, authorization);

        Ok(google_calendar_client)
    }

    #[must_use]
    pub fn build_linear_client(&self) -> LinearClient {
        LinearClient::new(
            self.endpoints
                .linear_mcp
                .clone(),
            self.credential_store
                .clone(),
        )
    }

    pub fn build_sentry_client(&self) -> Result<SentryClient> {
        let sentry_token =
            get_or_prompt_for_credential(&self.credential_store, Credential::Sentry)?;

        let sentry_api_endpoint = self
            .endpoints
            .sentry_api
            .clone();

        let sentry_client = SentryClient::new(sentry_api_endpoint, sentry_token);

        Ok(sentry_client)
    }

    pub fn build_language_model(
        &self,
        summarization_settings: &SummarizationSettings,
    ) -> Result<Box<dyn LanguageModel>> {
        let language_model_endpoints = self
            .endpoints
            .language_model_endpoints();

        get_language_model(
            summarization_settings,
            &language_model_endpoints,
            &self.credential_store,
        )
    }
}
