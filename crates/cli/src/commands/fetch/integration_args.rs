use clap::Args;
use config::config_file::read_config_file;
use miette::Result;

use crate::{
    commands::fetch::{
        integration::Integration,
        outcome::record_fetch_outcome,
        window::FetchWindow,
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
    },
};

#[derive(Debug, Args)]
pub struct IntegrationArgs {
    #[command(flatten)]
    date_flags: DateFlags,
}

impl IntegrationArgs {
    pub async fn run(self, integration: &dyn Integration, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let fetch_window = FetchWindow::from_date_selection(date_selection, environment.today)?;

        let swelog_config = read_config_file(&environment.config_file_path)?;

        let integration_spec = integration.spec();

        println!("{}", integration_spec.fetching_notice);

        let fetch_outcome = integration
            .collect_fetch_outcome(environment, &swelog_config, fetch_window)
            .await?;

        record_fetch_outcome(&swelog_config, integration_spec.section_title, fetch_outcome)
    }
}
