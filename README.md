# zentao-cli

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-%3E%3D1.70-blue.svg)](https://www.rust-lang.org/)
[![npm version](https://img.shields.io/npm/v/@imyuyu/zentao-cli.svg)](https://www.npmjs.com/package/@imyuyu/zentao-cli)

中文版 | [English](./README.en.md)

zentao-cli - 让人类和 AI Agent 都能在终端中操作禅道。覆盖需求、缺陷、任务，产品、项目、用户，用例、执行、发布、构建、文档等核心业务域。

[安装](#安装与快速开始) · [AI Agent Skills](#ai-agent-skills) · [认证](#认证) · [命令](#三层命令系统) · [进阶用法](#进阶用法) · [安全与风险提示](#安全与风险提示使用前必读) · [贡献](#贡献)

## 为什么选择 zentao-cli?

- **Agent-Native Design** — 开箱即用的结构化 Skills，兼容主流 AI 工具，Agent 可零配置操作 禅道
- **广泛覆盖** — 11 个业务域，涵盖需求、缺陷、任务、产品、项目、用户、用例、执行、发布、构建、文档
- **AI 友好** — 每个命令都经过实际 Agent 测试，参数简洁、智能默认值、结构化输出，最大化 Agent 调用成功率
- **开源零门槛** — MIT 许可证，下载即用，`npm install` 即可
- **三步上手** — 一键安装、交互式配置，从安装到首次 API 调用仅需三步
- **安全可控** — Token 认证、配置验证、安全提示
- **三层架构** — 高价值 Shortcuts（工作流/浏览）→ API Commands（主命令面）→ Raw API/Schema 工具，按场景选择合适粒度

## 功能

| 类别         | 能力                                                    |
| ------------ | ------------------------------------------------------- |
| 📝 Story    | 需求管理 - 创建、查询、更新、变更、关闭、删除需求            |
| 🐛 Bug      | Bug 管理 - 创建、查询、更新、解决、确认、关闭、激活、删除 Bug |
| ✅ Task      | 任务管理 - 查看、创建、更新、删除任务                       |
| 📦 Product  | 产品管理 - 查看、创建、更新、删除产品                       |
| 🏗️ Project | 项目管理 - 查看、创建、更新、删除项目                       |
| 👤 User     | 用户管理 - 查看用户信息                                   |
| 🧪 Testcase | 测试用例管理 - 查看、创建、更新、删除、执行用例              |
| ▶️ Execution | 执行管理 - 查看、创建、更新、删除执行                      |
| 🎉 Release  | 发布管理 - 查看发布信息                                   |
| 🔨 Build    | 构建管理 - 查看、创建、更新、删除构建                      |
| 📄 Doc      | 文档管理 - 查看和管理文档                                 |

## 安装与快速开始

### 环境要求

- Node.js `>=16`（`npm`/`npx`）
- Rust `v1.70`+（仅源码构建需要）

### 快速开始（人类用户）

> **AI 助手注意：** 如果你是帮助用户安装的 AI Agent，请直接跳转到[快速开始（AI Agent）](#快速开始ai-agent)，其中包含你需要完成的所有步骤。

#### 安装

选择**其中一种**安装方式：

**方式一 — 从 npm 安装（推荐）：**

```bash
# 安装 CLI
npm install -g @imyuyu/zentao-cli

# 安装 AI Agent Skills（必需）
npx skills add imyuyu/zentao-cli -y -g
```

**方式二 — 从源码安装：**

```bash
git clone https://github.com/imyuyu/zentao-cli.git
cd zentao-cli
cargo build --release
npm install -g .

# 安装 AI Agent Skills（必需）
npx skills add imyuyu/zentao-cli -y -g
```

**方式三 — 从 GitHub Releases 下载：**

访问 [Releases](https://github.com/imyuyu/zentao-cli/releases) 下载对应平台的二进制文件，添加到 PATH 中。

#### 配置与使用

```bash
# 1. 配置应用凭据（一次性，交互式引导）
zentao-cli config init

# 2. 登录
zentao-cli auth login --account admin --password 123456

# 3. 开始使用
zentao-cli story list --product 1
```

## 快速开始（AI Agent）

**第一步 — 安装 CLI 和 Skills**

```bash
npm install -g @imyuyu/zentao-cli
npx skills add imyuyu/zentao-cli -y -g
```

**第二步 — 配置应用凭据**

```bash
# 1. 设置服务器地址
zentao-cli config set url "https://your-zentao.company.com"

# 2. 使用账号密码登录获取 token
zentao-cli auth login --account 你的账号 --password 你的密码

# 3. 设置默认产品和项目（可选）
zentao-cli config set product_id 1
zentao-cli config set project_id 1
```

**第三步 — 验证**

```bash
zentao-cli auth status
```

## AI Agent Skills

| Skill              | Description                                                              |
| ----------------- | ----------------------------------------------------------------------- |
| `zentao-shared`   | 应用配置、认证、环境变量、错误处理、配置优先级、安全规则（所有技能自动加载）   |
| `zentao-story`    | 需求管理 - 列出、查看、创建、更新、变更、关闭、删除需求                      |
| `zentao-bug`      | Bug 管理 - 列出、查看、创建、更新、解决、确认、关闭、激活、删除 Bug           |
| `zentao-task`     | 任务管理 - 列出、查看、创建、更新、删除任务                                |
| `zentao-product`  | 产品管理 - 列出、查看、创建、更新、删除产品                               |
| `zentao-project`  | 项目管理 - 列出、查看、创建、更新、删除项目                               |
| `zentao-user`     | 用户管理 - 列出、查看用户信息                                             |
| `zentao-testcase` | 测试用例 - 列出、查看、创建、更新、删除、执行用例                         |
| `zentao-execution`| 执行管理 - 列出、查看、创建、更新、删除执行                               |
| `zentao-release`  | 发布管理 - 列出、查看发布信息                                             |
| `zentao-build`    | 构建管理 - 列出、查看、创建、更新、删除构建                               |
| `zentao-doc`      | 文档管理 - 列出、查看文档                                               |

## 认证

| 命令           | 说明                                             |
| ------------- | ------------------------------------------------ |
| `auth login`  | 登录命令 - account: 禅道账号, password: 禅道密码    |
| `auth logout` | 登出命令 - 清除保存的 token                       |
| `auth status` | 查看认证状态 - 验证 token 是否有效                 |

```bash
# 登录
zentao-cli auth login --account admin --password 123456

# 查看认证状态
zentao-cli auth status

# 登出
zentao-cli auth logout
```

### 环境变量

| 变量               | 必需 | 说明                                             |
| ------------------ | ---- | ------------------------------------------------|
| `ZENTAO_URL`       | 是   | 禅道服务器地址，例如 `https://zentao.example.com` |
| `ZENTAO_TOKEN`     | 是   | API Token，用于认证                                |
| `ZENTAO_PRODUCT_ID`| 否   | 默认产品 ID                                      |
| `ZENTAO_PROJECT_ID`| 否   | 默认项目 ID                                      |
| `ZENTAO_API_VERSION`| 否   | API 版本 (`v1` 或 `v2`)，默认 `v1`                |

## 三层命令系统

CLI 提供三个层次的粒度，从快捷操作到 API 自省与调试：

### 1. Shortcuts（快捷命令）

只保留高价值工作流或浏览入口，不再和常规 CRUD 命令重复。

```bash
zentao-cli bug-browse --product 1
zentao-cli story-browse --product 1
```

### 2. API Commands

资源 CRUD 和状态变更默认走这层。

```bash
zentao-cli story list --product 1
zentao-cli bug get 123
zentao-cli task update 456 --status done
```

### 3. Raw API / 调试入口

用于连通性测试、schema 自省和原始 API 调用。

```bash
zentao-cli api test
zentao-cli api endpoints
zentao-cli api schema --service story --output json
zentao-cli api GET /api.php/v1/stories --params '{"product":1}'
zentao-cli api POST /api.php/v1/stories --data '{"title":"New Story","product":1}'
```

## 进阶用法

### 输出格式

```bash
--format json      # 完整 JSON 响应
--format pretty    # 格式化输出
--format table     # 表格输出（默认）
--format ndjson    # 换行分隔 JSON（适合管道处理）
--format csv       # 逗号分隔值
--dry-run          # 只显示将执行的操作，不实际调用 API
```

### 日志

```bash
--debug                  # 开启 debug 日志
--log-level info         # 显式指定日志级别：error/warn/info/debug
```

启用日志后，会同时输出到 `stderr` 和系统日志文件。日志按天滚动，文件名模式为 `zentao-cli.log.YYYY-MM-DD`。默认目录：

- Windows: `%LOCALAPPDATA%\\zentao-cli\\logs\\`
- macOS: `~/Library/Logs/zentao-cli/`
- Linux: `$XDG_STATE_HOME/zentao-cli/logs/` 或 `~/.local/state/zentao-cli/logs/`

`--debug` 等价于 `--log-level debug`。未显式开启时，CLI 默认只输出必要错误，不打印调试日志。

### 配置命令

```bash
# 初始化配置（交互式引导）
zentao-cli config init

# 查看当前配置
zentao-cli config show

# 设置配置项
zentao-cli config set url https://your-zentao.company.com
zentao-cli config set token your-api-token

# 获取配置项
zentao-cli config get url

# 取消设置配置项
zentao-cli config unset url
```

### 诊断

```bash
zentao-cli doctor
```

## 安全与风险提示（使用前必读）

本工具可被 AI Agent 调用以自动化操作 禅道，存在模型幻觉、执行不可控、提示词注入等固有风险。授权后，AI Agent 将以您的用户身份在授权范围内执行操作，可能导致敏感数据泄露、越权操作等高风险后果，请您谨慎操作和使用。

为降低上述风险，工具已在多个层面启用默认安全保护。但上述风险仍然存在。我们强烈建议您不要主动修改任何默认安全配置；一旦放开相关限制，上述风险将显著提高，由此产生的后果需由您自行承担。

我们建议您将 AI Agent 作为私人对话助手使用，请勿将其用于公共场景或允许其他用户与其交互，以避免权限被滥用或数据泄露。

请充分知悉全部使用风险。使用本工具即视为您自愿承担相关所有责任。

## 贡献

我们欢迎社区的贡献！如果你发现 Bug 或有功能建议，请随时提交 [Issue](https://github.com/imyuyu/zentao-cli/issues) 或 [Pull Request](https://github.com/imyuyu/zentao-cli/pulls)。

## 许可证

本项目基于 **MIT 许可证**。

