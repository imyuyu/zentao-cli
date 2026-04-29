# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release binary: target/release/zentao-cli.exe
cargo check              # Fast compile check
cargo test               # Run tests
cargo run -- story +list --product 1   # Run with args
cargo fmt                # Format code
cargo clippy -- -D warnings  # Lint (warnings as errors)
```

## Architecture

```
src/
├── main.rs              # Entry point, calls zentao_cli::run()
├── lib.rs               # Exports public API (Config, ApiClient, types)
├── cmd/                 # CLI command handlers
│   ├── root.rs          # Clap CLI structure, config loading, command dispatch
│   ├── story.rs/bug.rs/task.rs/product.rs/project.rs  # CRUD commands
│   ├── auth.rs          # Token-based auth commands
│   ├── config_cmd.rs    # Config management
│   ├── api_cmd.rs       # API testing
│   ├── doctor.rs        # Diagnostic command
│   └── browse.rs        # TUI browse mode
├── api/                 # HTTP client layer
│   ├── client.rs        # Reqwest HTTP client
│   ├── auth.rs          # Auth types
│   ├── types.rs         # Shared types (Story, Bug, etc.)
│   └── */               # Domain APIs (story, bug, task, product, project)
├── core/                # Core modules
│   ├── config.rs        # Config loading (env → global → project)
│   ├── error.rs         # Error types
│   └── output.rs        # Output formatting
├── shortcuts/           # AI-friendly shortcut commands
└── tui/                 # Terminal UI (ratatui-based)
```

## Command Style

All subcommands use `+` prefix as the primary name (lark-cli style):

```bash
story +list              # list stories
story +get 123           # get story detail
story +create --title "..." --product 1 --pri 1
story +update 123 --status closed

bug +list --product 1
bug +get 456
bug +create --title "..." --product 1 --severity 1
bug +update 456 --status resolved

task +list --project 1
task +get 123
task +create --name "..." --project 1 --pri 1
task +update 123 --status done

product +list
product +get 1
project +list
project +get 1
user +list
user +get 1
testcase +list --product 1
testcase +get 123
execution +list --project 1
execution +get 100
release +list
release +get 1
build +list --product 1
build +get 1
doc +list
doc +get 10

auth login --account admin --password 123456
auth status
auth logout

config init
config show
config set url https://...
config get url
config unset url

api test
api endpoints

doctor              # 诊断配置和网络
story-browse --product 1  # TUI 浏览需求
bug-browse --product 1    # TUI 浏览缺陷
```

## Config Precedence (highest first)

1. Environment variables: `ZENTAO_URL`, `ZENTAO_TOKEN`, `ZENTAO_PRODUCT_ID`, `ZENTAO_PROJECT_ID`, `ZENTAO_API_VERSION`
2. Project config: `.zentao-cli/config.toml`
3. Global config: `~/.zentao-cli/config.toml` (Windows: `%APPDATA%/zentao-cli`)

## Key Patterns

- **CLI**: Clap with derive macros; global `--format` flag (json/pretty/table/ndjson/csv)
- **Errors**: anyhow::Result at app layer, thiserror for typed errors in libraries
- **Async**: Tokio runtime created per-command for sync commands (auth, api, doctor)
- **API**: Reqwest with rustls-tls, token auth via ZENTAO_TOKEN
- **Shortcuts**: Separate `shortcuts` command layer with pagination and format options

## Skills

Claude Code skills in `skills/` provide AI-friendly command references. Read `skills/zentao-shared/SKILL.md` first as other skills depend on it.
