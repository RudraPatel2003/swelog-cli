use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LinearIssue {
    #[serde(rename = "id")]
    pub identifier: String,

    pub title: String,

    pub url: String,

    #[serde(rename = "status")]
    pub status_name: String,

    pub status_type: LinearStatusType,

    #[serde(flatten)]
    pub timestamps: LinearIssueTimestamps,
}

/// The moments Linear records on an issue, used to decide which day an issue
/// belongs to.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LinearIssueTimestamps {
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub started_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub completed_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub canceled_at: Option<DateTime<Utc>>,

    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LinearStatusType {
    Backlog,
    Unstarted,
    Started,
    Completed,

    #[serde(alias = "cancelled")]
    Canceled,

    #[serde(other)]
    Other,
}

impl LinearStatusType {
    #[must_use]
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::Completed | Self::Canceled)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearIssuePage {
    #[serde(default)]
    pub issues: Vec<LinearIssue>,

    /// The Linear MCP server returns this as `cursor`; the alias keeps older
    /// `nextCursor` responses working.
    #[serde(default, rename = "cursor", alias = "nextCursor")]
    pub next_cursor: Option<String>,

    #[serde(default)]
    pub has_next_page: Option<bool>,
}

impl LinearIssuePage {
    /// Returns the cursor for the following page, or `None` when this is the
    /// last page.
    pub fn take_next_cursor(&mut self) -> Option<String> {
        if self.has_next_page == Some(false) {
            return None;
        }

        self.next_cursor
            .take()
            .filter(|next_cursor| !next_cursor.is_empty())
    }
}
