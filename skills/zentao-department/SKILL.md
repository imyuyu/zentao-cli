---
name: zentao-department
version: 1.0.0
description: "禅道(ZenTao) 部门（Department）管理 — 列出部门、获取部门详情。当用户说：'查询部门'、'部门列表'、'有哪些部门'、'department 列表'、'查看部门'、'部门详情'、'组织架构'、'禅道部门' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli department --help"
---

# department (v1)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`department list`](./references/zentao-department-list.md) — 列出所有部门
- [`department get`](./references/zentao-department-get.md) — 获取部门详情

## Core Concepts

- **Department（部门）**：ZenTao 中的组织架构单元，用于管理用户所属部门。
- **Parent（父部门）**：部门可以有层级关系，parent 字段表示父部门 ID。
- **Path（路径）**：部门的完整路径，如 `/1/2/3/` 表示从根部门到当前部门的完整路径。

## Department Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 部门 ID |
| name | string | 部门名称 |
| parent | u64 | 父部门 ID（0 表示根部门） |
| order | u64 | 排序序号 |
| path | string | 部门路径，如 `/1/2/` |

## Common Use Cases

### 1. 查看所有部门

```bash
# 列出所有部门
zentao-cli department list
```

### 2. 查看特定部门详情

```bash
# 获取部门详情
zentao-cli department get 1
```

## Examples

```bash
# 列出所有部门
zentao-cli department list

# 获取部门详情
zentao-cli department get 1
```

## Gotchas

1. **部门层级**：部门有层级关系，可以通过 `path` 字段了解部门的完整层级位置。

2. **parent=0**：当 `parent` 为 0 时，表示该部门是根部门。

3. **与用户关联**：用户的 `dept` 字段关联到部门的 ID。
