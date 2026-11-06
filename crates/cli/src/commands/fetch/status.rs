use clap::Args;
use config::config_file::read_config_file;
use credentials::credential::Credential;
use highlight::stdout::{
    highlight_stdout_dimmed,
    highlight_stdout_green,
};
use miette::Result;

use crate::{
    commands::fetch::integrations::{
        IntegrationAvailability,
        collect_integration_availabilities,
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
            collect_integration_availabilities(&swelog_config, &environment.credential_store)?;

        for (integration, availability) in availabilities {
            let integration_spec = integration.spec();

            println!(
                "{:LABEL_WIDTH$}{}",
                integration_spec.label,
                describe_integration_availability(integration_spec.credential, availability)
            );
        }

        println!();

        println!("Run `swelog auth status` to see every credential swelog has stored.");

        Ok(())
    }
}

fn describe_integration_availability(
    credential: Credential,
    availability: IntegrationAvailability,
) -> String {
    match availability {
        IntegrationAvailability::Included => highlight_stdout_green("included"),

        IntegrationAvailability::MissingAuthorization => highlight_stdout_dimmed(format!(
            "not included, {} is not stored",
            credential
                .spec()
                .label
        )),

        IntegrationAvailability::MissingConfiguration(missing_configuration) => {
            highlight_stdout_dimmed(format!(
                "not included, {} is not configured",
                missing_configuration.config_key
            ))
        }
    }
}
