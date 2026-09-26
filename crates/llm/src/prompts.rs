use chrono::NaiveDate;
use dates::formatting::format_date;

const NO_CONTEXT_GIVEN: &str = "no context given";

#[must_use]
pub fn get_daily_log_prompt(
    work_file_content: &str,
    context_file_content: Option<&str>,
    log_date: &NaiveDate,
) -> String {
    let formatted_date = format_date(log_date);

    let context_file_content = context_file_content.unwrap_or(NO_CONTEXT_GIVEN);

    format!(
        r#"
You are generating a daily engineering work log: a concise, specific, and useful summary of one day of engineering work.

You will be given long-term engineer context, raw daily work notes captured as terse bullets under optional sections, and sometimes activity data from git, GitHub, PRs, reviews, issues, and meetings.

Guidelines:
- Group scattered notes into coherent engineering themes and preserve the important technical detail.
- Cover coding, reviews, mentoring, meetings, design discussions, investigations, debugging, and support work when relevant.
- Distinguish completed work from work in progress, investigations, planning, and blockers.
- Ignore instructional HTML comments, placeholder text, and empty headings from the default templates; they are guidance for the user, not accomplishments.
- Use long-term context only to clarify systems, ownership, priorities, and collaborators; it is not evidence that work happened today.
- Treat commits, PRs, reviews, and activity metadata as supporting evidence, and explain the engineering outcome rather than listing repository actions.
- Where the notes support it, state why the work mattered: reliability, performance, developer productivity, customer impact, risk reduction, maintainability, operational efficiency, or team enablement.
- Do not invent impact, overstate exploratory work, or imply that anything was shipped or resolved unless the notes say so.
- Prefer density over verbosity: compress dense notes into the most meaningful themes and keep sparse notes short rather than padded. Every bullet should carry a real outcome, decision, or contribution.
- Write in a professional tone suitable for future performance reviews. Avoid filler such as "worked on" or "made progress", and avoid commit-by-commit narration.

The output should be Markdown.

The resulting output should be a valid Markdown document that can be directly copied and pasted into the daily log file.
Do not include any extraneous text or formatting outside the Markdown document.

Use the following structure:

```markdown
# Daily Log - {formatted_date}

## Summary

3-5 sentence overview of the day's primary engineering themes, outcomes, and notable context.

## Wins

- Resume-style bullets focused on meaningful outcomes, shipped work, resolved issues, risk reduction, productivity improvements, or impactful contributions.
- Do not include routine activity unless it had clear impact.
```

CONTEXT:

work file content:

{work_file_content}

context file content:

{context_file_content}
"#
    )
}

#[must_use]
pub fn get_weekly_log_prompt(
    daily_logs: &[String],
    context_file_content: Option<&str>,
    log_date: &NaiveDate,
) -> String {
    let formatted_date = format_date(log_date);

    let context_file_content = context_file_content.unwrap_or(NO_CONTEXT_GIVEN);

    let combined_daily_logs = daily_logs.join("\n\n--- DAILY LOG ---\n\n");

    format!(
        r#"
You are generating a weekly engineering work log: a concise, specific, and useful summary of a week of engineering work.

You will be given long-term engineer context and a collection of daily engineering logs covering the week, which may overlap and may still contain the original raw notes.

Guidelines:
- Consolidate related work that appears across multiple days into the week's most important engineering themes; a topic that recurs is a significant theme.
- Emphasize outcomes, decisions, debugging, investigations, design work, reviews, operational work, and collaboration rather than daily activity.
- Distinguish completed work from ongoing efforts, investigations, and follow-up items.
- Ignore instructional HTML comments, placeholder text, and empty headings carried over from the default templates; they are guidance for the user, not accomplishments.
- Use long-term context only to clarify systems, ownership, collaborators, and impact areas; it is not evidence that work occurred this week.
- Where the logs support it, state why the work mattered: reliability, performance, developer productivity, customer impact, risk reduction, maintainability, operational efficiency, or team enablement.
- Do not invent accomplishments or shipped outcomes, and do not overstate exploratory or unfinished work.
- Prefer density over verbosity: compress low-signal routine work into broader themes and leave out implementation detail that a higher-level summary covers better.
- Write in a professional tone suitable for future performance reviews. Avoid filler such as "worked on" or "made progress", and avoid day-by-day narration.

The output should be Markdown.

The resulting output should be a valid Markdown document that can be directly copied and pasted into the weekly log file.
Do not include any extraneous text or formatting outside the Markdown document.

Use the following structure:

```markdown
# Weekly Log - {formatted_date}

## Summary

1-2 paragraphs summarizing the week's primary engineering themes, major outcomes, notable investigations, and overall progress.

## Key Outcomes

- Resume-style bullets highlighting meaningful accomplishments, risk reduction, productivity gains, reliability improvements, customer impact, or delivered value.
```

CONTEXT:

weekly daily logs:

{combined_daily_logs}

context file content:

{context_file_content}
"#
    )
}

#[cfg(test)]
#[path = "prompts_tests.rs"]
mod tests;
