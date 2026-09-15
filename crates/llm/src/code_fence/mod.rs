const CODE_FENCE: &str = "```";

#[must_use]
pub fn strip_markdown_code_fence(response: &str) -> &str {
    let trimmed_response = response.trim();

    let Some((opening_line, fenced_content)) = trimmed_response.split_once('\n') else {
        return response;
    };

    if !is_code_fence_opening_line(opening_line) {
        return response;
    }

    strip_closing_code_fence(fenced_content).unwrap_or(response)
}

fn is_code_fence_opening_line(line: &str) -> bool {
    let Some(language_tag) = line.trim_end().strip_prefix(CODE_FENCE) else {
        return false;
    };

    language_tag.chars().all(char::is_alphanumeric)
}

fn strip_closing_code_fence(fenced_content: &str) -> Option<&str> {
    let content_before_closing_fence = fenced_content.trim_end().strip_suffix(CODE_FENCE)?;

    let closing_fence_stands_alone =
        content_before_closing_fence.is_empty() || content_before_closing_fence.ends_with('\n');

    closing_fence_stands_alone.then(|| content_before_closing_fence.trim())
}

#[cfg(test)]
mod tests;
