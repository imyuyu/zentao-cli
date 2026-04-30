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
- **Three-Layer Architecture** — Shortcuts (human & AI friendly) → API Commands → Raw API (full coverage), choose the right granularity

## Features

| Category      | Capabilities                                              |
| ------------ | --------------------------------------------------------|
| 📝 Story     | Story management - create, query, update stories          |
| 🐛 Bug       | Bug management - create, query, update bugs              |
| ✅ Task      | Task management - view and manage tasks                 |
| 📦 Product   | Product management - view and manage products           |
| 🏗️ Project  | Project management - view and manage projects           |
| 👤 User      | User management - view user information                |
| 🧪 Testcase | Testcase management - view testcases                    |
| ▶️ Execution | Execution management - view execution progress          |
| 🎉 Release   | Release management - view release information          |
| 🔨 Build     | Build management - view build information              |
| 📄 Doc       | Doc management - view and manage documents            |

## Installation & Quick Start

### Requirements

- Node.js `>=16` (`npm`/`npx`)
- Rust `v1.70`+ (only required for building from source)

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
cp target/release/zentao-cli.exe bin/zentao.exe
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
zentao-cli story +list --product 1
```

## Quick Start (AI Agent)

**Step 1 — Install CLI and Skills**

```bash
npm install -g @imyuyu/zentao-cli
npx skills add imyuyu/zentao-cli -y -g
```

**Step 2 — Configure app credentials**

```bash
export ZENTAO_URL=https://your-zentao.company.com
export ZENTAO_TOKEN=your-api-token
```

**Step 3 — Verify**

```bash
zentao-cli auth status
```

## Agent Skills

| Skill              | Description                                                              |
| ----------------- | ----------------------------------------------------------------------- |
| `zentao-shared`   | App config, auth, env vars, error handling, config priority, security rules (auto-loaded by all skills) |
| `zentao-story`    | Story management - list, get, create, update stories                     |
| `zentao-bug`      | Bug management - list, get, create, update bugs                         |
| `zentao-task`     | Task management - list, get, create, update tasks                       |
| `zentao-product`  | Product management - list, get product information                      |
| `zentao-project`  | Project management - list, get project information                       |
| `zentao-user`     | User management - list, get user information                            |
| `zentao-testcase` | Testcase - list, get testcases                                         |
| `zentao-execution`| Execution management - list, get execution progress                     |
| `zentao-release`  | Release management - list, get release information                       |
| `zentao-build`    | Build management - list, get build information                          |
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

Prefixed with `+`, designed to be friendly for both humans and AI, with smart defaults, table output, and dry-run previews.

```bash
zentao-cli story +list --product 1
zentao-cli bug +list --product 1
zentao-cli task +list --project 1
```

Run `zentao-cli <service> --help` to see all shortcut commands.

### 2. API Commands

Curated commands mapped to ZenTao API endpoints - covering all business domains.

```bash
zentao-cli story list --product 1
zentao-cli bug list --product 1
```

### 3. Raw API Calls

Call any ZenTao API endpoint directly.

```bash
zentao-cli api test
zentao-cli api endpoints
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

### Pagination

```bash
--page-all                  # Auto-paginate through all pages
--page-limit 5              # Max 5 pages
--page-delay 500            # Delay between page requests (ms)
```

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
