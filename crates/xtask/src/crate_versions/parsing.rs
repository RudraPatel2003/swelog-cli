#[cfg(test)]
#[path = "parsing_tests.rs"]
mod tests;

use miette::{
    Result,
    miette,
};

pub fn parse_crate_version(manifest: &str) -> Result<&str> {
    let version_line = find_version_line(manifest)?;

    let Some(version) = version_line.split('"').nth(1) else {
        return Err(miette!("could not read a quoted version from `{version_line}`"));
    };

    Ok(version)
}

pub fn replace_crate_version(manifest: &str, release_version: &str) -> Result<String> {
    let mut lines: Vec<String> = manifest.lines().map(str::to_string).collect();

    let version_line = lines
        .iter_mut()
        .find(|line| is_version_line(line))
        .ok_or_else(|| miette!("no version line found"))?;

    *version_line = format!("version = \"{release_version}\"");

    let mut updated_manifest = lines.join("\n");

    updated_manifest.push('\n');

    Ok(updated_manifest)
}

fn find_version_line(manifest: &str) -> Result<&str> {
    manifest
        .lines()
        .find(|line| is_version_line(line))
        .ok_or_else(|| miette!("no version line found"))
}

fn is_version_line(line: &str) -> bool {
    line.trim_start().starts_with("version =")
}
