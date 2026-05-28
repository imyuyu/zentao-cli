# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release binary: target/release/zentao-cli.exe
cargo check              # Fast compile check (no codegen)
cargo fmt                # Format code
cargo clippy -- -D warnings  # Lint (warnings as errors)
```

## Test Commands

```bash
cargo test                          # Run all tests
cargo test -- --nocapture           # Show println output
cargo test <filter>                 # Run matching tests (e.g. `cargo test bug`)
cargo test --test story_api_test    # Run a specific integration test file
cargo test --lib                    # Unit tests only
cargo llvm-cov                      # Coverage summary (install: cargo install cargo-llvm-cov)
cargo llvm-cov --fail-under-lines 80  # CI coverage gate
```

## Setup

```bash
# Interactive config wizard
zentao config set

# Or set env vars directly
export ZENTAO_URL="https://your-zentao.example.com"
export ZENTAO_TOKEN="your-api-token"

# Verify config
zentao doctor
```

Credentials are stored via OS keyring (`keyring` crate). Fallback: `~/.zentao-cli/credentials`.

## Workflow Notes

- **Always run `cargo fmt` before commit** - CI checks formatting, failures block merge.
- Commit directly after formatting; let CI verify.

## Architecture

Layered design, top to bottom:

```
src/
├── main.rs              # Entry point, calls zentao_cli::run()
├── lib.rs               # Crate root, re-exports public API
├── cmd/                 # CLI command handlers (clap derive) — one file per domain
│   ├── root.rs          # CLI structure, config loading, command dispatch
│   └── *.rs             # Domain commands: story, bug, task, product, project, ...
├── service/             # Business logic layer (async), one file per domain
├── api/                 # HTTP client layer (reqwest + rustls-tls, token auth)
│   ├── client.rs        # Base HTTP client
│   ├── auth_client.rs   # Token/session management
│   └── *.rs             # Domain APIs: story, bug, task, product, ...
├── core/                # Config, credentials, error, output, logging, runtime
├── tui/                 # Terminal UI (ratatui + crossterm)
│   ├── app.rs           # App state and event loop
│   ├── browser.rs       # Browse mode (list entities interactively)
│   ├── wizard.rs        # Multi-step creation wizards
│   └── pages/           # One page per domain (story, bug, task, ...)
└── tests/               # Integration tests (one file per domain + common/)
```

Three command layers:
- **Shortcuts**: `bug-browse`, `story-browse` → TUI browser
- **CRUD Commands**: `story list`, `bug get`, `task create`, etc.
- **Raw API**: `api test`, `api GET /path`

## TUI Keybindings

| Key | Context | Action |
|-----|---------|--------|
| `↑` `↓` / `j` `k` | Lists | Navigate items |
| `Enter` | Lists | Open detail / select |
| `Esc` / `q` | Detail | Back to list |
| `Esc` / `q` | List | Back to main menu |
| `Esc` / `q` | Main menu | Quit (with confirm) |
| `p` | Main menu | Switch product |
| `?` | Any | Toggle help overlay |
| `Ctrl+F` | Lists | Search/filter |
| `o` | Detail | Open in browser |
| `r` / `F5` | Lists | Reload data |
| `c` | Bug list | Create bug |
| `e` | Bug detail | Edit bug |

## Config Precedence (highest first)

1. CLI args: `--url`, `--token`
2. Env vars: `ZENTAO_URL`, `ZENTAO_TOKEN`, `ZENTAO_PRODUCT_ID`, etc.
3. Project config: `.zentao-cli/config.toml`
4. Global config: `~/.zentao-cli/config.toml`

Multi-account support via `MultiAccountConfig` — stores per-account credentials and config, selected at startup.

## Key Patterns

- **CLI**: Clap derive; global `--format` flag (`json` | `pretty` | `table` | `ndjson` | `csv`)
- **Errors**: `anyhow::Result` at app layer, `thiserror` for typed errors in libs
- **Async**: Tokio runtime, one per command invocation for sync commands
- **API**: Reqwest + rustls-tls, token-based auth via `X-Zentao-Token` header
- **Auth**: Credentials stored in OS keyring (`keyring` crate). `auth_client.rs` handles token lifecycle. Use `rpassword` for interactive password prompts.
- **Service layer**: `src/service/` — business logic, depends on API layer. Each domain has its own service file.
- **TUI**: Ratatui immediate-mode rendering. Pages implement a common trait. Wizard flow for multi-step creation.

## Skills

Claude Code skills in `skills/` provide AI-friendly command references. Read `skills/zentao-shared/SKILL.md` first as other skills depend on it.
