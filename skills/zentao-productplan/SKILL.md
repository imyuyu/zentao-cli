---
name: zentao-productplan
version: 1.0.0
description: "禅道(ZenTao) 产品计划（ProductPlan）管理 — 列出产品计划、获取产品计划详情。当用户说：'查询产品计划'、'产品计划列表'、'有哪些计划'、'productplan 列表'、'查看产品计划'、'计划详情'、'禅道产品计划' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli product-plan --help"
---

# productplan (v1)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`productplan list`](./references/zentao-productplan-list.md) — 列出产品计划
- [`productplan get`](./references/zentao-productplan-get.md) — 获取产品计划详情
- [`productplan create`](./references/zentao-productplan-create.md) — 创建产品计划
- [`productplan update`](./references/zentao-productplan-update.md) — 修改产品计划
- [`productplan delete`](./references/zentao-productplan-delete.md) — 删除产品计划

## Core Concepts

- **ProductPlan（产品计划）**：ZenTao 中用于规划产品发布的功能模块。
- **Plan vs Roadmap**：产品计划可以是发布计划（ship）或路线图（roadmap）。
- **与产品的关系**：每个产品计划属于特定产品（product）。

## ProductPlan Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `wait` | 未开始 | 计划还未开始 |
| `doing` | 进行中 | 计划正在执行 |
| `done` | 已完成 | 计划已完成 |

## Common Use Cases

### 1. 查看产品的所有计划

```bash
# 列出产品 1 下的所有计划
zentao-cli product-plan list --product 1
```

### 2. 查看特定计划详情

```bash
# 获取计划详情
zentao-cli product-plan get 1
```

## Examples

```bash
# 列出产品 1 下的所有计划
zentao-cli product-plan list --product 1

# 获取计划详情
zentao-cli product-plan get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 计划 ID |
| product | u64 | 所属产品 ID |
| name | string | 计划名称 |
| code | string | 计划代号 |
| status | string | 计划状态：wait / doing / done |
| type | string | 计划类型：ship / roadmap |

## Gotchas

1. **Product Required**：`productplan list` 需要指定 `--product` 参数来筛选特定产品的计划。

2. **计划类型**：通过 `type` 字段区分发布计划（ship）和路线图（roadmap）。

3. **与 Release 的区别**：ProductPlan 是规划层面的，Release 是实际发布的版本记录。
