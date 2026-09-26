const CODE_FENCE: &str = "```";

#[must_use]
pub fn strip_markdown_code_fence(response: &str) -> &str {
    let trimmed_response = response.trim();

    let Some((first_line, remaining_lines)) = trimmed_response.split_once('\n') else {
        return response;
    };

    let Some((fenced_content, last_line)) = remaining_lines.rsplit_once('\n') else {
        return response;
    };

    if !is_code_fence_line(first_line) || !is_code_fence_line(last_line) {
        return response;
    }

    fenced_content.trim()
}

fn is_code_fence_line(line: &str) -> bool {
    line.trim().starts_with(CODE_FENCE)
}

#[cfg(test)]
#[path = "code_fence_tests.rs"]
mod tests;
