---
name: zentao-user
version: 1.2.0
description: "禅道(ZenTao) 用户（User）管理 — 列出用户、查看用户详情、按部门/角色查询用户。当用户说：'查询用户'、'用户列表'、'user 列表'、'查看用户'、'用户详情'、'团队成员'、'成员列表'、'谁负责'、'开发者'、'测试人员'、'禅道用户' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
    envs: ["ZENTAO_URL", "ZENTAO_TOKEN"]
  cliHelp: "zentao-cli user --help"
---

# User (v2)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量和错误处理。**

> **用户识别技巧**：ZenTao 中用户通过数字 ID 标识，但在指派任务、Bug 等场景中可能使用 account（账号名）。API 调用时注意区分 `id` 和 `account`。
>
> **用户身份场景**：当用户提到"我"（例如"指派给我"、"我创建的"），需要使用当前登录用户的 ID。可以通过 `zentao-cli user list` 查看自己的账号信息。
>
> **部门与角色**：用户归属于部门（dept），拥有角色（role）。常见角色包括：dev（开发）、manager（经理）、qa（测试）、pd（产品）等。
>
> **友好输出**：在输出用户详情时，除了展示 `account` 和 `id`，还应展示 `realname`（真实姓名）以便识别。

## User Role Types

| Role | Description | 描述 |
|------|-------------|------|
| dev | Developer | 开发人员 |
| manager | Manager | 经理 |
| qa | QA/Test | 测试人员 |
| pd | Product Designer | 产品设计 |
| op | Operations | 运维 |
| admin | Administrator | 管理员 |

## Commands

- [`user list`](./references/zentao-user-list.md) — List users with optional filters
- [`user get`](./references/zentao-user-get.md) — Get user details by ID
- [`user me`](./references/zentao-user-me.md) — Get current user info
- [`user create`](./references/zentao-user-create.md) — Create a new user
- [`user update`](./references/zentao-user-update.md) — Update user information
- [`user delete`](./references/zentao-user-delete.md) — Delete a user

## Common Use Cases

### 场景 1：查询所有用户

```bash
# 列出系统中的所有用户
zentao-cli user list
```

### 场景 2：按部门查询用户

```bash
# 查看某部门下的所有用户
zentao-cli user list --dept 1

# 组合筛选：某部门的开发人员
zentao-cli user list --dept 1 --role dev
```

### 场景 3：按角色查询用户

```bash
# 查看所有开发人员
zentao-cli user list --role dev

# 查看所有测试人员
zentao-cli user list --role qa
```

### 场景 4：获取用户详情

```bash
# 通过用户 ID 获取详细信息
zentao-cli user get 123

# 输出包括：账号、姓名、邮箱、部门、角色等
```

## Examples

```bash
# 列出系统中的所有用户
zentao-cli user list

# 查看某部门下的所有用户
zentao-cli user list --dept 1

# 查看所有开发人员
zentao-cli user list --role dev

# 通过用户 ID 获取详细信息
zentao-cli user get 123
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 用户 ID（唯一标识） |
| account | string | 登录账号名 |
| realname | string | 真实姓名 |
| email | string | 邮箱（可选） |
| dept | u64 | 部门 ID（可选） |
| role | string | 角色（可选） |
| phone | string | 电话（可选） |
| mobile | string | 手机（可选） |

## Error Handling

### 常见错误

| 错误码 | 说明 | 解决方案 |
|--------|------|----------|
| `ZEN_AUTH_FAILED` | Token 无效或过期 | 检查 ZENTAO_TOKEN 配置 |
| `ZEN_NOT_FOUND` | 用户不存在 | 检查用户 ID 是否正确 |
| `ZEN_CONFIG_INVALID` | 配置无效 | 验证 ZENTAO_URL 和 ZENTAO_TOKEN |
| `ZEN_API_ERROR` | ZenTao API 返回错误 | 检查服务器状态或稍后重试 |

### 错误处理示例

```bash
# 遇到 auth 错误时检查认证状态
zentao-cli auth status

# 验证配置
echo $ZENTAO_URL
echo $ZENTAO_TOKEN
```

## Gotchas

1. **id vs account**：用户有 `id`（数字）和 `account`（账号名字符串）两种标识方式。API 参数可能接受其中一种或两种都接受，注意区分。

2. **dept 为 0**：有些用户可能没有部门，dept 字段为 0 或空。

3. **role 字段值**：role 是字符串如 "dev"、"manager"，不是数字代码。使用 `--role` 参数时应使用完整的角色名字符串。

4. **用户不存在时的处理**：当查询的用户 ID 或账号不存在时，API 会返回错误而不是空列表。

5. **批量操作**：ZenTao 用户管理主要支持查询，不支持批量创建/更新用户（通常由管理员在 Web 端操作）。

