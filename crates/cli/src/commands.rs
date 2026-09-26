mod auth;
mod config;
mod fetch;
mod init;
mod log;
mod reset;
mod setup;
mod summarize;
mod undo;

use auth::AuthArgs;
use clap::Subcommand;
use fetch::FetchArgs;
use init::InitArgs;
use log::LogArgs;
use reset::ResetArgs;
use setup::SetupArgs;
use summarize::SummarizeArgs;
use undo::UndoArgs;

use self::config::ConfigArgs;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a default swelog config file.
    Init(InitArgs),

    /// Setup the swelog files in your Obsidian vault.
    Setup(SetupArgs),

    /// Summarize your configured work file. By default, this is an alias for `swelog summarize
    /// day`.
    Summarize(SummarizeArgs),

    /// Write your work file into a dated daily log.
    Log(LogArgs),

    /// Reset your work file to the default content.
    Reset(ResetArgs),

    /// Undo your last swelog log, swelog summarize day, or swelog reset.
    Undo(UndoArgs),

    /// Add data from external sources to your work file.
    Fetch(FetchArgs),

    /// Display your current swelog configuration and where it is stored.
    Config(ConfigArgs),

    /// Manage the credentials swelog stores in your operating system keyring.
    Auth(AuthArgs),
}
