#[cfg(test)]
#[path = "release_version_tests.rs"]
mod tests;

use miette::{
    Result,
    miette,
};
use semver::Version;

pub fn parse_release_version(release_version: &str) -> Result<Version> {
    Version::parse(release_version)
        .map_err(|_| miette!("`{release_version}` is not a valid semantic version like 1.2.3"))
}
