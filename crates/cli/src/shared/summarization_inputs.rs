use config::setup::swelog_paths::SwelogPaths;
use markdown::{
    context_file::format_context_file,
    work_file::format_work_file,
};
use miette::Result;

pub fn format_summarization_inputs(swelog_paths: &SwelogPaths) -> Result<()> {
    format_work_file(&swelog_paths.work_file)?;

    format_context_file(&swelog_paths.context_file)
}
