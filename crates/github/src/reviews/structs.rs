use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Review {
    pub user: Option<ReviewAuthor>,

    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewAuthor {
    pub login: String,
}

impl Review {
    pub fn is_submitted_by(&self, github_username: &str) -> bool {
        self.user.as_ref().is_some_and(|author| author.login.eq_ignore_ascii_case(github_username))
    }

    pub fn is_submitted_on(&self, activity_date: NaiveDate) -> bool {
        self.submitted_at.is_some_and(|submitted_at| submitted_at.date_naive() == activity_date)
    }
}
