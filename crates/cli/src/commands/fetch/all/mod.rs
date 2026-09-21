mod errors;
mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::{
    config_file::read_config_file,
    swelog_config::SwelogConfig,
};
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
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
                format_fetch_source_labels,
                format_running_notice,
            },
        },
        github::collect_github_activity,
        google_calendar::collect_google_calendar_meetings,
        linear::collect_linear_issues,
        outcome::{
            FetchOutcome,
            record_fetch_outcome,
        },
        sources::{
            FetchSource,
            collect_included_fetch_sources,
        },
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
    },
};

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct AllArgs {
    /// Date to fetch activity for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch activity for yesterday instead of today.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Fetch activity for last Friday instead of today.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

impl AllArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let swelog_config = read_config_file(&environment.config_file_path)?;

        let included_fetch_sources =
            collect_included_fetch_sources(&swelog_config, &environment.credential_store)?;

        if included_fetch_sources.is_empty() {
            let no_configured_fetch_sources_error = NoConfiguredFetchSources;

            return Err(no_configured_fetch_sources_error.into());
        }

        println!("{}", format_running_notice(&included_fetch_sources));

        println!();

        let date_selection = DateSelection::from_date_flags(DateFlags {
            date: self.date,
            use_yesterday: self.use_yesterday,
            use_last_friday: self.use_last_friday,
        });

        let failed_fetch_sources =
            run_fetch_sources(&included_fetch_sources, environment, &swelog_config, date_selection)
                .await;

        if failed_fetch_sources.is_empty() {
            return Ok(());
        }

        let fetch_sources_failed_error = FetchSourcesFailed {
            failed_source_labels: format_fetch_source_labels(&failed_fetch_sources),
        };

        Err(fetch_sources_failed_error.into())
    }
}

async fn run_fetch_sources(
    fetch_sources: &[FetchSource],
    environment: &Environment,
    swelog_config: &SwelogConfig,
    date_selection: DateSelection,
) -> Vec<FetchSource> {
    print_fetching_notices(fetch_sources);

    let fetch_results =
        collect_fetch_outcomes(fetch_sources, environment, swelog_config, date_selection).await;

    println!();

    record_fetch_results(fetch_sources, swelog_config, fetch_results)
}

fn print_fetching_notices(fetch_sources: &[FetchSource]) {
    for fetch_source in fetch_sources {
        fetch_source.print_fetching_notice();
    }
}

async fn collect_fetch_outcomes(
    fetch_sources: &[FetchSource],
    environment: &Environment,
    swelog_config: &SwelogConfig,
    date_selection: DateSelection,
) -> Vec<Result<FetchOutcome>> {
    let fetches = fetch_sources.iter().map(|fetch_source| {
        collect_fetch_outcome(*fetch_source, environment, swelog_config, date_selection)
    });

    join_all(fetches).await
}

async fn collect_fetch_outcome(
    fetch_source: FetchSource,
    environment: &Environment,
    swelog_config: &SwelogConfig,
    date_selection: DateSelection,
) -> Result<FetchOutcome> {
    match fetch_source {
        FetchSource::Github => collect_github_activity(environment, date_selection).await,

        FetchSource::Linear => {
            collect_linear_issues(environment, swelog_config, date_selection).await
        }

        FetchSource::GoogleCalendar => {
            collect_google_calendar_meetings(environment, date_selection).await
        }
    }
}

fn record_fetch_results(
    fetch_sources: &[FetchSource],
    swelog_config: &SwelogConfig,
    fetch_results: Vec<Result<FetchOutcome>>,
) -> Vec<FetchSource> {
    let mut failed_fetch_sources = Vec::new();

    for (fetch_source, fetch_result) in fetch_sources.iter().copied().zip(fetch_results) {
        let record_result = fetch_result
            .and_then(|fetch_outcome| record_fetch_outcome(swelog_config, fetch_outcome));

        if let Err(error) = record_result {
            eprintln!("{error:?}");

            failed_fetch_sources.push(fetch_source);
        }
    }

    failed_fetch_sources
}
