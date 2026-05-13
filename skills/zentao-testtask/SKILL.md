---
name: zentao-testtask
version: 1.0.0
description: "禅道(ZenTao) 测试单（Testtask）管理 — 列出测试单、获取测试单详情。当用户说：'查询测试单'、'测试单列表'、'有哪些测试'、'testtask 列表'、'查看测试单'、'测试单详情'、'禅道测试' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli testtask --help"
---

# testtask (v1)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`testtask list`](./references/zentao-testtask-list.md) — 列出测试单
- [`testtask get`](./references/zentao-testtask-get.md) — 获取测试单详情

## Core Concepts

- **Testtask（测试单）**：ZenTao 中用于管理测试任务的单元。
- **与执行的关系**：测试单关联到特定项目（project）和执行（execution）。
- **测试单状态**：wait（等待）/ doing（进行中）/ done（已完成）/ closed（已关闭）

## Testtask Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `wait` | 等待 | 测试任务还未开始 |
| `doing` | 进行中 | 测试正在执行 |
| `done` | 已完成 | 测试已完成 |
| `closed` | 已关闭 | 测试单已关闭 |

## Common Use Cases

### 1. 查看项目的测试单

```bash
# 列出项目 1 下的所有测试单
zentao-cli testtask list --project 1
```

### 2. 查看特定测试单详情

```bash
# 获取测试单详情
zentao-cli testtask get 1
```

## Examples

```bash
# 列出项目 1 下的所有测试单
zentao-cli testtask list --project 1

# 获取测试单详情
zentao-cli testtask get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 测试单 ID |
| name | string | 测试单名称 |
| project | u64 | 所属项目 ID |
| execution | u64 | 所属执行/迭代 ID |
| status | string | 测试单状态：wait / doing / done / closed |
| type | string | 测试单类型 |
| product | u64 | 所属产品 ID |

## Gotchas

1. **Project Required**：`testtask list` 需要指定 `--project` 参数来筛选特定项目的测试单。

2. **与 Testcase 的区别**：Testtask 是测试任务，Testcase 是具体的测试用例。

3. **执行关联**：测试单通过 `execution` 字段关联到具体的执行/迭代。
