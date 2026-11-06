mod formatting;

use chrono::NaiveDate;
use clap::Args;
use config::config_file::read_config_file;
use dates::formatting::format_date;
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
        window::FetchWindow,
    },
    environment::Environment,
    shared::date_selection::{
        DateFlags,
        DateSelection,
    },
};

const GOOGLE_CALENDAR_SECTION_TITLE: &str = "Google Calendar";

#[derive(Debug, Args)]
pub struct GoogleCalendarArgs {
    #[command(flatten)]
    date_flags: DateFlags,
}

impl GoogleCalendarArgs {
    pub async fn run(self, environment: &Environment) -> Result<()> {
        let date_selection = DateSelection::from_date_flags(self.date_flags);

        let fetch_window = FetchWindow::from_date_selection(date_selection, environment.today)?;

        fetch_google_calendar_meetings(environment, fetch_window).await
    }
}

pub async fn fetch_google_calendar_meetings(
    environment: &Environment,
    fetch_window: FetchWindow,
) -> Result<()> {
    let swelog_config = read_config_file(&environment.config_file_path)?;

    FetchSource::GoogleCalendar.print_fetching_notice();

    let fetch_outcome = collect_google_calendar_meetings(environment, fetch_window).await?;

    record_fetch_outcome(&swelog_config, fetch_outcome)
}

pub async fn collect_google_calendar_meetings(
    environment: &Environment,
    fetch_window: FetchWindow,
) -> Result<FetchOutcome> {
    let meeting_date = fetch_window.date_or_today(environment.today);

    let google_calendar_client = environment.build_google_calendar_client()?;

    let meetings = google_calendar_client
        .get_primary_calendar_meetings_on_date(&meeting_date)
        .await?;

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
