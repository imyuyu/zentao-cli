---
name: zentao-ticket
version: 1.0.0
description: "禅道(ZenTao) 工单（Ticket）管理 — 列出工单、获取工单详情。当用户说：'查询工单'、'工单列表'、'有哪些工单'、'ticket 列表'、'查看工单'、'工单详情'、'处理工单'、'禅道工单' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli ticket --help"
---

# ticket (v1)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`ticket list`](./references/zentao-ticket-list.md) — 列出工单
- [`ticket get`](./references/zentao-ticket-get.md) — 获取工单详情
- [`ticket create`](./references/zentao-ticket-create.md) — 创建工单
- [`ticket update`](./references/zentao-ticket-update.md) — 修改工单
- [`ticket delete`](./references/zentao-ticket-delete.md) — 删除工单

## Core Concepts

- **Ticket（工单）**：ZenTao 中用于处理用户问题报告的功能模块。
- **工单类型**：通过 `type` 字段区分不同类型的工单。
- **工单状态**：表示工单当前的处理状态。
- **严重程度**：通过 `severity` 字段表示工单的严重程度。
- **优先级**：通过 `pri` 字段表示工单的处理优先级。

## Ticket Severity Levels

| Level | Name | Description |
|-------|------|-------------|
| 1 | Critical | 紧急 - 系统崩溃或功能完全不可用 |
| 2 | Major | 重要 - 主要功能失效 |
| 3 | Normal | 普通 - 普通问题 |
| 4 | Minor | 轻微 - 轻微问题 |

## Common Use Cases

### 1. 查看所有工单

```bash
# 列出所有工单
zentao-cli ticket list
```

### 2. 查看特定工单详情

```bash
# 获取工单详情
zentao-cli ticket get 1
```

## Examples

```bash
# 列出所有工单
zentao-cli ticket list

# 获取工单详情
zentao-cli ticket get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 工单 ID |
| title | string | 工单标题 |
| type | string | 工单类型 |
| status | string | 工单状态 |
| pri | u8 | 优先级 |
| severity | u8 | 严重程度 |

## Gotchas

1. **Ticket vs Feedback**：工单（Ticket）通常用于处理需要解决的问题，反馈（Feedback）更多用于收集用户意见。

2. **Severity vs Priority**：severity 表示问题的严重程度，pri 表示处理的优先级。

3. **状态流转**：工单有明确的状态流转规则，根据实际业务设置。
