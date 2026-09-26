use clap::Args;
use config::config_file::read_config_file;
use highlight::stdout::{
    highlight_dimmed,
    highlight_green,
};
use miette::Result;

use crate::{
    commands::fetch::sources::{
        FetchSource,
        FetchSourceAvailability,
        collect_fetch_source_availabilities,
    },
    environment::Environment,
};

const LABEL_WIDTH: usize = 20;

#[derive(Debug, Args)]
pub struct StatusArgs {}

impl StatusArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file(&environment.config_file_path)?;

        println!("Fetch commands included in `swelog fetch all`:");

        println!();

        let availabilities =
            collect_fetch_source_availabilities(&swelog_config, &environment.credential_store)?;

        for (fetch_source, availability) in availabilities {
            println!(
                "{:LABEL_WIDTH$}{}",
                fetch_source.label(),
                describe_fetch_source_availability(fetch_source, availability)
            );
        }

        println!();

        println!("Run `swelog auth status` to see every credential swelog has stored.");

        Ok(())
    }
}

fn describe_fetch_source_availability(
    fetch_source: FetchSource,
    availability: FetchSourceAvailability,
) -> String {
    match availability {
        FetchSourceAvailability::Included => highlight_green("included"),

        FetchSourceAvailability::MissingAuthorization => highlight_dimmed(format!(
            "not included, {} is not stored",
            fetch_source.credential().label()
        )),

        FetchSourceAvailability::MissingConfiguration { reason } => {
            highlight_dimmed(format!("not included, {reason}"))
        }
    }
}
