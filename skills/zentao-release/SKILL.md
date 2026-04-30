---
name: zentao-release
version: 0.2.0
description: ZenTao Release (发布) management - list and get product releases
metadata:
  bins:
    - zentao-cli
  cliHelp: |
    # Release Management
    zentao-cli release list              # List all releases
    zentao-cli release get <id>         # Get release details
---

# Release (发布) Management

**MUST** - 在使用此模块前，先阅读 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md) 了解认证和环境变量配置。

## Core Concepts

- **Release**: A product release in ZenTao, associated with a specific build/version
- **Release ID**: Unique identifier for a release
- **Build**: The build/version ID that the release references

## Shortcuts (推荐优先使用)

| Shortcut | 说明 |
|----------|------|
| `+release-list` | List all releases |
| `+release-get` | Get release details |

## Commands

### List Releases
```bash
zentao-cli release +list
zentao-cli release +list --product 1
zentao-cli release +list --project 1
```

### Get Release
```bash
zentao-cli release +get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Release ID |
| name | string | Release name (e.g., "v1.0.0") |
| product | u64 | Product ID |
| build | u64 | Associated Build ID |
| status | string | Release status (normal/closed) |
| marker | string | Release marker (e.g., "stable", "beta") |
| date | string | Release date |

## Examples

```bash
# List all releases
zentao-cli release +list

# Get release details
zentao-cli release +get 1
```
