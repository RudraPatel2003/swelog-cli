use highlight::stderr::{
    highlight_cyan,
    highlight_dimmed,
};
use semver::Version;

const UPGRADE_COMMAND: &str = "npm update -g swelog-cli";

#[must_use]
pub fn get_update_notice(current_version: &str, latest_version: Option<&str>) -> Option<String> {
    let latest_version = latest_version?;

    if !is_newer_version(current_version, latest_version) {
        return None;
    }

    Some(format_update_notice(current_version, latest_version))
}

fn is_newer_version(current_version: &str, latest_version: &str) -> bool {
    let (Ok(current_version), Ok(latest_version)) =
        (Version::parse(current_version), Version::parse(latest_version))
    else {
        return false;
    };

    latest_version > current_version
}

fn format_update_notice(current_version: &str, latest_version: &str) -> String {
    format!(
        "A new version of swelog is available: {} → {}\nRun {} to upgrade.\n",
        highlight_dimmed(current_version),
        highlight_cyan(latest_version),
        highlight_cyan(UPGRADE_COMMAND),
    )
}

#[cfg(test)]
#[path = "notice_tests.rs"]
mod tests;
