use std::{
    fmt::Display,
    path::Path,
};

use owo_colors::Stream;
use supports_hyperlinks::Stream as HyperlinkStream;

use crate::{
    link::format_path_link,
    style::{
        cyan,
        dimmed,
        green,
        highlight_with_style,
        yellow,
    },
};

pub fn highlight_stderr_cyan(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stderr, cyan())
}

pub fn highlight_stderr_yellow(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stderr, yellow())
}

pub fn highlight_stderr_green(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stderr, green())
}

pub fn highlight_stderr_dimmed(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stderr, dimmed())
}

#[must_use]
pub fn stderr_path_link(path: &Path) -> String {
    let formatted_path_link = format_path_link(path, HyperlinkStream::Stderr);

    highlight_stderr_cyan(formatted_path_link)
}
