/// Returns the portion of `markdown` before `index`, or an empty string when
/// `index` is not a character boundary.
pub fn slice_up_to(markdown: &str, index: usize) -> &str {
    markdown
        .get(..index)
        .unwrap_or_default()
}

/// Returns the portion of `markdown` starting at `index`, or an empty string
/// when `index` is not a character boundary.
pub fn slice_from(markdown: &str, index: usize) -> &str {
    markdown
        .get(index..)
        .unwrap_or_default()
}
