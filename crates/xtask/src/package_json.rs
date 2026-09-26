mod parsing;

use std::fs;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};

use crate::package_json::parsing::{
    parse_package_json_version,
    replace_package_json_version,
};

pub const NPM_PACKAGE_JSON_PATH: &str = "./npm/package.json";

pub const DOCS_PACKAGE_JSON_PATH: &str = "./docs/package.json";

pub fn read_package_json_version(path: &str) -> Result<String> {
    let package_json = read_package_json(path)?;

    parse_package_json_version(&package_json)
        .wrap_err_with(|| format!("failed to read the version in {path}"))
}

pub fn write_package_json_version(path: &str, release_version: &str) -> Result<()> {
    let package_json = read_package_json(path)?;

    let updated_package_json = replace_package_json_version(&package_json, release_version)
        .wrap_err_with(|| format!("failed to update the version in {path}"))?;

    fs::write(path, updated_package_json)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write {path}"))
}

fn read_package_json(path: &str) -> Result<String> {
    fs::read_to_string(path).into_diagnostic().wrap_err_with(|| format!("failed to read {path}"))
}
