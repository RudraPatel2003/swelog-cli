use config::{
    setup::swelog_paths::SwelogPaths,
    swelog_config::SwelogConfig,
};
use markdown::work_file::{
    format_work_file,
    remove_work_file_section_from_config,
    upsert_work_file_section_from_config,
};
use miette::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum WorkFileChange {
    UpsertSection { section_title: &'static str, content: String },

    RemoveSection { section_title: &'static str },
}

#[derive(Debug, PartialEq, Eq)]
pub struct FetchOutcome {
    pub work_file_change: WorkFileChange,
    pub summary: String,
}

pub fn record_fetch_outcome(
    swelog_config: &SwelogConfig,
    fetch_outcome: FetchOutcome,
) -> Result<()> {
    apply_work_file_change(swelog_config, fetch_outcome.work_file_change)?;

    let swelog_paths = SwelogPaths::new(swelog_config);

    format_work_file(&swelog_paths.work_file)?;

    println!("{}", fetch_outcome.summary);

    Ok(())
}

fn apply_work_file_change(
    swelog_config: &SwelogConfig,
    work_file_change: WorkFileChange,
) -> Result<()> {
    match work_file_change {
        WorkFileChange::UpsertSection { section_title, content } => {
            upsert_work_file_section_from_config(swelog_config, section_title, &content)
        }

        WorkFileChange::RemoveSection { section_title } => {
            remove_work_file_section_from_config(swelog_config, section_title)
        }
    }
}
