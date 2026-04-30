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
story +change 123 --status active    # 变更需求
story +delete 123                        # 删除需求
story +close 123                        # 关闭需求

bug +list --product 1
bug +get 456
bug +create --title "..." --product 1 --severity 1
bug +update 456 --status resolved
bug +resolve 456 --resolution fixed --resolved-build trunk  # 解决 Bug
bug +confirm 456       # 确认 Bug
bug +close 456         # 关闭 Bug
bug +activate 456      # 激活 Bug
bug +delete 456        # 删除 Bug

task +list --project 1
task +get 123
task +create --name "..." --project 1 --pri 1
task +update 123 --status done
task +delete 123       # 删除任务

product +list
product +get 1
product +create --name "..." --code myproduct  # 创建产品
product +update 1 --status closed  # 更新产品
product +delete 1       # 删除产品

project +list
project +get 1
project +create --name "..." --code myproject  # 创建项目
project +update 1 --status closed  # 更新项目
project +delete 1       # 删除项目

user +list
user +get 1
testcase +list --product 1
testcase +get 123
testcase +create --product 1 --title "..."  # 创建用例
testcase +update 123 --status normal  # 更新用例
testcase +delete 123     # 删除用例
testcase +result 123 --result pass    # 执行用例

execution +list --project 1
execution +get 100
execution +create --name "Sprint 1" --project 1  # 创建执行
execution +update 100 --status closed  # 更新执行
execution +delete 100     # 删除执行

release +list
release +get 1
build +list --product 1
build +get 1
build +create --name "v1.0.0" --project 1 --product 1  # 创建版本
build +update 1 --ci "Jenkins #1"  # 更新版本
build +delete 1           # 删除版本

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
