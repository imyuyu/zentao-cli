---
name: zentao-build
version: 1.2.0
description: "禅道(ZenTao) 版本/Build 管理 — 列出版本、获取版本详情、创建版本、关联 Story 和 Bug。当用户说：'查询版本'、'版本列表'、'build 列表'、'查看版本'、'版本详情'、'创建版本'、'新建 build'、'发布版本'、'版本信息'、'CI 版本'、'禅道版本' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli build --help"
---

# Build (版本)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、权限处理、配置说明**

> **版本搜索技巧**：先区分用户是否**特地指定使用搜索 skill**，以及是否真的提供了**查询关键字**（例如版本名称、关键词）。如果用户特地指定使用搜索 skill，或明确给出了查询关键字，则优先使用搜索。如果用户没有特地指定使用搜索 skill，且意图里没有查询关键字，只有范围条件（例如"某产品的版本"、"某项目的版本"），应优先使用 `build list`。
>
> **友好输出**：在输出版本详情时，建议同时提取并输出命令返回结果中的相关链接字段（如果有），以便用户可以直接点击跳转查看详情。

## Commands

- [`build list`](./references/zentao-build-list.md) — List builds/versions
- [`build get`](./references/zentao-build-get.md) — Get build details
- [`build create`](./references/zentao-build-create.md) — Create a build
- [`build update`](./references/zentao-build-update.md) — Update a build
- [`build delete`](./references/zentao-build-delete.md) — Delete a build

## Common Use Cases

### 列出版本

```bash
# 列出所有版本
zentao-cli build list

# 列出某项目的版本
zentao-cli build list --project 1

# 列出某产品的版本
zentao-cli build list --product 1

# 同时按项目和产品筛选
zentao-cli build list --project 1 --product 1

# 列出某执行的版本
zentao-cli build list --execution 5
```

### 获取版本详情

```bash
# 获取版本详情
zentao-cli build get 10

# 获取版本详情（JSON 格式）
zentao-cli build get 10 --format json
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Build ID |
| name | string | Build 名称（例如 "v1.0.0", "Build-2024-01-15"） |
| product | u64 | 产品 ID |
| project | u64 | 项目 ID |
| branch | u64 | 分支/平台 ID |
| scm_path | string | SCM 仓库路径 |
| ci | string | CI 任务名称 |
| pkg | string | 包路径 |
| file_size | string | 文件大小（字节） |
| generated_at | string | 构建时间戳 |
| editor | string | 最后编辑者 |
| created_by | string | 创建人 |
| created_date | string | 创建时间 |
| last_edited_by | string | 最后编辑者 |
| last_edited_date | string | 最后编辑时间 |
| stories | string | 关联的需求数量 |
| bugs | string | 关联的 Bug 数量 |

## Error Handling

### 常见错误

| 错误码 | 说明 | 处理方式 |
|--------|------|----------|
| `ZEN_NOT_FOUND` | 版本不存在 | 检查 Build ID 是否正确 |
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

## Examples

```bash
# 列出所有版本
zentao-cli build list

# 列出某项目的版本
zentao-cli build list --project 1

# 列出某产品的版本
zentao-cli build list --product 1

# 获取版本详情
zentao-cli build get 10

# 获取版本详情（JSON 格式）
zentao-cli build get 10 --format json
```

## Gotchas

1. **Build ID 是全局唯一标识**，不是本地展示编号
2. **一个 Build 可能属于某个 Product 或 Project**，筛选时注意区分
3. **版本可能关联多个 Story 和 Bug**，查看详情时可注意 `stories` 和 `bugs` 字段

## 权限说明

| 操作 | 所需权限 |
|------|----------|
| `build list` | 项目/产品查看权限 |
| `build get` | 项目/产品查看权限 |

