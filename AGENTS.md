# Repository Guidelines

## Project Structure & Module Organization
`src/` contains the Rust CLI implementation. Keep API wrappers in `src/api/`, command entrypoints in `src/cmd/`, shared configuration and output helpers in `src/core/`, shortcut flows in `src/shortcuts/`, and terminal UI code in `src/tui/`. Integration-style tests live in `tests/` with one file per resource area, for example `tests/project_api_test.rs`. Packaged binaries go to `bin/`, release artifacts to `target/` and `release/`, automation helpers to `scripts/`, and reusable agent skills to `skills/`.

## Build, Test, and Development Commands
Use `cargo build` for a debug build and `cargo build --release` for optimized binaries. Run `cargo test --verbose` or `make test` before opening a PR. Use `cargo fmt --all` to format and `cargo clippy -- -D warnings` to enforce lint cleanliness; `make lint` runs both checks. For npm packaging, `npm run build` compiles the release binary and copies it to `bin/zentao.exe`; `npm run clean` removes Cargo build output.

## Coding Style & Naming Conventions
Follow standard Rust formatting with 4-space indentation and `cargo fmt` as the source of truth. Prefer small modules with explicit responsibilities and keep public exports centralized in `src/lib.rs` when needed. Use `snake_case` for files, modules, functions, and tests; use `PascalCase` for structs and enums. Match the existing command naming pattern such as `src/cmd/product.rs` and `src/api/product.rs`.

## Testing Guidelines
Add or update tests in `tests/` whenever a command, API client, or serialization path changes. Name tests descriptively with the `test_*` pattern, for example `test_api_client_creation`. Favor focused integration coverage around public behavior instead of re-testing implementation details. Run `cargo test --verbose` locally and keep clippy warnings at zero before submitting.

## Commit & Pull Request Guidelines
This repository uses Conventional Commit prefixes visible in history: `feat:`, `fix:`, and `docs:` are already in use. Keep commit subjects imperative and concise, for example `fix: handle empty token response`. PRs should describe the user-visible change, list verification steps, and link related issues. Include screenshots or terminal output only when the TUI, CLI UX, or packaging behavior changes.

## Security & Configuration Tips
Do not commit real ZenTao URLs, tokens, or keychain data. Test with local or dummy credentials, and keep environment-specific settings out of tracked files. If you change auth or config flows, verify both CLI behavior and stored configuration handling.
