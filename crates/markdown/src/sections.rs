mod slicing;

use std::ops::Range;

use pulldown_cmark::{
    Event,
    HeadingLevel,
    Parser,
    Tag,
    TagEnd,
};
use slicing::{
    slice_from,
    slice_up_to,
};

struct Heading {
    level: HeadingLevel,
    title: String,
    range: Range<usize>,
}

#[must_use]
pub fn format_section(section_title: &str, content: &str) -> String {
    format!("## {section_title}\n\n{}", content.trim_matches('\n'))
}

/// Returns the byte range covering the `## {section_title}` heading and
/// everything under it, up to the next heading of the same or a higher level.
#[must_use]
pub fn find_section_bounds(markdown: &str, section_title: &str) -> Option<Range<usize>> {
    let headings = collect_headings(markdown);

    let heading_index = headings
        .iter()
        .position(|heading| heading.level == HeadingLevel::H2 && heading.title == section_title)?;

    let heading = headings.get(heading_index)?;

    let end = headings
        .iter()
        .skip(heading_index.saturating_add(1))
        .find(|candidate| heading_level_number(candidate.level) <= 2)
        .map_or(markdown.len(), |candidate| candidate.range.start);

    Some(heading.range.start..end)
}

#[must_use]
pub fn replace_block(markdown: &str, range: Range<usize>, block: &str) -> String {
    let prefix = slice_up_to(markdown, range.start).trim_end_matches('\n');

    let suffix = slice_from(markdown, range.end).trim_start_matches('\n');

    match (prefix.is_empty(), suffix.is_empty()) {
        (true, true) => format!("{block}\n"),
        (true, false) => format!("{block}\n\n{suffix}"),
        (false, true) => format!("{prefix}\n\n{block}\n"),
        (false, false) => format!("{prefix}\n\n{block}\n\n{suffix}"),
    }
}

#[must_use]
pub fn remove_block(markdown: &str, range: Range<usize>) -> String {
    let prefix = slice_up_to(markdown, range.start).trim_end_matches('\n');

    let suffix = slice_from(markdown, range.end).trim_start_matches('\n');

    match (prefix.is_empty(), suffix.is_empty()) {
        (true, true) => String::new(),
        (true, false) => suffix.to_owned(),
        (false, true) => format!("{prefix}\n"),
        (false, false) => format!("{prefix}\n\n{suffix}"),
    }
}

fn collect_headings(markdown: &str) -> Vec<Heading> {
    let mut headings = Vec::new();

    let mut current_heading: Option<Heading> = None;

    for (event, range) in Parser::new(markdown).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_heading = Some(Heading { level, title: String::new(), range });
            }

            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut current_heading {
                    heading.title.push_str(&text);
                }
            }

            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut heading) = current_heading.take() {
                    heading.range.end = range.end;

                    headings.push(heading);
                }
            }

            _ => {}
        }
    }

    headings
}

const fn heading_level_number(heading_level: HeadingLevel) -> u8 {
    match heading_level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}
