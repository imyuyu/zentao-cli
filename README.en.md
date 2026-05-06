# zentao-cli

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-%3E%3D1.70-blue.svg)](https://www.rust-lang.org/)
[![npm version](https://img.shields.io/npm/v/@imyuyu/zentao-cli.svg)](https://www.npmjs.com/package/@imyuyu/zentao-cli)

[中文版](./README.md) | English 

zentao-cli - A CLI tool for both humans and AI Agents to operate ZenTao. Covers core business domains including Story, Bug, Task, Product, Project, User, Testcase, Execution, Release, Build, and Doc.

[Install](#installation--quick-start) · [AI Agent Skills](#agent-skills) · [Auth](#authentication) · [Commands](#three-layer-command-system) · [Advanced](#advanced-usage) · [Security](#security--risk-warnings-read-before-use) · [Contributing](#contributing)

## Why zentao-cli?

- **Agent-Native Design** — Out-of-the-box structured Skills, compatible with mainstream AI tools, Agent can operate ZenTao with zero configuration
- **Wide Coverage** — 11 business domains: Story, Bug, Task, Product, Project, User, Testcase, Execution, Release, Build, Doc
- **AI-Friendly** — Every command tested with real Agents, featuring concise parameters, smart defaults, and structured output to maximize Agent call success rates
- **Open Source, Zero Barriers** — MIT license, ready to use, just `npm install`
- **Up and Running in 3 Minutes** — One-click install, interactive configuration, from install to first API call in just 3 steps
- **Secure & Controllable** — Token authentication, configuration validation, security prompts
- **Three-Layer Architecture** — Shortcuts (high-value workflow and browse commands) → API Commands → Raw API (full coverage), choose the right granularity

## Features

| Category      | Capabilities                                              |
| ------------ | --------------------------------------------------------|
| 📝 Story     | Story management - create, query, update, change, close, delete stories |
| 🐛 Bug       | Bug management - create, query, update, resolve, confirm, close, activate, delete bugs |
| ✅ Task      | Task management - view, create, update, delete tasks    |
| 📦 Product   | Product management - view, create, update, delete products |
| 🏗️ Project  | Project management - view, create, update, delete projects |
| 👤 User      | User management - view user information                |
| 🧪 Testcase | Testcase management - view, create, update, delete, execute testcases |
| ▶️ Execution | Execution management - view, create, update, delete executions |
| 🎉 Release   | Release management - view release information          |
| 🔨 Build     | Build management - view, create, update, delete builds |
| 📄 Doc       | Doc management - view and manage documents            |

## Installation & Quick Start

### Requirements

- Node.js `>=16` (`npm`/`npx`)
- Rust `v1.70`+ (required only for source builds)

### Quick Start (Human Users)

> **Note for AI assistants:** If you are an AI Agent helping the user with installation, jump directly to [Quick Start (AI Agent)](#quick-start-ai-agent), which contains all the steps you need to complete.

#### Install

Choose **one** of the following methods:

**Option 1 — From npm (recommended):**

```bash
# Install CLI
npm install -g @imyuyu/zentao-cli

# Install AI Agent Skills (required)
npx skills add imyuyu/zentao-cli -y -g
```

**Option 2 — From source:**

```bash
git clone https://github.com/imyuyu/zentao-cli.git
cd zentao-cli
cargo build --release
npm install -g .

# Install AI Agent Skills (required)
npx skills add imyuyu/zentao-cli -y -g
```

**Option 3 — From GitHub Releases:**

Download binaries for your platform from [Releases](https://github.com/imyuyu/zentao-cli/releases) and add to PATH.

#### Configure & Use

```bash
# 1. Configure app credentials (one-time, interactive guided setup)
zentao-cli config init

# 2. Log in
zentao-cli auth login --account admin --password 123456

# 3. Start using
zentao-cli story list --product 1
```

## Quick Start (AI Agent)

**Step 1 — Install CLI and Skills**

```bash
npm install -g @imyuyu/zentao-cli
npx skills add imyuyu/zentao-cli -y -g
```

**Step 2 — Configure app credentials**

```bash
# 1. Set server URL
zentao-cli config set url "https://your-zentao.company.com"

# 2. Login to get token via account credentials
zentao-cli auth login --account your-account --password your-password

# 3. Set default product and project (optional)
zentao-cli config set product_id 1
zentao-cli config set project_id 1
```

**Step 3 — Verify**

```bash
zentao-cli auth status
```

## Agent Skills

| Skill              | Description                                                              |
| ----------------- | ----------------------------------------------------------------------- |
| `zentao-shared`   | App config, auth, env vars, error handling, config priority, security rules (auto-loaded by all skills) |
| `zentao-story`    | Story management - list, get, create, update, change, close, delete stories |
| `zentao-bug`      | Bug management - list, get, create, update, resolve, confirm, close, activate, delete bugs |
| `zentao-task`     | Task management - list, get, create, update, delete tasks                |
| `zentao-product`  | Product management - list, get, create, update, delete products        |
| `zentao-project`  | Project management - list, get, create, update, delete projects          |
| `zentao-user`     | User management - list, get user information                            |
| `zentao-testcase` | Testcase - list, get, create, update, delete, execute testcases         |
| `zentao-execution`| Execution management - list, get, create, update, delete executions     |
| `zentao-release`  | Release management - list, get release information                       |
| `zentao-build`    | Build management - list, get, create, update, delete builds            |
| `zentao-doc`      | Doc management - list, get documents                                    |

## Authentication

| Command        | Description                                      |
| -------------- | ------------------------------------------------ |
| `auth login`  | Login - account: ZenTao account, password: ZenTao password |
| `auth logout` | Logout - clear saved token                       |
| `auth status` | Show auth status - verify if token is valid      |

```bash
# Login
zentao-cli auth login --account admin --password 123456

# Check auth status
zentao-cli auth status

# Logout
zentao-cli auth logout
```

### Environment Variables

| Variable             | Required | Description                                      |
| ------------------- | -------- | ------------------------------------------------|
| `ZENTAO_URL`       | Yes      | ZenTao server URL, e.g. `https://zentao.example.com` |
| `ZENTAO_TOKEN`     | Yes      | API Token for authentication                     |
| `ZENTAO_PRODUCT_ID`| No       | Default product ID                              |
| `ZENTAO_PROJECT_ID`| No       | Default project ID                              |
| `ZENTAO_API_VERSION`| No      | API version (`v1` or `v2`), default `v1`        |

## Three-Layer Command System

The CLI provides three levels of granularity, covering everything from quick operations to fully custom API calls:

### 1. Shortcuts

Shortcuts keep only high-value workflow and browse entry points. Regular CRUD should use API commands.

```bash
zentao-cli bug-browse --product 1
zentao-cli story-browse --product 1
```

### 2. API Commands

Curated commands mapped to ZenTao API endpoints.

```bash
zentao-cli story list --product 1
zentao-cli bug get 123
zentao-cli task update 456 --status done
```

### 3. Raw API Calls

For connectivity checks, schema inspection, and raw API calls.

```bash
zentao-cli api test
zentao-cli api endpoints
zentao-cli api schema --service story --output json
zentao-cli api GET /api.php/v1/stories --params '{"product":1}'
zentao-cli api POST /api.php/v1/stories --data '{"title":"New Story","product":1}'
```

## Advanced Usage

### Output Formats

```bash
--format json      # Full JSON response
--format pretty    # Human-friendly formatted output
--format table     # Readable table (default)
--format ndjson    # Newline-delimited JSON (for piping)
--format csv       # Comma-separated values
```

### Logging

```bash
--debug                  # Enable debug logging
--log-level info         # Explicit level: error/warn/info/debug
```

When logging is enabled, output is written to both `stderr` and a system log file. Default paths:

Logs rotate daily with the filename pattern `zentao-cli.log.YYYY-MM-DD`. Default directories:

- Windows: `%LOCALAPPDATA%\\zentao-cli\\logs\\`
- macOS: `~/Library/Logs/zentao-cli/`
- Linux: `$XDG_STATE_HOME/zentao-cli/logs/` or `~/.local/state/zentao-cli/logs/`

`--debug` is equivalent to `--log-level debug`. Without either flag, the CLI stays quiet and only prints necessary errors.

### Config Commands

```bash
# Initialize config (interactive guided setup)
zentao-cli config init

# Show current config
zentao-cli config show

# Set config item
zentao-cli config set url https://your-zentao.company.com
zentao-cli config set token your-api-token

# Get config item
zentao-cli config get url

# Unset config item
zentao-cli config unset url
```

### Diagnostic

```bash
zentao-cli doctor
```

## Security & Risk Warnings (Read Before Use)

This tool can be invoked by AI Agents to automate operations on ZenTao, and carries inherent risks such as model hallucinations, unpredictable execution, and prompt injection. After authorization, the AI Agent will act under your user identity within the authorized scope, which may lead to high-risk consequences such as leakage of sensitive data or unauthorized operations. Please use with caution.

To reduce these risks, the tool enables default security protections at multiple layers. However, these risks still exist. We strongly recommend that you do not proactively modify any default security settings; once relevant restrictions are relaxed, the risks will increase significantly, and you will bear the consequences.

We recommend using the AI Agent as a private conversational assistant. Do not use it in public scenarios or allow other users to interact with it, to avoid abuse of permissions or data leakage.

Please fully understand all usage risks. By using this tool, you are deemed to voluntarily assume all related responsibilities.

## Contributing

Community contributions are welcome! If you find a bug or have feature suggestions, please submit an [Issue](https://github.com/imyuyu/zentao-cli/issues) or [Pull Request](https://github.com/imyuyu/zentao-cli/pulls).

## License

This project is licensed under the **MIT License**.

