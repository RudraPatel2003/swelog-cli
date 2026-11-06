pub mod structs;

use base_url::base_url::BaseUrl;
use chrono::{
    Local,
    NaiveDate,
};
use http_api::{
    api_service::ApiService,
    client::ApiClient,
};
use miette::Result;
use reqwest::{
    Response,
    StatusCode,
};

use crate::{
    client::structs::{
        CalendarEvent,
        CalendarEventPage,
        Meeting,
    },
    day_window::{
        DayWindow,
        get_day_window,
    },
    errors::UnsupportedGoogleCalendarResponse,
    meetings::collect_meetings,
    oauth::GoogleAuthorization,
};

pub const DEFAULT_GOOGLE_CALENDAR_API_BASE_URL: &str = "https://www.googleapis.com/";

const PRIMARY_CALENDAR_EVENTS_ENDPOINT_PATH: &str = "calendar/v3/calendars/primary/events";

const PAGE_SIZE: &str = "250";

// Only fetch what is needed
const EVENT_FIELDS: &str = "nextPageToken,items(summary,status,start(dateTime),end(dateTime),attendees(self,responseStatus))";

const GOOGLE_CALENDAR_API_SERVICE: ApiService = ApiService {
    label: "Google Calendar",
    credential_name: "authorization",
    credential_command_name: "google-calendar",
    authorization_hint: "check that the Google Calendar API is enabled for the swelog OAuth client",
    accept_header: None,
};

pub struct GoogleCalendarClient {
    api_client: ApiClient,
    authorization: GoogleAuthorization,
}

impl GoogleCalendarClient {
    #[must_use]
    pub fn new(api_base_url: BaseUrl, authorization: GoogleAuthorization) -> Self {
        Self {
            api_client: ApiClient::new(api_base_url, GOOGLE_CALENDAR_API_SERVICE),
            authorization,
        }
    }

    pub async fn get_primary_calendar_meetings_on_date(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<Meeting>> {
        let day_window = get_day_window(*date, &Local)?;

        let events = self
            .fetch_all_events(&day_window)
            .await?;

        let meetings = collect_meetings(&events);

        Ok(meetings)
    }

    async fn fetch_all_events(&self, day_window: &DayWindow) -> Result<Vec<CalendarEvent>> {
        let mut events = Vec::new();

        let mut page_token: Option<String> = None;

        loop {
            let mut page = self
                .fetch_event_page(day_window, page_token.as_deref())
                .await?;

            events.append(&mut page.items);

            let Some(next_page_token) = page.take_next_page_token() else {
                return Ok(events);
            };

            if page_token.as_deref() == Some(next_page_token.as_str()) {
                let unsupported_google_calendar_response_error =
                    UnsupportedGoogleCalendarResponse {
                        message: "Google Calendar returned a repeated pagination token".to_string(),
                    };

                return Err(unsupported_google_calendar_response_error.into());
            }

            page_token = Some(next_page_token);
        }
    }

    async fn fetch_event_page(
        &self,
        day_window: &DayWindow,
        page_token: Option<&str>,
    ) -> Result<CalendarEventPage> {
        let response_text = self
            .request_event_page_reauthorizing_once(day_window, page_token)
            .await?;

        parse_event_page(&response_text)
    }

    async fn request_event_page_reauthorizing_once(
        &self,
        day_window: &DayWindow,
        page_token: Option<&str>,
    ) -> Result<String> {
        let mut reauthorization_attempted = false;

        loop {
            let access_token = self
                .authorization
                .get_access_token_authorizing_if_needed()
                .await?;

            let response = self
                .send_event_page_request(day_window, page_token, &access_token)
                .await?;

            if response.status() == StatusCode::UNAUTHORIZED && !reauthorization_attempted {
                self.authorization
                    .clear()?;

                reauthorization_attempted = true;

                continue;
            }

            return self
                .api_client
                .read_successful_response_body(response)
                .await;
        }
    }

    async fn send_event_page_request(
        &self,
        day_window: &DayWindow,
        page_token: Option<&str>,
        access_token: &str,
    ) -> Result<Response> {
        let query_parameters = get_event_page_query_parameters(day_window, page_token);

        self.api_client
            .send_get_request(
                PRIMARY_CALENDAR_EVENTS_ENDPOINT_PATH,
                &query_parameters,
                access_token,
            )
            .await
    }
}

fn parse_event_page(response_text: &str) -> Result<CalendarEventPage> {
    serde_json::from_str(response_text).map_err(|error| {
        UnsupportedGoogleCalendarResponse {
            message: format!("the event response did not match the expected shape: {error}"),
        }
        .into()
    })
}

fn get_event_page_query_parameters(
    day_window: &DayWindow,
    page_token: Option<&str>,
) -> Vec<(&'static str, String)> {
    let mut query_parameters = vec![
        (
            "timeMin",
            day_window
                .time_minimum
                .clone(),
        ),
        (
            "timeMax",
            day_window
                .time_maximum
                .clone(),
        ),
        // Ensure recurring meetings are returned as instances rather than recurrence rules
        ("singleEvents", "true".to_string()),
        ("orderBy", "startTime".to_string()),
        ("showDeleted", "true".to_string()),
        ("maxResults", PAGE_SIZE.to_string()),
        ("fields", EVENT_FIELDS.to_string()),
    ];

    if let Some(page_token) = page_token {
        query_parameters.push(("pageToken", page_token.to_string()));
    }

    query_parameters
}

#[cfg(test)]
#[path = "client_tests.rs"]
mod tests;
