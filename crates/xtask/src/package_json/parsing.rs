#[cfg(test)]
#[path = "parsing_tests.rs"]
mod tests;

use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
    miette,
};
use serde_json::Value;

pub fn parse_package_json_version(package_json: &str) -> Result<String> {
    let package_json = parse_package_json(package_json)?;

    let Some(version) = package_json.get("version").and_then(Value::as_str) else {
        return Err(miette!("no version found"));
    };

    Ok(version.to_string())
}

pub fn replace_package_json_version(package_json: &str, release_version: &str) -> Result<String> {
    let mut package_json = parse_package_json(package_json)?;

    let Some(package_json_object) = package_json.as_object_mut() else {
        return Err(miette!("expected a JSON object"));
    };

    package_json_object.insert(String::from("version"), Value::String(release_version.to_string()));

    let mut updated_package_json = serde_json::to_string_pretty(&package_json)
        .into_diagnostic()
        .wrap_err("failed to serialize")?;

    updated_package_json.push('\n');

    Ok(updated_package_json)
}

fn parse_package_json(package_json: &str) -> Result<Value> {
    serde_json::from_str(package_json).into_diagnostic().wrap_err("failed to parse")
}
