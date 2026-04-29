---
name: zentao-execution
version: 0.1.0
description: ZenTao Execution (执行/迭代) management - list and get executions
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
zentao execution list --project 1
zentao execution list
```

### Get Execution
```bash
zentao execution get 100
```

详细命令参考见 references/ 目录。
