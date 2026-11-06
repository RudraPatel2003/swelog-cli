mod errors;
mod formatting;

use clap::Args;
use config::{
    config_file::read_config_file,
    swelog_config::SwelogConfig,
};
use futures::future::join_all;
use miette::Result;

use crate::{
    commands::fetch::{
        all::{
            errors::{
                FetchSourcesFailed,
                NoConfiguredFetchSources,
            },
            formatting::{
                format_integration_labels,
                format_running_notice,
            },
        },
        integration::Integration,
        integrations::collect_included_integrations,
        outcome::{
            FetchOutcome,
            record_fetch_outcome,
        },
        window::FetchWindow,
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
    },
};

#[derive(Debug, Args)]
pub struct AllArgs {
    #[command(flatten)]
    date_flags: DateFlags,
}

impl AllArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let swelog_config = read_config_file(&environment.config_file_path)?;

        let included_integrations =
            collect_included_integrations(&swelog_config, &environment.credential_store)?;

        if included_integrations.is_empty() {
            let no_configured_fetch_sources_error = NoConfiguredFetchSources;

            return Err(no_configured_fetch_sources_error.into());
        }

        println!("{}", format_running_notice(&included_integrations));

        println!();

        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let fetch_window = FetchWindow::from_date_selection(date_selection, environment.today)?;

        let failed_integrations =
            run_integrations(&included_integrations, environment, &swelog_config, fetch_window)
                .await;

        if failed_integrations.is_empty() {
            return Ok(());
        }

        let fetch_sources_failed_error = FetchSourcesFailed {
            failed_source_labels: format_integration_labels(&failed_integrations),
        };

        Err(fetch_sources_failed_error.into())
    }
}

async fn run_integrations(
    integrations: &[&'static dyn Integration],
    environment: &Environment,
    swelog_config: &SwelogConfig,
    fetch_window: FetchWindow,
) -> Vec<&'static dyn Integration> {
    print_fetching_notices(integrations);

    let fetch_results =
        collect_fetch_outcomes(integrations, environment, swelog_config, fetch_window).await;

    println!();

    record_fetch_results(integrations, swelog_config, fetch_results)
}

fn print_fetching_notices(integrations: &[&dyn Integration]) {
    for integration in integrations {
        println!(
            "{}",
            integration
                .spec()
                .fetching_notice
        );
    }
}

async fn collect_fetch_outcomes(
    integrations: &[&dyn Integration],
    environment: &Environment,
    swelog_config: &SwelogConfig,
    fetch_window: FetchWindow,
) -> Vec<Result<FetchOutcome>> {
    let fetches = integrations
        .iter()
        .map(|integration| {
            integration.collect_fetch_outcome(environment, swelog_config, fetch_window)
        });

    join_all(fetches).await
}

fn record_fetch_results(
    integrations: &[&'static dyn Integration],
    swelog_config: &SwelogConfig,
    fetch_results: Vec<Result<FetchOutcome>>,
) -> Vec<&'static dyn Integration> {
    let mut failed_integrations = Vec::new();

    for (integration, fetch_result) in integrations
        .iter()
        .copied()
        .zip(fetch_results)
    {
        let section_title = integration
            .spec()
            .section_title;

        let record_result = fetch_result.and_then(|fetch_outcome| {
            record_fetch_outcome(swelog_config, section_title, fetch_outcome)
        });

        if let Err(error) = record_result {
            eprintln!("{error:?}");

            failed_integrations.push(integration);
        }
    }

    failed_integrations
}
