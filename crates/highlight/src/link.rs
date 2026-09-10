use std::{
    io::IsTerminal,
    path::Path,
};

use supports_hyperlinks::Stream;
use url::Url;

/// Format links printed to the terminal so that they are clickable
pub fn format_path_link(path: &Path, stream: Stream) -> String {
    let displayed_path = path.display();

    let Ok(url) = Url::from_file_path(path) else {
        return displayed_path.to_string();
    };

    if supports_hyperlinks::on(stream) {
        return format_as_osc_8_hyperlink(&url, path);
    }

    if is_terminal(stream) {
        return url.to_string();
    }

    displayed_path.to_string()
}

fn is_terminal(stream: Stream) -> bool {
    match stream {
        Stream::Stdout => std::io::stdout().is_terminal(),
        Stream::Stderr => std::io::stderr().is_terminal(),
    }
}

fn format_as_osc_8_hyperlink(url: &Url, path: &Path) -> String {
    let displayed_path = path.display();

    format!("\x1b]8;;{url}\x1b\\{displayed_path}\x1b]8;;\x1b\\")
}
