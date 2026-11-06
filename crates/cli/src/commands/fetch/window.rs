use chrono::NaiveDate;
use miette::Result;

use crate::shared::date_selection::{
    DateSelection,
    resolve_selected_date,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FetchWindow {
    Date(NaiveDate),
    Current,
}

impl FetchWindow {
    pub fn from_date_selection(date_selection: DateSelection, today: NaiveDate) -> Result<Self> {
        let selected_date = resolve_selected_date(date_selection, today)?;

        let fetch_window = selected_date.map_or(Self::Current, Self::Date);

        Ok(fetch_window)
    }

    #[must_use]
    pub const fn date_or_today(self, today: NaiveDate) -> NaiveDate {
        match self {
            Self::Date(date) => date,

            Self::Current => today,
        }
    }
}

#[cfg(test)]
#[path = "window_tests.rs"]
mod tests;
