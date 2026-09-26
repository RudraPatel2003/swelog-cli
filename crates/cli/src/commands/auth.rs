mod clear;
mod status;

use clap::{
    Args,
    Subcommand,
};
use miette::Result;

use crate::{
    commands::auth::{
        clear::ClearArgs,
        status::StatusArgs,
    },
    environment::Environment,
};

#[derive(Debug, Args)]
pub struct AuthArgs {
    #[command(subcommand)]
    command: AuthCommands,
}

#[derive(Debug, Subcommand)]
enum AuthCommands {
    /// Show which credentials swelog has stored in your operating system keyring.
    Status(StatusArgs),

    /// Remove a stored credential so the next command asks for it again.
    Clear(ClearArgs),
}

impl AuthArgs {
    pub fn run(self, environment: &Environment) -> Result<()> {
        match self.command {
            AuthCommands::Status(status_args) => status_args.run(environment),

            AuthCommands::Clear(clear_args) => clear_args.run(environment),
        }
    }
}
