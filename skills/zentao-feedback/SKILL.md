---
name: zentao-feedback
version: 1.0.0
description: "禅道(ZenTao) 反馈（Feedback）管理 — 列出反馈、获取反馈详情。当用户说：'查询反馈'、'反馈列表'、'有哪些反馈'、'feedback 列表'、'查看反馈'、'反馈详情'、'用户反馈'、'禅道反馈' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli feedback --help"
---

# feedback (v1)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`feedback list`](./references/zentao-feedback-list.md) — 列出反馈
- [`feedback get`](./references/zentao-feedback-get.md) — 获取反馈详情
- [`feedback create`](./references/zentao-feedback-create.md) — 创建反馈
- [`feedback assign`](./references/zentao-feedback-assign.md) — 指派反馈
- [`feedback close`](./references/zentao-feedback-close.md) — 关闭反馈
- [`feedback update`](./references/zentao-feedback-update.md) — 修改反馈
- [`feedback delete`](./references/zentao-feedback-delete.md) — 删除反馈

## Core Concepts

- **Feedback（反馈）**：ZenTao 中用于收集和管理用户反馈的功能模块。
- **反馈类型**：通过 `type` 字段区分不同类型的反馈。
- **反馈状态**：open（待处理）/ assigned（已指派）/ closed（已关闭）

## Feedback Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `open` | 待处理 | 反馈待处理 |
| `assigned` | 已指派 | 反馈已指派给人员 |
| `closed` | 已关闭 | 反馈已关闭 |

## Common Use Cases

### 1. 查看所有反馈

```bash
# 列出所有反馈
zentao-cli feedback list
```

### 2. 查看特定反馈详情

```bash
# 获取反馈详情
zentao-cli feedback get 1
```

## Examples

```bash
# 列出所有反馈
zentao-cli feedback list

# 获取反馈详情
zentao-cli feedback get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 反馈 ID |
| title | string | 反馈标题 |
| type | string | 反馈类型 |
| status | string | 反馈状态：open / assigned / closed |
| pri | u8 | 优先级 |
| desc | string | 反馈描述 |

## Gotchas

1. **反馈与工单**：反馈（Feedback）和工单（Ticket）是不同的模块，反馈通常用于收集用户意见，工单用于处理具体问题。

2. **指派处理**：反馈可以通过指派给相关人员进行处理。

3. **优先级**：通过 `pri` 字段设置反馈的处理优先级。
