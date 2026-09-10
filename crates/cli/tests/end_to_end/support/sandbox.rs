use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::Duration,
};

use assert_cmd::Command as AssertCommand;
use chrono::NaiveDate;
use command_extra::CommandExtra;
use config::{
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
};
use credentials::{
    credential::Credential,
    store::CredentialStore,
};
use daily_log::file::get_daily_log_file_path;
use summary::week::get_weekly_log_file_path;
use tempfile::{
    TempDir,
    tempdir,
};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

pub const TODAY: &str = "07-05-2026";

pub const ACTIVITY_DATE: &str = "07-04-2026";

pub const GITHUB_TOKEN: &str = "ghp_end_to_end";

pub const DEFAULT_WORK_FILE_CONTENT: &str = "# Today's Work

## Priorities
<!-- What you plan to focus on today. -->

## Log
<!-- Quick capture. Use short bullets; include systems, outcomes, reviews, debugging, meetings, or support work when useful. -->
";

pub const DEFAULT_WORK_FILE_CONTENT_WITHOUT_COMMENTS: &str = "# Today's Work

## Priorities

## Log
";

pub const WRITTEN_WORK_FILE_CONTENT: &str = "# Today's Work

## Priorities
- Ship end-to-end tests

## Log
- Reviewed the auth PR
- Paired on the release flow
";

pub struct SwelogSandbox {
    root: TempDir,
}

impl SwelogSandbox {
    pub fn new() -> Self {
        let root = tempdir().expect("sandbox directory should be created");

        let sandbox = Self { root };

        sandbox.write_config(&sandbox.default_config());

        sandbox
    }

    pub fn default_config(&self) -> SwelogConfig {
        SwelogConfig {
            obsidian_vault_path: self.vault_directory(),
            ..SwelogConfig::get_default_config()
        }
    }

    pub fn write_config(&self, config: &SwelogConfig) {
        let config_json = serde_json::to_string_pretty(config).expect("config should serialize");

        let config_file_path = self.config_file_path();

        fs::create_dir_all(config_file_path.parent().expect("config file should have a parent"))
            .expect("config directory should be created");

        fs::write(config_file_path, config_json).expect("config file should be written");
    }

    pub fn read_config(&self) -> SwelogConfig {
        let config_json =
            fs::read_to_string(self.config_file_path()).expect("config file should be readable");

        serde_json::from_str(&config_json).expect("config should parse")
    }

    pub fn vault_directory(&self) -> PathBuf {
        self.root.path().join("vault")
    }

    pub fn config_file_path(&self) -> PathBuf {
        self.root.path().join("config").join("swelog.json")
    }

    pub fn cache_directory(&self) -> PathBuf {
        self.root.path().join("cache")
    }

    pub fn credential_file(&self) -> PathBuf {
        self.root.path().join("credentials.json")
    }

    pub fn swelog_paths(&self) -> SwelogPaths {
        SwelogPaths::new(&self.default_config())
    }

    pub fn work_file(&self) -> PathBuf {
        self.swelog_paths().work_file
    }

    pub fn daily_log_file(&self, date: &str) -> PathBuf {
        get_daily_log_file_path(&self.swelog_paths(), &parse_date(date))
    }

    pub fn weekly_log_file(&self, monday_date: &str) -> PathBuf {
        get_weekly_log_file_path(&self.swelog_paths(), &parse_date(monday_date))
    }

    pub fn read_work_file(&self) -> String {
        fs::read_to_string(self.work_file()).expect("work file should be readable")
    }

    pub fn write_work_file(&self, content: &str) {
        fs::write(self.work_file(), content).expect("work file should be written");
    }

    pub fn read_daily_log(&self, date: &str) -> String {
        fs::read_to_string(self.daily_log_file(date)).expect("daily log should be readable")
    }

    pub fn write_daily_log(&self, date: &str, content: &str) {
        fs::write(self.daily_log_file(date), content).expect("daily log should be written");
    }

    pub fn store_credential(&self, credential: Credential, secret: &str) {
        CredentialStore::File(self.credential_file())
            .write(credential, secret)
            .expect("credential should be stored");
    }

    pub fn read_credential(&self, credential: Credential) -> Option<String> {
        CredentialStore::File(self.credential_file())
            .read(credential)
            .expect("credential should be readable")
    }

    pub fn swelog(&self) -> AssertCommand {
        let command = Command::new(env!("CARGO_BIN_EXE_swelog"))
            .with_no_env()
            .with_env("SWELOG_CONFIG_FILE", self.config_file_path())
            .with_env("SWELOG_CACHE_DIRECTORY", self.cache_directory())
            .with_env(
                "SWELOG_CREDENTIAL_STORE",
                format!("file:{}", self.credential_file().display()),
            )
            .with_env("SWELOG_UPDATE_CHECK", "off")
            .with_env("SWELOG_TODAY", TODAY)
            .with_env("NO_COLOR", "1")
            .with_env("TZ", "UTC");

        let mut assert_command = AssertCommand::from_std(command);

        assert_command.timeout(COMMAND_TIMEOUT);

        assert_command
    }

    pub fn setup(&self) {
        self.swelog().arg("setup").assert().success();
    }
}

pub fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%m-%d-%Y").expect("test date should be valid")
}
