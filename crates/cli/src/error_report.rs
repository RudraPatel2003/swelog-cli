use miette::{
    MietteHandlerOpts,
    Result,
};

/// Do not wrap miette errors
/// Doing so breaks terminal hyperlinks to files
pub fn configure_miette_error_handling() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        let miette_options = MietteHandlerOpts::new().wrap_lines(false).build();

        Box::new(miette_options)
    }))?;

    Ok(())
}
