# ZenTao CLI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-%3E%3D1.70-blue.svg)](https://www.rust-lang.org/)
[![npm version](https://img.shields.io/npm/v/@imyuyu/zentao-cli.svg)](https://www.npmjs.com/package/@imyuyu/zentao-cli)

ZenTao CLI - 让人类和 AI Agent 都能在终端中操作禅道。覆盖需求、缺陷、任务，产品、项目、用户，用例、执行、发布、构建、文档等核心业务域。

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

## 快速开始（人类用户）

### 配置

```bash
# 设置环境变量
export ZENTAO_URL=https://your-zentao.company.com
export ZENTAO_TOKEN=your-api-token

# 检查认证状态
zentao-cli auth status
```

### 开始使用

```bash
# 需求
zentao-cli story +list --product 1
zentao-cli story +get 123

# 缺陷
zentao-cli bug +list --product 1
zentao-cli bug +get 456

# 产品/项目
zentao-cli product +list
zentao-cli project +list
```

## 快速开始（AI Agent）

### 安装

```bash
npm install -g @imyuyu/zentao-cli
```

### 配置

```bash
export ZENTAO_URL=https://your-zentao.company.com
export ZENTAO_TOKEN=your-api-token
```

### 验证

```bash
zentao-cli auth status
```

## 命令

ZenTao CLI 采用 lark-cli 风格的双层命令系统：

### Domain Commands

标准命令，`+` 前缀提供快捷访问：

```bash
# 需求 (Story)
zentao-cli story +list --product 1
zentao-cli story +get 123
zentao-cli story +create --title "新功能" --product 1 --pri 1
zentao-cli story +update 123 --status closed

# 缺陷 (Bug)
zentao-cli bug +list --product 1
zentao-cli bug +get 456
zentao-cli bug +create --title "页面崩溃" --product 1 --severity 1
zentao-cli bug +update 456 --status resolved --resolution fixed

# 任务 (Task)
zentao-cli task +list --project 1
zentao-cli task +get 123
zentao-cli task +create --name "新任务" --project 1 --pri 1
zentao-cli task +update 123 --status done

# 产品 (Product)
zentao-cli product +list
zentao-cli product +get 1

# 项目 (Project)
zentao-cli project +list
zentao-cli project +get 1

# 用户 (User)
zentao-cli user +list
zentao-cli user +get 1

# 用例 (Testcase)
zentao-cli testcase +list --product 1
zentao-cli testcase +get 123

# 执行 (Execution)
zentao-cli execution +list --project 1
zentao-cli execution +get 100

# 发布 (Release)
zentao-cli release +list
zentao-cli release +get 1

# 构建 (Build)
zentao-cli build +list --product 1
zentao-cli build +get 1

# 文档 (Doc)
zentao-cli doc +list
zentao-cli doc +get 10
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
zentao-cli config unset url
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
