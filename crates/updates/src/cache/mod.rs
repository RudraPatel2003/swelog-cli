use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    process,
};

use chrono::{
    DateTime,
    TimeDelta,
    Utc,
};
use highlight::stderr::path_link;
use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use serde::{
    Deserialize,
    Serialize,
};

const VERSION_CACHE_FILE_NAME: &str = "version-check.json";

const VERSION_CHECK_INTERVAL: TimeDelta = TimeDelta::days(1);

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct VersionCache {
    pub latest_version: String,

    pub checked_at: DateTime<Utc>,
}

#[must_use]
pub fn get_version_cache_file_path(cache_directory: &Path) -> PathBuf {
    cache_directory.join(VERSION_CACHE_FILE_NAME)
}

pub fn read_version_cache(cache_file_path: &Path) -> Result<VersionCache> {
    let cache_file_contents =
        fs::read_to_string(cache_file_path).into_diagnostic().wrap_err_with(|| {
            format!("failed to read the version cache at {}", path_link(cache_file_path))
        })?;

    let version_cache = serde_json::from_str(&cache_file_contents)
        .into_diagnostic()
        .wrap_err("failed to parse the version cache")?;

    Ok(version_cache)
}

pub fn write_version_cache(cache_file_path: &Path, version_cache: &VersionCache) -> Result<()> {
    if let Some(parent) = cache_file_path.parent() {
        fs::create_dir_all(parent).into_diagnostic().wrap_err_with(|| {
            format!("failed to create the version cache directory at {}", path_link(parent))
        })?;
    }

    let json = serde_json::to_string(version_cache)
        .into_diagnostic()
        .wrap_err("failed to serialize the version cache")?;

    let temporary_file_path = cache_file_path.with_extension(format!("{}.tmp", process::id()));

    fs::write(&temporary_file_path, json).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the version cache at {}", path_link(&temporary_file_path))
    })?;

    fs::rename(&temporary_file_path, cache_file_path).into_diagnostic().wrap_err_with(|| {
        format!("failed to write the version cache at {}", path_link(cache_file_path))
    })?;

    Ok(())
}

#[must_use]
pub fn is_refresh_due(version_cache: Option<&VersionCache>, now: DateTime<Utc>) -> bool {
    version_cache.is_none_or(|version_cache| {
        now.signed_duration_since(version_cache.checked_at) >= VERSION_CHECK_INTERVAL
    })
}

#[cfg(test)]
mod tests;
