---
name: zentao-doc
version: 0.3.0
description: "禅道(ZenTao) 文档（Doc）管理 — 列出文档、查看文档详情。当用户说：'查询文档'、'文档列表'、'doc 列表'、'查看文档'、'文档详情'、'文档库'、'wiki'、'禅道文档' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
  cliHelp: "zentao-cli doc --help"
---

# doc (v0.2.0)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量配置和错误处理。**

## Shortcuts

- `+doc-list` — 列出文档列表
- `+doc-get` — 获取文档详情

## Commands

### List Documents
```bash
zentao-cli doc +list
```

### Get Document
```bash
zentao-cli doc +get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 文档 ID |
| title | string | 文档标题 |
| product | u64 | 关联产品 ID |
| project | u64 | 关联项目 ID |
| lib | u64 | 文档库 ID |
| type | string | 文档类型（如 "doc", "article"） |
| size | string | 文档大小（字节） |
| added_by | string | 创建者账号 |
| added_date | string | 创建日期 |
| edited_date | string | 最后编辑日期 |
| deleted | string | 删除标志（"0"/"1"） |

## Examples

```bash
# 列出所有文档
zentao-cli doc +list

# 根据 ID 获取文档详情
zentao-cli doc +get 10
```
