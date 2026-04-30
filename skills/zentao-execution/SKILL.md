---
name: zentao-execution
version: 0.2.0
description: ZenTao Execution (执行/迭代) management - list and get executions
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

## API Endpoint

| 操作 | URL 模式 |
|------|----------|
| 列表 | `GET /api.php/v1/projects/{projectId}/executions` |
| 详情 | `GET /api.php/v1/executions/{id}` |

详细命令参考见 references/ 目录。
