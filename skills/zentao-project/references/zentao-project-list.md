# project list

列出所有项目，支持分页和多种输出格式。

## Command

```bash
zentao project list
zentao shortcuts projects [--page-limit <n>] [--page-delay <ms>] [--page-all]
```

## Shortcuts (AI Agent Friendly)

```bash
zentao shortcuts +projects
zentao shortcuts +projects --page-limit 50
zentao shortcuts +projects --page-all
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--page-limit` | No | 每页数量（默认 100，最大 500） |
| `--page-delay` | No | 分页请求间隔毫秒（默认 100） |
| `--page-all` | No | 获取所有数据（分页遍历） |
| `--format` | No | 输出格式：json / pretty / table / ndjson / csv（默认 table） |

## Examples

```bash
# List all projects (default: table format)
zentao project list

# List projects with JSON output
zentao shortcuts +projects --format json

# List projects in CSV format (for Excel)
zentao shortcuts +projects --format csv

# List projects in NDJSON format (for pipeline processing)
zentao shortcuts +projects --format ndjson

# List first 50 projects
zentao shortcuts +projects --page-limit 50

# Get all projects (paginated, with 200ms delay between requests)
zentao shortcuts +projects --page-all --page-delay 200
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Project ID |
| name | string | Project name |
| code | string | Project code |
| status | string | Project status：wait / doing / closed / suspended |
| desc | string | Project description（可选） |
