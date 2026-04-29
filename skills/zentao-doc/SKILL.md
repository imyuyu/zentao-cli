---
name: zentao-doc
version: 0.1.0
description: ZenTao Doc (文档) management - list and get documents from document libraries
---

# Doc (文档) Management

**MUST** - 在使用此模块前，先阅读 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md) 了解认证和环境变量配置。

## Core Concepts

- **Doc**: A document in ZenTao's document library
- **Doc ID**: Unique identifier for a document
- **Lib**: Document library ID - documents belong to a library
- **Product/Project**: Documents can be associated with products or projects

## Shortcuts (推荐优先使用)

| Shortcut | 说明 |
|----------|------|
| `+doc-list` | List all documents |
| `+doc-get` | Get document details |

## Commands

### List Documents
```bash
zentao doc list
```

### Get Document
```bash
zentao doc get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Document ID |
| title | string | Document title |
| product | u64 | Associated product ID |
| project | u64 | Associated project ID |
| lib | u64 | Document library ID |
| type | string | Document type (e.g., "doc", "article") |
| size | string | Document size in bytes |
| added_by | string | Creator account |
| added_date | string | Creation date |
| edited_date | string | Last edit date |
| deleted | string | Deletion flag ("0"/"1") |

## Examples

```bash
# List all documents
zentao doc list

# Get document details by ID
zentao doc get 10
```
