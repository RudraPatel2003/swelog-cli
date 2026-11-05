#![allow(
    clippy::expect_used,
    reason = "test fixtures fail loudly on setup problems, the way `#[test]` functions do"
)]

mod support;

mod auth;
mod config;
mod daily_log;
mod fetch_all;
mod fetch_github;
mod fetch_google_calendar;
mod fetch_linear;
mod fetch_sentry;
mod summarize;
mod update_check;
