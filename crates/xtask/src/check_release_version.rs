use miette::{
    Result,
    miette,
};

use crate::{
    crate_versions::{
        list_crate_manifest_paths,
        read_cli_version,
        read_crate_version,
    },
    package_json::{
        DOCS_PACKAGE_JSON_PATH,
        NPM_PACKAGE_JSON_PATH,
        read_package_json_version,
    },
};

struct VersionedFile {
    path: String,
    version: String,
}

pub fn run_check_release_version() -> Result<()> {
    let release_version = read_cli_version()?;

    let versioned_files = read_versioned_files()?;

    let mismatches = describe_version_mismatches(&release_version, &versioned_files);

    if !mismatches.is_empty() {
        return Err(miette!(
            "expected every version to be {release_version}, but found:\n{}",
            mismatches.join("\n")
        ));
    }

    println!("All crate and package versions match {release_version}");

    Ok(())
}

fn read_versioned_files() -> Result<Vec<VersionedFile>> {
    let mut versioned_files = Vec::new();

    for manifest_path in list_crate_manifest_paths()? {
        versioned_files.push(VersionedFile {
            path: manifest_path
                .display()
                .to_string(),
            version: read_crate_version(&manifest_path)?,
        });
    }

    for package_json_path in [NPM_PACKAGE_JSON_PATH, DOCS_PACKAGE_JSON_PATH] {
        versioned_files.push(VersionedFile {
            path: package_json_path.to_string(),
            version: read_package_json_version(package_json_path)?,
        });
    }

    Ok(versioned_files)
}

fn describe_version_mismatches(
    release_version: &str,
    versioned_files: &[VersionedFile],
) -> Vec<String> {
    versioned_files
        .iter()
        .filter(|file| file.version != release_version)
        .map(|file| format!("  {} is {}", file.path, file.version))
        .collect()
}
