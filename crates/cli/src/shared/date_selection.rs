use chrono::{
    Datelike,
    Duration,
    NaiveDate,
    Weekday,
};
use clap::Args;
use dates::{
    date_format::DATE_VALUE_NAME,
    parsing::parse_date,
};
use miette::{
    Result,
    miette,
};

const DAYS_IN_WEEK: i64 = 7;

#[derive(Clone, Copy, Debug, Args)]
#[group(multiple = false)]
pub struct DateFlags {
    /// Use the given date in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Use yesterday's date.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Use last Friday's date.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateSelection {
    Unspecified,
    Yesterday,
    LastFriday,
    On(NaiveDate),
}

impl DateSelection {
    #[must_use]
    pub const fn from_date_flags(date_flags: DateFlags) -> Self {
        if date_flags.use_yesterday {
            return Self::Yesterday;
        }

        if date_flags.use_last_friday {
            return Self::LastFriday;
        }

        if let Some(date) = date_flags.date {
            return Self::On(date);
        }

        Self::Unspecified
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeekSelection {
    Current,
    LastWeek,
    WeekOf(NaiveDate),
}

impl WeekSelection {
    #[must_use]
    pub const fn from_week_flags(monday_date: Option<NaiveDate>, use_last_week: bool) -> Self {
        if use_last_week {
            return Self::LastWeek;
        }

        if let Some(monday_date) = monday_date {
            return Self::WeekOf(monday_date);
        }

        Self::Current
    }
}

pub fn resolve_selected_date(
    date_selection: DateSelection,
    today: NaiveDate,
) -> Result<Option<NaiveDate>> {
    match date_selection {
        DateSelection::Unspecified => Ok(None),

        DateSelection::On(date) => Ok(Some(date)),

        DateSelection::Yesterday => {
            let yesterday = today
                .pred_opt()
                .ok_or_else(|| miette!("failed to determine yesterday's date"))?;

            Ok(Some(yesterday))
        }

        DateSelection::LastFriday => get_last_friday(today).map(Some),
    }
}

pub fn resolve_monday_date(week_selection: WeekSelection, today: NaiveDate) -> Result<NaiveDate> {
    match week_selection {
        WeekSelection::WeekOf(monday_date) => Ok(monday_date),

        WeekSelection::Current => get_monday_of_current_week(today),

        WeekSelection::LastWeek => get_monday_of_current_week(today)?
            .checked_sub_signed(Duration::days(DAYS_IN_WEEK))
            .ok_or_else(|| miette!("failed to determine the Monday of the previous week")),
    }
}

fn get_last_friday(today: NaiveDate) -> Result<NaiveDate> {
    let days_since_friday = i64::from(
        today
            .weekday()
            .days_since(Weekday::Fri),
    );

    let days_back = if days_since_friday == 0 { DAYS_IN_WEEK } else { days_since_friday };

    today
        .checked_sub_signed(Duration::days(days_back))
        .ok_or_else(|| miette!("failed to determine the date of last Friday"))
}

fn get_monday_of_current_week(today: NaiveDate) -> Result<NaiveDate> {
    let days_since_monday = i64::from(
        today
            .weekday()
            .num_days_from_monday(),
    );

    today
        .checked_sub_signed(Duration::days(days_since_monday))
        .ok_or_else(|| miette!("failed to determine the Monday of the current week"))
}

#[cfg(test)]
#[path = "date_selection_tests.rs"]
mod tests;
