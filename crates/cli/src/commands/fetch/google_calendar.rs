mod formatting;

use async_trait::async_trait;
use chrono::NaiveDate;
use config::swelog_config::SwelogConfig;
use credentials::credential::Credential;
use dates::formatting::format_date;
use google_calendar::client::structs::Meeting;
use miette::Result;

use crate::{
    commands::fetch::{
        google_calendar::formatting::format_meetings,
        integration::{
            Integration,
            IntegrationSpec,
        },
        outcome::{
            FetchOutcome,
            WorkFileChange,
        },
        window::FetchWindow,
    },
    environment::Environment,
};

const GOOGLE_CALENDAR_SPEC: IntegrationSpec = IntegrationSpec {
    label: "Google Calendar",
    fetching_notice: "Fetching Google Calendar events...",
    section_title: "Google Calendar",
    credential: Credential::GoogleCalendar,
};

pub struct GoogleCalendarIntegration;

#[async_trait]
impl Integration for GoogleCalendarIntegration {
    fn spec(&self) -> &'static IntegrationSpec {
        &GOOGLE_CALENDAR_SPEC
    }

    async fn collect_fetch_outcome(
        &self,
        environment: &Environment,
        _swelog_config: &SwelogConfig,
        fetch_window: FetchWindow,
    ) -> Result<FetchOutcome> {
        let meeting_date = fetch_window.date_or_today(environment.today);

        let google_calendar_client = environment.build_google_calendar_client()?;

        let meetings = google_calendar_client
            .get_primary_calendar_meetings_on_date(&meeting_date)
            .await?;

        let google_calendar_fetch_outcome =
            get_google_calendar_fetch_outcome(&meetings, meeting_date);

        Ok(google_calendar_fetch_outcome)
    }
}

fn get_google_calendar_fetch_outcome(
    meetings: &[Meeting],
    meeting_date: NaiveDate,
) -> FetchOutcome {
    if meetings.is_empty() {
        return FetchOutcome {
            work_file_change: WorkFileChange::RemoveSection,
            summary: format!("No meetings found for {}.", format_date(&meeting_date)),
        };
    }

    FetchOutcome {
        work_file_change: WorkFileChange::UpsertSection { content: format_meetings(meetings) },
        summary: format!(
            "Added {} meetings from {} to your work file.",
            meetings.len(),
            format_date(&meeting_date)
        ),
    }
}
