use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SentryIssue {
    pub id: String,

    pub short_id: String,

    pub title: String,

    pub permalink: String,

    #[serde(default)]
    pub substatus: Option<SentryIssueSubstatus>,
}

/// How Sentry classifies an unresolved issue.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SentryIssueSubstatus {
    Escalating,
    Regressed,
    New,
    Ongoing,

    #[serde(other)]
    Other,
}
