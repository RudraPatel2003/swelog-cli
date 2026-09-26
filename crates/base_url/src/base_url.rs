use std::{
    fmt::{
        Display,
        Formatter,
        Result as FormatResult,
    },
    str::FromStr,
};

use miette::Result;
use url::Url;

use crate::errors::InvalidBaseUrl;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseUrl {
    url: Url,
}

impl BaseUrl {
    pub fn parse(value: &str) -> Result<Self, InvalidBaseUrl> {
        let mut url = Url::parse(value).map_err(|error| InvalidBaseUrl {
            value: value.to_string(),
            message: error.to_string(),
        })?;

        if url.cannot_be_a_base() {
            let invalid_base_url = InvalidBaseUrl {
                value: value.to_string(),
                message: "the URL cannot have paths joined onto it".to_string(),
            };

            return Err(invalid_base_url);
        }

        ensure_trailing_slash(&mut url);

        let base_url = Self { url };

        Ok(base_url)
    }

    pub fn join(&self, endpoint_path: &str) -> Result<Url> {
        let endpoint_url = self.url.join(endpoint_path).map_err(|error| InvalidBaseUrl {
            value: format!("{}{endpoint_path}", self.url),
            message: error.to_string(),
        })?;

        Ok(endpoint_url)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

fn ensure_trailing_slash(url: &mut Url) {
    if !url.path().ends_with('/') {
        let path_with_trailing_slash = format!("{}/", url.path());

        url.set_path(&path_with_trailing_slash);
    }
}

impl FromStr for BaseUrl {
    type Err = InvalidBaseUrl;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Display for BaseUrl {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{}", self.url)
    }
}

#[cfg(test)]
#[path = "base_url_tests.rs"]
mod tests;
