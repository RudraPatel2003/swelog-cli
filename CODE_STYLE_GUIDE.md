# Code Style Guide

## General

- Use Rust 2024 conventions and the repository `rustfmt.toml`.
- Spell names out: `directory`, not `dir`.
- Put a blank line between unrelated lines of code.
- Keep files small and focused: give a self-contained feature its own sub-module, and extract logic so the core flow stays readable.
- Never use `mod.rs`. A module with submodules is `issues.rs` next to an `issues/` folder.
- A crate's `lib.rs` holds only module declarations, in one alphabetical block. Callers write `use updates::check::print_update_notice;`.
- Import the full path and call unqualified: `use crate::utils::read_npm_package_json;` then `read_npm_package_json()`.
- Never `pub use` to shorten an import path. The path should name the module that defines the item.
- No naked `bool` parameters, because `write_default_config(&path, &config, true)` does not say what `true` means. Use an enum such as `Overwrite::Yes`, and model mutually exclusive flags as one enum, the way `DateSelection` covers `--date` and `--yesterday`. clap fields stay `bool`; convert once in a constructor such as `Overwrite::from_force_flag`.
- Keep comments out of function bodies; when a block needs one, extract a named function. Comment only what the code cannot say: how an external system behaves, why a workaround exists, a constraint with no local evidence.
- Omit doc comments most of the time. If possible, just name the function to be more descriptive.

## Error Handling

- Typed errors with `thiserror`, user-facing diagnostics with `miette`.
- Return `miette::Result<T>`, never a spelled-out error type.

## API Calls

- If making external API calls, do the call in the module file, such as `issues.rs`, and hold types in `issues/structs.rs`.

## CLI

- The CLI should read arguments and call the corresponding `run` function. If the `run` function has no flags, avoid an unused self warning by using `let _ = self;`.

## Testing

- Place a module's unit tests in a sibling file named after it, such as `issues_tests.rs` next to `issues.rs`, declared with `#[cfg(test)] #[path = "issues_tests.rs"] mod tests;`.
- Place crate integration tests in the `tests/` folder of the crate, at the same level as the `src/` folder.
- Name tests by behavior, for example `write_config_fails_when_file_exists_without_force`.

## Documentation

- Documentation pages use the `.mdx` extension, never `.md`.
- User-facing documentation belongs in `docs/`, not in `README.md`. The README
  covers installation, contributing, and links to the docs site.
- `README.md` and `npm/README.md` are kept in sync by hand. Update both.
