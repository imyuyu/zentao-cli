# ZenTao CLI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-%3E%3D1.70-blue.svg)](https://www.rust-lang.org/)
[![npm version](https://img.shields.io/npm/v/@imyuyu/zentao-cli.svg)](https://www.npmjs.com/package/@imyuyu/zentao-cli)

ZenTao CLI - 让人类和 AI Agent 都能在终端中操作禅道。覆盖需求、缺陷、任务、产品、项目、用户、用例、执行、发布、构建、文档等核心业务域。

## Features

| 类别 | 能力 |
|------|------|
| 需求 (Story) | 查看、创建、更新需求，状态流转 |
| 缺陷 (Bug) | 查看、创建、更新缺陷，缺陷状态与解决方式 |
| 任务 (Task) | 查看、创建、更新任务，任务分配与进度 |
| 产品 (Product) | 查看产品列表和详情 |
| 项目 (Project) | 查看项目列表和详情 |
| 用户 (User) | 查看用户列表和详情 |
| 用例 (Testcase) | 查看和管理测试用例 |
| 执行 (Execution) | 查看执行/迭代进度和详情 |
| 发布 (Release) | 查看产品发布版本 |
| 构建 (Build) | 查看构建版本 |
| 文档 (Doc) | 查看和管理文档库 |

## 安装

### 环境要求

- Node.js `>=16`（`npm`/`npx`）
- Rust `v1.70`+（仅源码构建需要）

### 从 npm 安装（推荐）

```bash
npm install -g @imyuyu/zentao-cli
```

### 从源码安装

```bash
git clone https://github.com/imyuyu/zentao-cli.git
cd zentao-cli
cargo build --release
cp target/release/zentao-cli.exe bin/zentao.exe
npm install -g .
```

## 快速开始

```bash
# 配置环境变量
export ZENTAO_URL=https://your-zentao.company.com
export ZENTAO_TOKEN=your-api-token

# 验证认证
zentao-cli auth status

# 开始使用
zentao-cli story +list --product 1
zentao-cli bug +list --product 1
```

## 命令

ZenTao CLI 采用 lark-cli 风格的双层命令系统：

### Domain Commands

标准命令，`+` 前缀提供快捷访问：

```bash
# 需求
zentao-cli story +list --product 1
zentao-cli story +get 123
zentao-cli story +create --title "新功能" --product 1 --pri 1
zentao-cli story +update 123 --status closed

# 缺陷
zentao-cli bug +list --product 1
zentao-cli bug +get 456
zentao-cli bug +create --title "页面崩溃" --product 1 --severity 1
zentao-cli bug +update 456 --status resolved --resolution fixed

# 任务
zentao-cli task +list --project 1
zentao-cli task +get 123
zentao-cli task +create --name "新任务" --project 1 --pri 1
zentao-cli task +update 123 --status done

# 产品/项目/用户/用例/执行/发布/构建/文档
zentao-cli product +list
zentao-cli project +list
zentao-cli user +list
zentao-cli testcase +list --product 1
zentao-cli execution +list --project 1
zentao-cli release +list
zentao-cli build +list --product 1
zentao-cli doc +list
```

### Auth & Config

```bash
# 认证
zentao-cli auth login --account admin --password 123456
zentao-cli auth status
zentao-cli auth logout

# 配置
zentao-cli config init
zentao-cli config show
zentao-cli config set url https://...
zentao-cli config get url
```

### API 调试

```bash
zentao-cli api test
zentao-cli api endpoints
```

### 诊断

```bash
zentao-cli doctor
```

## 输出格式

所有命令支持 `--format` 参数：

| 格式 | 说明 |
|-----|------|
| `table` | 表格（默认） |
| `json` | 完整 JSON 数组 |
| `pretty` | 格式化 JSON |
| `ndjson` | 每行一个 JSON 对象 |
| `csv` | 逗号分隔值 |

```bash
zentao-cli story +list --product 1 --format json
```

## 环境变量

| 变量 | 必需 | 说明 |
|------|------|------|
| `ZENTAO_URL` | 是 | ZenTao 服务器地址 |
| `ZENTAO_TOKEN` | 是 | API Token |
| `ZENTAO_PRODUCT_ID` | 否 | 默认产品 ID |
| `ZENTAO_PROJECT_ID` | 否 | 默认项目 ID |
| `ZENTAO_API_VERSION` | 否 | API 版本 (`v1` 或 `v2`)，默认 `v1` |

## 编译

```bash
cargo check
cargo build
cargo build --release
cargo test
cargo fmt && cargo clippy -- -D warnings
```

## 许可证

MIT
