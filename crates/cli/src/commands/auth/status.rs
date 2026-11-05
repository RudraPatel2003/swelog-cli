use clap::Args;
use credentials::{
    credential::Credential,
    resolution::read_credential_from_environment,
    store::CredentialStore,
};
use highlight::stdout::{
    highlight_stdout_dimmed,
    highlight_stdout_green,
    highlight_stdout_yellow,
};
use miette::Result;

use crate::environment::Environment;

const LABEL_WIDTH: usize = 31;

#[derive(Debug, Args)]
pub struct StatusArgs {}

impl StatusArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        let _ = self;

        println!(
            "Credentials stored in {}:",
            environment
                .credential_store
                .describe()
        );

        println!();

        for credential in Credential::ALL_CREDENTIALS {
            println!(
                "{:LABEL_WIDTH$}{}",
                credential.label(),
                describe_credential_status(&environment.credential_store, credential)?
            );
        }

        println!();

        println!("Run `swelog auth clear <credential>` to remove a stored credential.");

        Ok(())
    }
}

fn describe_credential_status(
    credential_store: &CredentialStore,
    credential: Credential,
) -> Result<String> {
    if let Some(environment_variable) = credential.environment_variable()
        && read_credential_from_environment(credential).is_some()
    {
        return Ok(highlight_stdout_yellow(format!("set by ${environment_variable}")));
    }

    let description = if credential_store
        .read(credential)?
        .is_some()
    {
        highlight_stdout_green("stored")
    } else {
        highlight_stdout_dimmed("not stored")
    };

    Ok(description)
}
