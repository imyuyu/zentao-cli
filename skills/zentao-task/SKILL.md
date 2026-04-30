---
name: zentao-task
version: 1.2.0
description: "禅道(ZenTao) 任务管理 — 创建任务、查看任务列表、更新任务状态、分配负责人、开始任务、暂停任务、完成任务、关闭任务。当用户说：'查询任务'、'任务列表'、'有哪些任务'、'task 列表'、'查看任务'、'task 详情'、'创建任务'、'新建 task'、'指派任务'、'开始任务'、'完成任务'、'任务状态'、'待办任务'、'我的任务'、'禅道任务' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli task --help"
---

# Task (任务)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、权限处理、配置说明**

> **任务搜索技巧**：先区分用户是否**特地指定使用搜索 skill**，以及是否真的提供了**查询关键字**（例如任务名称、关键词、片段描述）。如果用户特地指定使用搜索 skill，或明确给出了任务查询关键字，则优先使用搜索型 shortcut。如果用户没有特地指定使用搜索 skill，且意图里没有查询关键字，只有范围条件（例如"已完成""由我创建""指派给我"），应优先使用列表型能力（`+task-list`）。
>
> **意图区分补充**：像"搜索任务"这类表达，虽然字面带有"搜索"，但如果没有真正的查询关键字，且本质是在限定范围条件，应优先走 `+task-list`。
>
> **用户身份识别**：在用户身份场景下，如果用户提到了"我"（例如"分配给我"、"由我创建"），请获取当前登录用户的相关信息。
>
> **友好输出**：在输出任务详情时，建议同时提取并输出命令返回结果中的相关链接字段（如果有），以便用户可以直接点击跳转查看详情。

## Shortcuts

- [`+task-list`](./references/zentao-task-list.md) — List tasks
- [`+task-get`](./references/zentao-task-get.md) — Get task details
- [`+task-create`](./references/zentao-task-create.md) — Create a task
- [`+task-update`](./references/zentao-task-update.md) — Update a task
- [`+task-delete`](./references/zentao-task-delete.md) — Delete a task

## API Resources

```bash
zentao-cli task +list --project <id>    # 列出任务
zentao-cli task +get <task_id>    # 获取任务详情
zentao-cli task +create --name <name> --project <id> --pri <priority>  # 创建任务
zentao-cli task +update <task_id> [flags]  # 更新任务
zentao-cli task +delete <task_id>  # 删除任务
```

## Common Use Cases

### 列出项目任务

```bash
# 列出项目所有任务
zentao-cli task +list --project 1

# 只看我被指派的任务
zentao-cli task +list --project 1 --assigned-to me
```

### 获取任务详情

```bash
zentao-cli task +get 456
```

### 创建任务

```bash
# 创建简单任务
zentao-cli task +create --name "实现登录功能" --project 1 --pri 3

# 创建任务并指定负责人、预估工时
zentao-cli task +create --name "代码评审" --project 1 --pri 2 --assigned-to developer-name --estimate 4
```

### 更新任务

```bash
# 更新任务状态
zentao-cli task +update 456 --status done

# 更新任务负责人
zentao-cli task +update 456 --assigned-to another-user

# 更新任务状态为进行中
zentao-cli task +update 456 --status doing
```

## 任务状态值

| 状态 | 说明 |
|------|------|
| `wait` | 等待中 |
| `doing` | 进行中 |
| `done` | 已完成 |
| `closed` | 已关闭 |

## 优先级

| 级别 | 说明 |
|------|------|
| 1 | 最高 |
| 2 | 高 |
| 3 | 中 |
| 4 | 低 |

## Error Handling

### 常见错误

| 错误码 | 说明 | 处理方式 |
|--------|------|----------|
| `ZEN_NOT_FOUND` | 任务不存在 | 检查任务 ID 是否正确 |
| `ZEN_AUTH_FAILED` | 认证失败 | 检查 ZENTAO_TOKEN 是否有效 |
| `ZEN_PARAM_MISSING` | 缺少参数 | 查看命令帮助确认必需参数 |

### 错误处理示例

```bash
# 查看认证状态
zentao-cli auth status

# 验证配置
echo $ZENTAO_URL
echo $ZENTAO_TOKEN
```

## Gotchas

1. **只有设置了 `due`（截止时间）的情况下，才能设置 `reminder`（提醒时间）**
2. **如果同时设置了 `start`（开始时间）和 `due`（截止时间），开始时间必须小于或等于截止时间**
3. **任务 ID 是全局唯一标识**，不是任务编号（例如 `t104121`）
4. **删除操作不可恢复**，执行前请确认用户意图

## Field Reference

| 字段 | 类型 | 说明 |
|------|------|------|
| id | u64 | 任务 ID |
| name | string | 任务名称 |
| project | u64 | 所属项目 ID |
| execution | u64 | 所属执行/迭代 ID |
| assigned_to | string | 负责人用户名 |
| status | string | 任务状态 |
| pri | u64 | 优先级 |
| estimate | float | 预估工时（小时） |
| consumed | float | 已消耗工时（小时） |
| start | string | 开始时间 |
| due | string | 截止时间 |
| created_by | string | 创建人 |
| created_date | string | 创建时间 |
