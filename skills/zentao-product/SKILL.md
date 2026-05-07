---
name: zentao-product
version: 2.2.0
description: "禅道(ZenTao) 产品（Product）管理 — 列出产品、获取产品详情、创建产品、更新产品、关闭产品。当用户说：'查询产品'、'产品列表'、'有哪些产品'、'product 列表'、'查看产品'、'product 详情'、'创建产品'、'新建 product'、'产品信息'、'产品线'、'禅道产品' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli product --help"
---

# product (v2)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置、错误处理和通用选项说明。**

## Commands

- [`product list`](./references/zentao-product-list.md) — 列出所有可访问的产品
- [`product get`](./references/zentao-product-get.md) — 获取产品详情
- [`product create`](./references/zentao-product-create.md) — 创建新产品
- [`product update`](./references/zentao-product-update.md) — 更新产品信息
- [`product delete`](./references/zentao-product-delete.md) — 删除产品

## Core Concepts

- **Product（产品）**：ZenTao 中的顶层实体，一个产品包含多个需求（Story）和缺陷（Bug）。
- **Product ID**：产品的唯一标识符，创建需求时需要指定所属产品 ID。
- **Product 与 Project 的关系**：产品定义"做什么"，项目定义"怎么做"。一个产品可以关联多个项目。

## Product Status Values

| Status | 中文 | 说明 |
|--------|------|------|
| `normal` | 正常 | 产品处于正常可用状态 |
| `closed` | 关闭 | 产品已关闭，不再接收新需求 |

> 使用前可先运行 `zentao-cli product --help` 查看完整选项。

## Common Use Cases

### 1. 查看所有可访问的产品

```bash
# 列出所有产品
zentao-cli product list
```

### 2. 查看特定产品详情

```bash
# 获取产品详情
zentao-cli product get 1
```

### 3. 结合 Story 使用

产品通常与需求管理结合使用：

```bash
# 先获取产品列表，了解有哪些产品
zentao-cli product list

# 再查看某个产品下的需求
zentao-cli story list --product 1
```

## Examples

```bash
# 列出所有产品
zentao-cli product list

# 获取产品详情
zentao-cli product get 1

# 查看某产品下的需求
zentao-cli story list --product 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 产品 ID |
| name | string | 产品名称 |
| code | string | 产品代号（英文标识） |
| status | string | 产品状态：normal（正常）/ closed（关闭） |
| desc | string | 产品描述（可选） |

## Gotchas

1. **No Filtering Options**：`product list` 命令不需要额外参数，直接列出所有有权限访问的产品。

2. **Product vs Project**：区分产品和项目：
   - **Product**：产品视角，关注"要做什么功能"
   - **Project**：项目视角，关注"怎么实现这些功能"

3. **Product ID Required**：创建需求（Story）时必须指定 `--product` 参数。

4. **Product ACL**：产品有权限控制，只有被分配了产品权限的用户才能查看或操作该产品下的资源。

5. **Closed Product**：已关闭的产品（status=closed）通常不再接收新的需求或 Bug。

