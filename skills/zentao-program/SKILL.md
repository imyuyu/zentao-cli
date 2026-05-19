---
name: zentao-program
version: 1.0.0
description: "禅道(ZenTao) 项目集（Program）管理 — 列出项目集、获取项目集详情。当用户说：'查询项目集'、'项目集列表'、'有哪些项目集'、'program 列表'、'查看项目集'、'项目集详情'、'禅道项目集' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli program --help"
---

# program (v1)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`program list`](./references/zentao-program-list.md) — 列出所有项目集
- [`program get`](./references/zentao-program-get.md) — 获取项目集详情
- [`program create`](./references/zentao-program-create.md) — 创建项目集
- [`program update`](./references/zentao-program-update.md) — 修改项目集
- [`program delete`](./references/zentao-program-delete.md) — 删除项目集

## Core Concepts

- **Program（项目集）**：ZenTao 中用于管理多个相关项目的顶层容器。
- **Program 与 Project 的关系**：一个项目集可以包含多个项目（Project）。
- **项目集状态**：doing（进行中）/ wait（等待）/ closed（已关闭）

## Program Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `doing` | 进行中 | 项目集正在执行 |
| `wait` | 等待 | 项目集等待开始 |
| `closed` | 已关闭 | 项目集已关闭 |

## Common Use Cases

### 1. 查看所有项目集

```bash
# 列出所有项目集
zentao-cli program list
```

### 2. 查看特定项目集详情

```bash
# 获取项目集详情
zentao-cli program get 1
```

## Examples

```bash
# 列出所有项目集
zentao-cli program list

# 获取项目集详情
zentao-cli program get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 项目集 ID |
| name | string | 项目集名称 |
| code | string | 项目集代号 |
| status | string | 项目集状态：doing / wait / closed |
| type | string | 项目集类型 |
| desc | string | 项目集描述 |
| parent | u64 | 父项目集 ID |

## Gotchas

1. **Program vs Project**：项目集（Program）是多个项目的容器，便于统一管理相关项目。

2. **层级关系**：项目集可以有层级关系，通过 `parent` 字段关联父项目集。

3. **权限控制**：只有被分配了项目集权限的用户才能查看或操作该项目集下的资源。
