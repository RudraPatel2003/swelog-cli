use chrono::NaiveDate;
use clap::Args;
use config::overwrite::Overwrite;
use daily_log::work_file::KeepWorkFile;
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
use miette::Result;

use crate::shared::date_selection::{
    DateFlags,
    DateSelection,
    resolve_selected_date,
};

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct DailyLogDateArgs {
    /// Date to write the daily log for in the format MM-DD-YYYY. Defaults to today.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Write the daily log for yesterday instead of today.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Write the daily log for last Friday instead of today.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

impl DailyLogDateArgs {
    fn resolve_log_date(&self, today: NaiveDate) -> Result<NaiveDate> {
        let date_selection = DateSelection::from_date_flags(DateFlags {
            date: self.date,
            use_yesterday: self.use_yesterday,
            use_last_friday: self.use_last_friday,
        });

        let log_date = resolve_selected_date(date_selection, today)?.unwrap_or(today);

        Ok(log_date)
    }
}

/// The flags shared by every command that writes a daily log
#[derive(Debug, Args)]
pub struct DailyLogArgs {
    #[command(flatten)]
    date_args: DailyLogDateArgs,

    /// Overwrite existing daily log file.
    #[arg(long = "force")]
    overwrite_existing_daily_log: bool,

    /// Keep the current contents of the configured work file.
    #[arg(long = "keep")]
    keep_work_file: bool,
}

impl DailyLogArgs {
    pub fn resolve_log_date(&self, today: NaiveDate) -> Result<NaiveDate> {
        self.date_args.resolve_log_date(today)
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
