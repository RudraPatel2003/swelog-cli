use url::Url;

const CURSOR_QUERY_KEY: &str = "cursor";

/// Sentry paginates with the `Link` header standard. The `rel="next"` entry
/// carries `results="false"` on the last page.
pub fn parse_next_cursor(link_header: &str) -> Option<String> {
    link_header
        .split('<')
        .skip(1)
        .find_map(parse_next_link_cursor)
}

fn parse_next_link_cursor(link: &str) -> Option<String> {
    let (link_url, link_parameters) = link.split_once('>')?;

    if !is_next_link_with_results(link_parameters) {
        return None;
    }

    get_cursor_query_value(link_url)
}

fn is_next_link_with_results(link_parameters: &str) -> bool {
    link_parameters.contains(r#"rel="next""#) && link_parameters.contains(r#"results="true""#)
}

fn get_cursor_query_value(link_url: &str) -> Option<String> {
    Url::parse(link_url)
        .ok()?
        .query_pairs()
        .find(|(key, _)| key == CURSOR_QUERY_KEY)
        .map(|(_, value)| value.into_owned())
}

#[cfg(test)]
#[path = "pagination_tests.rs"]
mod tests;
