use reqwest::{
    Response,
    header::LINK,
};

use crate::pagination::parse_next_cursor;

pub struct ResponsePage {
    pub body: String,
    pub next_cursor: Option<String>,
}

pub fn read_next_cursor(response: &Response) -> Option<String> {
    let link_header = response
        .headers()
        .get(LINK)?
        .to_str()
        .ok()?;

    parse_next_cursor(link_header)
}
