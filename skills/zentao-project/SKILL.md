---
name: zentao-project
version: 2.1.0
description: "ZenTao 项目（Project）管理 — 列出项目、获取项目详情。项目是一组关联的任务和团队成员。当用户需要查看项目列表、了解某个项目的详细信息、或查看项目下的任务时使用。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli project --help"
---

# project (v2)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Shortcuts

- [`+project-list`](./references/zentao-project-list.md) — 列出所有项目
- [`+project-get`](./references/zentao-project-get.md) — 获取项目详情

## Core Concepts

- **Project（项目）**：ZenTao 中组织任务和团队成员的实体，一个项目可以关联多个产品。
- **Project ID**：项目的唯一标识符，查看任务和团队成员时需要使用。
- **Project 与 Product 的关系**：项目从产品中选取需求来实施，一个项目可以对应多个产品，一个产品也可以对应多个项目。

## Project Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `wait` | 等待中 | 项目未开始 |
| `doing` | 进行中 | 项目正在进行 |
| `suspended` | 挂起 | 项目已暂停 |
| `closed` | 已关闭 | 项目已结束 |

## Project Team

| Field | Type | Description |
|-------|------|-------------|
| PM | u64 | 项目经理 ID |
| team | string | 团队名称 |
| users | array | 团队成员列表 |

## API Resources

```bash
zentao-cli project list           # 列出所有项目
zentao-cli project list --status <status>  # 按状态筛选
zentao-cli project get <id>       # 获取项目详情
```

> **重要**：使用原生命令时，可以先运行 `zentao-cli project --help` 查看完整选项。

## Common Use Cases

### 1. 查看所有项目

```bash
# 列出所有项目
zentao-cli project list
```

### 2. 按状态筛选项目

```bash
# 只看正在进行中的项目
zentao-cli project list --status doing

# 查看已关闭的项目
zentao-cli project list --status closed
```

### 3. 查看特定项目详情

```bash
# 获取项目详情
zentao-cli project get 1
```

### 4. 结合 Story 和 Task 使用

项目通常与需求和任务管理结合使用：

```bash
# 查看项目详情
zentao-cli project get 1

# 查看该项目关联的需求
zentao-cli story list --product 1 --project 1

# 查看该项目的任务
zentao-cli task list --project 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 项目 ID |
| name | string | 项目名称 |
| code | string | 项目代号 |
| status | string | 项目状态：wait / doing / suspended / closed |
| desc | string | 项目描述 |
| acl | string | 访问控制级别 |
| opened_by | string | 创建者 |
| opened_date | string | 创建日期 |
| PM | u64 | 项目经理 ID |
| team | string | 团队名称 |
| users | array | 团队成员 |

## Gotchas

1. **Project vs Product**：区分项目和产品：
   - **Product**：产品视角，关注"要做什么功能"（需求）
   - **Project**：项目视角，关注"怎么实现这些功能"（任务）

2. **Status Filtering**：`--status` 参数支持 `wait`、`doing`、`suspended`、`closed` 四个值，不区分大小写。

3. **Project ACL**：项目有访问控制级别（acl），只有项目成员或符合权限条件的用户才能查看项目详情。

4. **Team Members**：通过 `users` 字段返回团队成员列表，完整成员详情可能需要额外查询。

5. **Project Manager**：PM 字段只返回项目经理的用户 ID，需要时可结合 `zentao-cli user get <id>` 获取更多信息。

6. **Opened Date Format**：返回的日期格式为 ZenTao API 标准格式，可能需要转换本地时区显示。
