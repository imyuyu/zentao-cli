---
name: zentao-execution
version: 0.3.0
description: "禅道(ZenTao) 执行/迭代（Execution）管理 — 列出执行、获取执行详情、创建执行。当用户说：'查询执行'、'执行列表'、'execution 列表'、'查看执行'、'执行详情'、'创建执行'、'新建 execution'、'迭代'、'Sprint'、'里程碑'、'禅道执行' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli execution --help"
---

# Execution (执行/迭代) Management

**MUST** - 在使用此模块前，先阅读 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md) 了解认证和环境变量配置。

## Core Concepts

- **Execution**: An execution (also called iteration or milestone) is a concrete work unit within a project
- **Execution ID**: Unique identifier for an execution
- **Project**: Executions belong to a project
- **Execution Type**: `iteration` (迭代) or `milestone` (里程碑)
- **Status**: `wait` (未开始), `doing` (进行中), `closed` (已关闭), `suspended` (已暂停)

## Shortcuts (推荐优先使用)

| Shortcut | 说明 |
|----------|------|
| `+execution-list` | List executions for a project |
| `+execution-get` | Get execution details |
| `+execution-create` | Create a new execution |
| `+execution-update` | Update an execution |
| `+execution-delete` | Delete an execution |

## Commands

### List Executions
```bash
# List executions for a project (通过项目获取执行列表)
zentao-cli execution +list --project 1

# List all executions (不推荐，可能需要较大权限)
zentao-cli execution +list
```

### Get Execution
```bash
zentao-cli execution +get 100
```

### Create Execution
```bash
zentao-cli execution +create --name "Sprint 1" --project 1 --begin 2024-01-01 --end 2024-01-14
```

### Update Execution
```bash
zentao-cli execution +update 100 --status closed
```

### Delete Execution
```bash
zentao-cli execution +delete 100
```

## API Endpoint

| 操作 | URL 模式 |
|------|----------|
| 列表 | `GET /api.php/v1/projects/{projectId}/executions` |
| 详情 | `GET /api.php/v1/executions/{id}` |

详细命令参考见 references/ 目录。
