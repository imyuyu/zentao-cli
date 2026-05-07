---
name: zentao-story
version: 2.2.0
description: "禅道(ZenTao) 需求（Story）管理 — 列出、查看、创建、更新需求。当用户说：'查询需求'、'需求列表'、'有哪些需求'、'story 列表'、'查看需求'、'story 详情'、'创建需求'、'新建 story'、'需求变更'、'关闭需求'、'需求状态'、'需求评审'、'禅道需求' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli story --help"
---

# story (v2)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和状态值定义。**

## Commands

- [`story list`](./references/zentao-story-list.md) — 列出产品下的需求列表
- [`story get`](./references/zentao-story-get.md) — 获取需求详情
- [`story create`](./references/zentao-story-create.md) — 创建新需求
- [`story update`](./references/zentao-story-update.md) — 更新需求信息
- [`story change`](./references/zentao-story-change.md) — 变更需求
- [`story delete`](./references/zentao-story-delete.md) — 删除需求
- [`story close`](./references/zentao-story-close.md) — 关闭需求

## Story Lifecycle

需求在 ZenTao 中有明确的生命周期状态：

```
draft → active → changed → closed
         ↑         ↓
         └─────────┘ (可重新激活)
```

### Valid Status Transitions

| Current Status | Valid Next Status | 说明 |
|---------------|-------------------|------|
| draft | active | 提交审核 |
| active | changed, closed | 审核通过或关闭 |
| changed | active, closed | 变更后重新审核或关闭 |
| closed | active | 重新激活 |

## Story Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `draft` | 草稿 | 需求草稿状态 |
| `active` | 活跃 | 已提交审核 |
| `changed` | 已变更 | 需求有变更待审核 |
| `closed` | 已关闭 | 需求已关闭 |

## Priority Levels

| Level | Name | 说明 |
|-------|------|------|
| 1 | 最高 | 最高优先级 |
| 2 | 高 | 高优先级 |
| 3 | 中 | 中优先级 |
| 4 | 低 | 低优先级 |

> 注意：优先级 0 通常表示未设置优先级。

## Story Types

| Type | 中文 | 说明 |
|------|------|------|
| `feature` | 功能 | 新功能需求 |
| `enhance` | 增强 | 功能增强 |
| `bugfix` | 缺陷 | Bug 修复 |
| `task` | 任务 | 任务类需求 |
| `story` | 用户故事 | 用户故事 |

## Story Stage Values

| Stage | 说明 |
|-------|------|
| `wait` | 未开始 |
| `plan` | 已计划 |
| `developed` | 已开发 |
| `testing` | 测试中 |
| `released` | 已发布 |
| `closed` | 已关闭 |

> 使用前可先运行 `zentao-cli story --help` 查看完整选项。

## Common Use Cases

### 1. 查看产品下的所有需求

```bash
# 列出产品 1 下的所有需求
zentao-cli story list --product 1

# 只看活跃的需求
zentao-cli story list --product 1 --status active
```

### 2. 查看特定需求详情

```bash
# 获取需求详情（包含描述、预估工时、阶段等）
zentao-cli story get 123
```

### 3. 创建新需求

```bash
# 基础创建
zentao-cli story create --title "用户登录功能" --product 1 --pri 1

# 完整创建（带类型和预估工时）
zentao-cli story create --title "用户注册功能" --product 1 --pri 1 --type feature --estimate 8
```

### 4. 更新需求状态

```bash
# 关闭需求
zentao-cli story update 123 --status closed

# 更新优先级
zentao-cli story update 123 --pri 2

# 变更需求状态并指派
zentao-cli story update 123 --status changed --assigned-to developer-name
```

### 5. 按项目筛选需求

```bash
# 查看项目 1 下的所有需求
zentao-cli story list --product 1 --project 1
```

## Examples

```bash
# 列出产品 1 下的所有需求
zentao-cli story list --product 1

# 只看活跃的需求
zentao-cli story list --product 1 --status active

# 获取需求详情
zentao-cli story get 123

# 创建新需求
zentao-cli story create --title "用户登录功能" --product 1 --pri 1

# 关闭需求
zentao-cli story update 123 --status closed
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 需求 ID |
| title | string | 需求标题 |
| product | u64 | 产品 ID |
| status | string | 需求状态：draft / active / closed / changed |
| pri | u8 | 优先级：0-5 |
| category | string | 需求类别：feature / requirement / bug / improvement |
| stage | string | 当前阶段：wait / plan / developed / testing / released / closed |
| module | u64 | 所属模块 ID |
| assigned_to | string | 指派人 |
| opened_by | string | 创建者 |
| estimate | f64 | 预估工时（小时） |
| version | u64 | 版本号（用于乐观锁） |

## Gotchas

1. **Story ID vs Title**：创建需求后返回的是需求 ID（数字），而不是 title，后续操作需要使用 ID。

2. **Status Transition**：只有符合状态转换规则的状态变更才会被接受：
   - `draft` → `active`：提交审核
   - `active` → `changed`/`closed`：审核动作
   - 错误的转换会返回错误

3. **Version for Updates**：更新需求时建议提供 `version` 字段以实现乐观锁，避免并发更新冲突。

4. **Estimate Unit**：`--estimate` 参数的单位是**小时**（hours）。

5. **Type vs Category**：Story 有 `type`（story type）和隐含的 `category`（需求类型），两者可能不同，根据实际业务选择。

6. **Closed Story Reactivation**：已关闭的需求可以通过设置为 `active` 重新激活。

7. **Product Required**：创建需求时 `--product` 参数是必需的，且需求必须归属于某个产品。

