use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SentryIssueDetails {
    pub activity: Vec<SentryActivityEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SentryActivityEntry {
    #[serde(rename = "type")]
    pub activity_type: SentryActivityType,

    pub date_created: DateTime<Utc>,
}

/// The only activity types that we care about
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SentryActivityType {
    SetResolved,
    SetResolvedInRelease,
    SetResolvedInCommit,

    #[serde(other)]
    Other,
}

impl SentryActivityType {
    #[must_use]
    pub const fn is_resolution(self) -> bool {
        matches!(self, Self::SetResolved | Self::SetResolvedInRelease | Self::SetResolvedInCommit)
    }
}
