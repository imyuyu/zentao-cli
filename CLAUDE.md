# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release binary: target/release/zentao-cli.exe
cargo check              # Fast compile check
cargo test               # Run tests
cargo fmt                # Format code (CI runs fmt, don't run before commit)
cargo clippy -- -D warnings  # Lint (warnings as errors)
```

## Workflow Notes

- **Always run `cargo fmt` before commit** - CI checks formatting, failures block merge.
- Commit directly after formatting; let CI verify.

## Architecture

```
src/
├── main.rs              # Entry point, calls zentao_cli::run()
├── lib.rs               # Exports public API
├── cmd/                 # CLI command handlers (clap derive)
│   ├── root.rs          # CLI structure, config loading, command dispatch
│   ├── story.rs/bug.rs/task.rs/...  # Domain commands
│   └── browse.rs        # TUI browse mode
├── api/                 # HTTP client layer
│   ├── client.rs        # Reqwest HTTP client
│   └── */               # Domain APIs (story, bug, task, ...)
├── service/             # Business logic layer (async)
├── core/                # Config, error, output, logging
└── tui/                 # Terminal UI (ratatui)
```

三层命令：
- **Shortcuts**: `bug-browse`, `story-browse` (TUI)
- **API Commands**: `story list`, `bug get` 等 CRUD
- **Raw API**: `api test`, `api GET /path`

## Config Precedence (highest first)

1. CLI args: `--url`, `--token`
2. Env vars: `ZENTAO_URL`, `ZENTAO_TOKEN`, `ZENTAO_PRODUCT_ID`, etc.
3. Project config: `.zentao-cli/config.toml`
4. Global config: `~/.zentao-cli/config.toml`

## Key Patterns

- **CLI**: Clap derive; global `--format` flag (json/pretty/table/ndjson/csv)
- **Errors**: `anyhow::Result` at app layer, `thiserror` for typed errors in libs
- **Async**: Tokio runtime per-command for sync commands
- **API**: Reqwest + rustls-tls, token auth
- **Service layer**: `src/service/` - business logic, depends on API layer

## Skills

Claude Code skills in `skills/` provide AI-friendly command references. Read `skills/zentao-shared/SKILL.md` first as other skills depend on it.
