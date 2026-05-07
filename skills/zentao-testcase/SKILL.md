---
name: zentao-testcase
version: 1.2.0
description: "禅道(ZenTao) 测试用例（Testcase）管理 — 列出用例、查看用例详情、按产品/项目查询用例、执行用例。当用户说：'查询用例'、'用例列表'、'有哪些用例'、'testcase 列表'、'查看用例'、'用例详情'、'创建用例'、'新建 testcase'、'执行用例'、'用例结果'、'测试用例'、'禅道用例' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
    envs: ["ZENTAO_URL", "ZENTAO_TOKEN"]
  cliHelp: "zentao-cli testcase --help"
---

# Testcase (v2)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量和错误处理。**

> **测试用例类型**：ZenTao 测试用例有多种类型，包括功能测试、性能测试、接口测试、安全测试、并发测试、破坏性测试、安装测试等。查询时可按类型筛选。
>
> **状态判断**：测试用例的状态包括 wait（等待）、normal（正常）、blocked（阻塞）、bypass（跳过）。其中 normal 表示用例执行通过。
>
> **严重性级别**：测试用例的 severity 分为 1-4 级，1 为最严重（系统崩溃），4 为最低。
>
> **产品与项目**：测试用例可以关联产品（product）或项目（project），也可以同时关联两者。查询时可以只指定产品或同时指定产品和项目。
>
> **友好输出**：在输出测试用例详情时，建议同时输出用例的 URL 链接，便于用户直接点击查看。

## Testcase Status

| Status | Description | 说明 |
|--------|-------------|------|
| wait | Waiting | 等待执行 |
| normal | Normal/Pass | 正常/通过 |
| blocked | Blocked | 阻塞 |
| bypass | Bypass/Skip | 跳过 |

## Testcase Type

| Type | Description | 说明 |
|------|-------------|------|
| feature | Feature Test | 功能测试 |
| performance | Performance Test | 性能测试 |
| interface | Interface Test | 接口测试 |
| security | Security Test | 安全测试 |
| concurrency | Concurrency Test | 并发测试 |
| destructive | Destructive Test | 破坏性测试 |
| install | Installation Test | 安装测试 |
| others | Others | 其他 |

## Testcase Severity Levels

| Level | Name | Description |
|-------|------|-------------|
| 1 | Critical | 系统崩溃或核心功能完全不可用 |
| 2 | Major | 主要功能失效 |
| 3 | Normal | 普通测试用例 |
| 4 | Minor | 轻微问题/界面测试 |

## Commands

- [`testcase list`](./references/zentao-testcase-list.md) — List test cases for a product/project
- [`testcase get`](./references/zentao-testcase-get.md) — Get test case details
- [`testcase create`](./references/zentao-testcase-create.md) — Create a test case
- [`testcase update`](./references/zentao-testcase-update.md) — Update a test case
- [`testcase delete`](./references/zentao-testcase-delete.md) — Delete a test case
- [`testcase result`](./references/zentao-testcase-result.md) — Record test case execution result

## Common Use Cases

### 场景 1：查询产品的所有测试用例

```bash
# 列出产品 1 下的所有测试用例
zentao-cli testcase list --product 1

# 列出产品 1 下状态为正常的测试用例
zentao-cli testcase list --product 1 --status normal
```

### 场景 2：按项目查询测试用例

```bash
# 列出产品 1 下项目 5 的所有测试用例
zentao-cli testcase list --product 1 --project 5

# 列出项目 5 的所有功能测试用例
zentao-cli testcase list --product 1 --project 5 --type feature
```

### 场景 3：获取测试用例详情

```bash
# 获取测试用例详细信息
zentao-cli testcase get 123
```

## Examples

```bash
# 列出产品 1 下的所有测试用例
zentao-cli testcase list --product 1

# 列出状态为正常的测试用例
zentao-cli testcase list --product 1 --status normal

# 获取测试用例详情
zentao-cli testcase get 123

# 创建测试用例
zentao-cli testcase create --title "登录功能测试" --product 1 --type feature
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 测试用例 ID |
| title | string | 测试用例标题 |
| product | u64 | 产品 ID |
| project | u64 | 项目 ID（可选） |
| type | string | 用例类型 |
| status | string | 用例状态 |
| severity | u8 | 严重程度（1-4） |
| pri | u8 | 优先级（0-5） |
| openedBy | string | 创建人 |
| version | u64 | 版本号 |
| precondition | string | 前置条件（可选） |
| steps | string | 测试步骤（可选） |
| expected | string | 预期结果（可选） |

## Error Handling

### 常见错误

| 错误码 | 说明 | 解决方案 |
|--------|------|----------|
| `ZEN_AUTH_FAILED` | Token 无效或过期 | 检查 ZENTAO_TOKEN 配置 |
| `ZEN_NOT_FOUND` | 测试用例不存在 | 检查用例 ID 是否正确 |
| `ZEN_CONFIG_INVALID` | 配置无效 | 验证 ZENTAO_URL 和 ZENTAO_TOKEN |
| `ZEN_API_ERROR` | ZenTao API 返回错误 | 检查服务器状态或稍后重试 |
| `ZEN_PARAM_MISSING` | 缺少必需参数 | 确保提供产品 ID |

### 错误处理示例

```bash
# 遇到 auth 错误时检查认证状态
zentao-cli auth status

# 验证配置
echo $ZENTAO_URL
echo $ZENTAO_TOKEN
```

## Gotchas

1. **product 参数**：大多数情况下 `--product` 是必需的。如果没有指定 product，API 可能无法返回正确结果。

2. **project 与 product 的关系**：测试用例可以独立于项目存在（只关联 product），也可以同时关联项目和产品。查询时 `--project` 是可选的。

3. **status 值**：注意区分 testcase 的 status（wait/normal/blocked/bypass）和 bug 的 status（active/resolved/closed）。

4. **version 字段**：ZenTao 测试用例有版本概念，每次修改会用例版本递增。获取详情时可以查看是哪个版本的用例。

5. **执行结果命令名**：执行测试用例结果时使用 `zentao-cli testcase result <id> --result pass`，不是旧的 `execute` 子命令。

