use std::path::PathBuf;

use crate::{
    context_file::CONTEXT_FILE_NAME,
    swelog_config::SwelogConfig,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SwelogPaths {
    pub swelog_directory: PathBuf,
    pub context_file: PathBuf,
    pub work_file: PathBuf,
    pub daily_log_directory: PathBuf,
    pub weekly_log_directory: PathBuf,
}

impl SwelogPaths {
    #[must_use]
    pub fn new(swelog_config: &SwelogConfig) -> Self {
        let swelog_directory =
            swelog_config.obsidian_vault_path.join(&swelog_config.swelog_folder_name);

        Self {
            context_file: swelog_directory.join(CONTEXT_FILE_NAME),
            work_file: swelog_directory.join(&swelog_config.work_file_name),
            daily_log_directory: swelog_directory.join(&swelog_config.daily_log_folder_name),
            weekly_log_directory: swelog_directory.join(&swelog_config.weekly_log_folder_name),
            swelog_directory,
        }
    }

    #[must_use]
    pub const fn all_paths(&self) -> [&PathBuf; 3] {
        [&self.work_file, &self.daily_log_directory, &self.weekly_log_directory]
    }
}

#[cfg(test)]
#[path = "swelog_paths_tests.rs"]
mod tests;
