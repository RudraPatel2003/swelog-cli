use chrono::NaiveDate;
use clap::Args;
use config::overwrite::Overwrite;
use daily_log::work_file::KeepWorkFile;
use miette::Result;

use crate::shared::date_selection::{
    DateFlags,
    DateSelection,
    resolve_selected_date,
};

#[derive(Debug, Args)]
pub struct DailyLogArgs {
    #[command(flatten)]
    date_flags: DateFlags,

    /// Overwrite existing daily log file.
    #[arg(long = "force")]
    overwrite_existing_daily_log: bool,

    /// Keep the current contents of the configured work file.
    #[arg(long = "keep")]
    keep_work_file: bool,
}

impl DailyLogArgs {
    pub fn resolve_log_date(&self, today: NaiveDate) -> Result<NaiveDate> {
        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let log_date = resolve_selected_date(date_selection, today)?.unwrap_or(today);

        Ok(log_date)
    }

    #[must_use]
    pub const fn overwrite(&self) -> Overwrite {
        Overwrite::from_force_flag(self.overwrite_existing_daily_log)
    }

    #[must_use]
    pub const fn keep_work_file(&self) -> KeepWorkFile {
        KeepWorkFile::from_keep_flag(self.keep_work_file)
    }
}
