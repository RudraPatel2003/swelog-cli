mod parsing;

use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::crate_versions::parsing::{
    parse_crate_version,
    replace_crate_version,
};

const CRATES_DIRECTORY: &str = "./crates";

const CLI_MANIFEST_PATH: &str = "./crates/cli/Cargo.toml";

pub fn read_cli_version() -> Result<String> {
    let cli_crate_manifest_path = Path::new(CLI_MANIFEST_PATH);

    read_crate_version(cli_crate_manifest_path)
}

pub fn list_crate_manifest_paths() -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(CRATES_DIRECTORY)
        .into_diagnostic()
        .wrap_err("failed to read crates directory")?;

    let mut manifest_paths = Vec::new();

    for entry in entries {
        let entry = entry.into_diagnostic().wrap_err("failed to read crates directory entry")?;

        let manifest_path = entry.path().join("Cargo.toml");

        if manifest_path.is_file() {
            manifest_paths.push(manifest_path);
        }
    }

    manifest_paths.sort();

    Ok(manifest_paths)
}

pub fn read_crate_version(manifest_path: &Path) -> Result<String> {
    let manifest = read_manifest(manifest_path)?;

    let version = parse_crate_version(&manifest)
        .wrap_err_with(|| format!("failed to read the version in {}", manifest_path.display()))?;

    Ok(version.to_string())
}

pub fn write_crate_version(manifest_path: &Path, release_version: &str) -> Result<()> {
    let manifest = read_manifest(manifest_path)?;

    let updated_manifest = replace_crate_version(&manifest, release_version)
        .wrap_err_with(|| format!("failed to update the version in {}", manifest_path.display()))?;

    fs::write(manifest_path, updated_manifest)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {}", manifest_path.display()))
}

fn read_manifest(manifest_path: &Path) -> Result<String> {
    fs::read_to_string(manifest_path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", manifest_path.display()))
}
