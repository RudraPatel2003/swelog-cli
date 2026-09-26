mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::config_file::read_config_file;
use dates::{
    date_format::DATE_VALUE_NAME,
    formatting::format_date,
    parsing::parse_date,
};
use google_calendar::client::structs::Meeting;
use miette::Result;

use crate::{
    commands::fetch::{
        google_calendar::formatting::format_meetings,
        outcome::{
            FetchOutcome,
            WorkFileChange,
            record_fetch_outcome,
        },
        sources::FetchSource,
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
        resolve_selected_date,
    },
};

const GOOGLE_CALENDAR_SECTION_TITLE: &str = "Google Calendar";

#[derive(Debug, Args)]
#[group(multiple = false)]
pub struct GoogleCalendarArgs {
    /// Date to fetch Google Calendar meetings for in the format MM-DD-YYYY.
    #[arg(long, value_name = DATE_VALUE_NAME, value_parser = parse_date)]
    date: Option<NaiveDate>,

    /// Fetch the meetings you had yesterday.
    #[arg(long = "yesterday")]
    use_yesterday: bool,

    /// Fetch the meetings you had last Friday.
    #[arg(long = "last-friday")]
    use_last_friday: bool,
}

impl GoogleCalendarArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(DateFlags {
            date: self.date,
            use_yesterday: self.use_yesterday,
            use_last_friday: self.use_last_friday,
        });

        fetch_google_calendar_meetings(environment, date_selection).await
    }
}

pub async fn fetch_google_calendar_meetings(
    environment: &Environment,
    date_selection: DateSelection,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::GoogleCalendar.print_fetching_notice();

    let fetch_outcome = collect_google_calendar_meetings(environment, date_selection).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_google_calendar_meetings(
    environment: &Environment,
    date_selection: DateSelection,
) -> Result<FetchOutcome> {
    let meeting_date =
        resolve_selected_date(date_selection, environment.today)?.unwrap_or(environment.today);

    let google_calendar_client = environment.build_google_calendar_client()?;

    let meetings =
        google_calendar_client.get_primary_calendar_meetings_on_date(&meeting_date).await?;

    let google_calendar_fetch_outcome = get_google_calendar_fetch_outcome(&meetings, meeting_date);

    Ok(google_calendar_fetch_outcome)
}

fn get_google_calendar_fetch_outcome(
    meetings: &[Meeting],
    meeting_date: NaiveDate,
) -> FetchOutcome {
    if meetings.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection {
                section_title: GOOGLE_CALENDAR_SECTION_TITLE,
            },
            summary: format!("No meetings found for {}.", format_date(&meeting_date)),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection {
            section_title: GOOGLE_CALENDAR_SECTION_TITLE,
            content: format_meetings(meetings),
        },
        summary: format!(
            "Added {} meetings from {} to your work file.",
            meetings.len(),
            format_date(&meeting_date)
        ),
    }
}
