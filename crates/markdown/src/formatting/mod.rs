use dprint_plugin_markdown::{
    configuration::ConfigurationBuilder,
    format_text,
};

#[must_use]
pub fn format_markdown(markdown: &str) -> String {
    let configuration = ConfigurationBuilder::new().build();

    let formatted_markdown =
        format_text(markdown, &configuration, |_language, _code, _line_width| Ok(None));

    match formatted_markdown {
        Ok(Some(formatted_markdown)) => formatted_markdown,

        Ok(None) | Err(_) => markdown.to_owned(),
    }
}

#[cfg(test)]
mod tests;
