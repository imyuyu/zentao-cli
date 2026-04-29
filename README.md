# ZenTao CLI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-%3E%3D1.70-blue.svg)](https://www.rust-lang.org/)
[![npm version](https://img.shields.io/npm/v/@imyuyu/zentao-cli.svg)](https://www.npmjs.com/package/@imyuyu/zentao-cli)

ZenTao CLI 工具 — 让人类和 AI Agent 都能在终端中操作禅道。覆盖需求、缺陷、任务、产品、项目、用户、用例、执行、发布、构建、文档等核心业务域。

[安装](#安装与快速开始) · [三层命令](#三层命令系统) · [AI Agent Skills](#agent-skills) · [认证](#认证) · [命令](#命令) · [进阶用法](#进阶用法) · [贡献](#贡献)

## 为什么选 ZenTao CLI？

- **为 Agent 原生设计** — 12 个 [Skills](#agent-skills) 开箱即用，适配主流 AI 工具，Agent 无需额外适配即可操作禅道
- **覆盖面广** — 11 大业务域、50+ 精选命令、12 个 AI Agent [Skills](#agent-skills)
- **AI 友好调优** — 每条命令经过 Agent 实测验证，提供更友好的参数、智能默认值和结构化输出
- **开源零门槛** — MIT 协议，开箱即用，`npm install` 即可使用
- **安全可控** — Token 认证、环境变量配置、OS 原生密钥链存储凭证
- **进阶格式支持** — 多输出格式（JSON/CSV/NDJSON/Table）、分页控制、快捷别名

## 功能

| 类别        | 能力                                         |
|-------------|--------------------------------------------|
| 需求 (Story) | 查看、创建、更新需求，状态流转                      |
| 缺陷 (Bug)  | 查看、创建、更新缺陷，缺陷状态与解决方式                |
| 任务 (Task) | 查看、创建、更新任务，任务分配与进度                   |
| 产品 (Product) | 查看产品列表和详情                             |
| 项目 (Project) | 查看项目列表和详情                            |
| 用户 (User) | 查看用户列表和详情                             |
| 用例 (Testcase) | 查看和管理测试用例                           |
| 执行 (Execution) | 查看执行/迭代进度和详情                      |
| 发布 (Release) | 查看产品发布版本                            |
| 构建 (Build) | 查看构建版本                                |
| 文档 (Doc)  | 查看和管理文档库                              |

## 安装与快速开始

### 环境要求

- Node.js（`npm`/`npx`）
- Rust `v1.70`+（仅源码构建需要）

### 快速开始（人类用户）

#### 安装

以下两种方式**任选其一**：

**方式一 — 从 npm 安装（推荐）：**

```bash
# 安装 CLI
npm install -g @imyuyu/zentao-cli

# 安装 CLI SKILL（必需）
npx skills add @imyuyu/zentao-cli -y -g
```

**方式二 — 从源码安装：**

需要 Rust `v1.70`+。

```bash
git clone https://github.com/zentao-cli/cli.git
cd cli
cargo install --path .

# 安装 CLI SKILL（必需）
npx skills add @imyuyu/zentao-cli -y -g
```

#### 配置与使用

```bash
# 1. 配置环境变量
export ZENTAO_URL=https://your-zentao.company.com
export ZENTAO_TOKEN=your-api-token

# 2. 检查认证状态
zentao-cli auth status

# 3. 开始使用
zentao-cli story list --product 1
```

### 快速开始（AI Agent）

**第 1 步 — 安装**

```bash
# 安装 CLI
npm install -g @imyuyu/zentao-cli

# 安装 CLI SKILL（必需）
npx skills add @imyuyu/zentao-cli -y -g
```

**第 2 步 — 配置**

```bash
# 设置环境变量
export ZENTAO_URL=https://your-zentao.company.com
export ZENTAO_TOKEN=your-api-token
```

**第 3 步 — 验证**

```bash
zentao-cli auth status
```


## Agent Skills

| Skill                      | 说明                                    |
| -------------------------- | --------------------------------------- |
| `zentao-shared`          | 配置初始化、认证、Token、环境变量、安全规则（所有其他 skill 自动加载） |
| `zentao-story`           | 需求列表、详情、创建、更新、状态流转                  |
| `zentao-bug`             | 缺陷列表、详情、创建、更新、状态与解决方式              |
| `zentao-task`            | 任务列表、详情、创建、更新、分配                    |
| `zentao-product`         | 产品列表、详情                            |
| `zentao-project`         | 项目列表、详情                            |
| `zentao-user`            | 用户列表、详情                            |
| `zentao-testcase`        | 用例列表、详情                            |
| `zentao-execution`        | 执行/迭代列表、详情                        |
| `zentao-release`          | 发布列表、详情                            |
| `zentao-build`            | 构建列表、详情                            |
| `zentao-doc`              | 文档列表、详情                            |

## 认证

### 环境变量

| 变量                | 必需 | 说明                                      |
| ------------------- | ---- | ---------------------------------------- |
| `ZENTAO_URL`        | 是   | ZenTao 服务器地址，例如 `https://zentao.example.com` |
| `ZENTAO_TOKEN`      | 是   | API Token，用于认证                        |
| `ZENTAO_PRODUCT_ID`  | 否   | 默认产品 ID                               |
| `ZENTAO_PROJECT_ID`  | 否   | 默认项目 ID                               |
| `ZENTAO_API_VERSION` | 否   | API 版本 (`v1` 或 `v2`)，默认 `v1`         |

### 命令

```bash
# 检查认证状态
zentao-cli auth status

# 登录 (需要账号密码)
zentao-cli auth login --account admin --password 123456
```

## 三层命令系统

ZenTao CLI 提供三层命令，从快捷到原始，满足不同场景需求：

### 第 1 层：Shortcuts（快捷命令）

最简洁的快速访问方式，适合 AI Agent 和日常快速操作。支持多种输出格式和分页：

```bash
# 基础用法
zentao-cli shortcuts products                    # 获取产品列表
zentao-cli shortcuts projects                   # 获取项目列表
zentao-cli shortcuts bugs --product 1         # 获取产品的 Bug 列表
zentao-cli shortcuts stories --product 1       # 获取产品的故事列表
zentao-cli shortcuts tasks --project 1         # 获取项目的任务列表

# 支持 + 别名
zentao-cli shortcuts +products
zentao-cli shortcuts +bugs --product 1

# 多种输出格式
zentao-cli shortcuts products --format json
zentao-cli shortcuts products --format csv
zentao-cli shortcuts products --format ndjson

# 分页支持
zentao-cli shortcuts stories --product 1 --page-all --page-delay 200
```

### 第 2 层：API Commands（API 命令）

标准命令方式，提供更完整的选项和参数：

```bash
# 需求
zentao-cli story list --product 1 --status active
zentao-cli story get 123

# 缺陷
zentao-cli bug list --product 1 --status active
zentao-cli bug get 456

# 产品/项目/任务
zentao-cli products list
zentao-cli projects list
zentao-cli tasks list --project 1
```

### 第 3 层：Raw API（原始 API 调用）

直接调试和探索 ZenTao API：

```bash
zentao-cli api test       # 测试 API 连接
zentao-cli api endpoints  # 列出所有可用端点
zentao-cli api list       # 交互式选择并调用端点
```

## 命令

### 需求 (Story)

```bash
# 列出需求
zentao-cli story list --product 1
zentao-cli story list --product 1 --status active

# 查看需求详情
zentao-cli story get 123

# 创建需求
zentao-cli story create --title "新功能" --product 1 --pri 1

# 更新需求
zentao-cli story update 123 --status closed
```

### 缺陷 (Bug)

```bash
# 列出缺陷
zentao-cli bug list --product 1
zentao-cli bug list --product 1 --status active

# 查看缺陷详情
zentao-cli bug get 123

# 创建缺陷
zentao-cli bug create --title "页面崩溃" --product 1 --severity 1

# 更新缺陷
zentao-cli bug update 123 --status resolved --resolution fixed
```

### 任务 (Task)

```bash
# 列出任务
zentao-cli task list --project 1

# 查看任务详情
zentao-cli task get 123

# 创建任务
zentao-cli task create --name "新任务" --project 1 --pri 1

# 更新任务
zentao-cli task update 123 --status done
```

### 产品 (Product)

```bash
# 列出产品
zentao-cli product list

# 查看产品详情
zentao-cli product get 1
```

### 项目 (Project)

```bash
# 列出项目
zentao-cli project list

# 查看项目详情
zentao-cli project get 1
```

### 用户 (User)

```bash
# 列出用户
zentao-cli user list

# 查看用户详情
zentao-cli user get 1
```

### 用例 (Testcase)

```bash
# 列出用例
zentao-cli testcase list --product 1

# 查看用例详情
zentao-cli testcase get 123
```

### 执行 (Execution)

```bash
# 列出执行
zentao-cli execution list --project 1

# 查看执行详情
zentao-cli execution get 100
```

### 发布 (Release)

```bash
# 列出发布
zentao-cli release list

# 查看发布详情
zentao-cli release get 1
```

### 构建 (Build)

```bash
# 列出构建
zentao-cli build list --product 1

# 查看构建详情
zentao-cli build get 1
```

### 文档 (Doc)

```bash
# 列出文档
zentao-cli doc list

# 查看文档详情
zentao-cli doc get 10
```


## 进阶用法

### 输出格式

所有命令支持多种输出格式，通过 `--format` 参数指定：

```bash
--format json      # 完整 JSON 数组
--format pretty    # 格式化 JSON（美化输出）
--format table    # 制表符分隔表格（默认）
--format ndjson   # 每行一个 JSON 对象（适合管道处理）
--format csv      # 逗号分隔值
```

**示例：**

```bash
# CSV 格式（适合 Excel 导入）
zentao shortcuts products --format csv

# NDJSON 格式（适合管道处理）
zentao shortcuts stories --product 1 --format ndjson

# 表格格式（默认，适合人类阅读）
zentao shortcuts bugs --product 1 --format table
```

### 分页支持

快捷命令支持分页参数：

```bash
--page-all         # 获取所有数据（不分页）
--page-limit <N>  # 每页数量（默认 100，最大 500）
--page-delay <N>  # 分页请求间隔毫秒数（默认 100）
```

**示例：**

```bash
# 获取所有 stories（带请求延迟，避免频率限制）
zentao shortcuts stories --product 1 --page-all --page-delay 200

# 自定义每页数量
zentao shortcuts stories --product 1 --page-limit 50
```

### 快捷别名

Shortcuts 层支持带 `+` 前缀的快捷别名：

```bash
zentao shortcuts +products    # 等同于 shortcuts products
zentao shortcuts +projects   # 等同于 shortcuts projects
zentao shortcuts +bugs       # 等同于 shortcuts bugs
zentao shortcuts +stories     # 等同于 shortcuts stories
zentao shortcuts +tasks      # 等同于 shortcuts tasks
```

### 配置优先级

CLI 参数优先级从高到低：

1. **命令行参数**（`--product`, `--project` 等）
2. **环境变量**（`ZENTAO_PRODUCT_ID`, `ZENTAO_PROJECT_ID`）
3. **默认配置**

### 配置命令

```bash
# 显示配置
zentao-cli config show

# 获取特定值
zentao-cli config get url
```

### 诊断

```bash
# 检查配置和连接
zentao-cli doctor
```

### API 测试

```bash
# 测试 API 连接
zentao-cli api test

# 查看可用端点
zentao-cli api endpoints
```

### V2 API 支持

ZenTao V2 API 使用不同的认证 Header：

```bash
# 设置 API 版本
zentao-cli config set api_version v2

# 或使用环境变量
export ZENTAO_API_VERSION=v2
```

| API 版本 | Header 名称 |
|---------|------------|
| V1      | `Token: xxx` |
| V2      | `token: xxx` |


## 项目结构

```
zentao-cli/
├── src/
│   ├── main.rs              # 入口
│   ├── lib.rs               # 库入口
│   ├── cmd/                 # CLI 命令
│   ├── api/                 # API 客户端
│   ├── core/                # 核心模块
│   ├── shortcuts/           # Shortcuts 层
│   └── tui/                # TUI 组件
├── skills/                  # Claude Code Skills
├── bin/                    # 预编译二进制
├── package.json            # npm 包配置
├── Cargo.toml
└── README.md
```


## 编译

```bash
# 检查编译
cargo check

# Debug 构建
cargo build

# Release 构建
cargo build --release

# 运行
cargo run -- story list --product 1

# 测试
cargo test
```


## 贡献

欢迎社区贡献！如果你发现 bug 或有功能建议，请提交 [Issue](https://github.com/zentao-cli/cli/issues) 或 [Pull Request](https://github.com/zentao-cli/cli/pulls)。


## 许可证

本项目基于 **MIT 许可证** 开源。
