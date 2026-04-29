# story list

列出某个产品下的需求列表，支持分页和多种输出格式。

## Command

```bash
zentao story list --product <id> [--status <status>] [--project <id>]
zentao shortcuts stories --product <id> [--page-limit <n>] [--page-delay <ms>] [--page-all]
```

## Shortcuts (AI Agent Friendly)

```bash
zentao shortcuts +stories --product 1
zentao shortcuts +stories --product 1 --page-limit 50
zentao shortcuts +stories --product 1 --page-all
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | Yes | 产品 ID |
| `--status` | No | 按状态筛选：draft / active / closed / changed |
| `--project` | No | 按项目 ID 筛选 |
| `--page-limit` | No | 每页数量（默认 100，最大 500） |
| `--page-delay` | No | 分页请求间隔毫秒（默认 100） |
| `--page-all` | No | 获取所有数据（分页遍历） |
| `--format` | No | 输出格式：json / pretty / table / ndjson / csv（默认 table） |

## Examples

```bash
# List stories for product 1 (default: table format)
zentao story list --product 1

# List stories with JSON output
zentao shortcuts +stories --product 1 --format json

# List stories in CSV format (for Excel)
zentao shortcuts +stories --product 1 --format csv

# List stories in NDJSON format (for pipeline processing)
zentao shortcuts +stories --product 1 --format ndjson

# List first 50 stories
zentao shortcuts +stories --product 1 --page-limit 50

# Get all stories (paginated, with 200ms delay between requests)
zentao shortcuts +stories --product 1 --page-all --page-delay 200
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 需求 ID |
| title | string | 需求标题 |
| product | u64 | 产品 ID |
| status | string | 需求状态：draft / active / closed / changed |
| pri | u8 | 优先级：0-5 |
| category | string | 需求类别（可选）：feature / requirement / bug / improvement |
| stage | string | 当前阶段（可选）：wait / plan / developed / 测试中 / released / closed |
| module | u64 | 所属模块 ID（可选） |
| assigned_to | string | 指派人（可选） |
| estimate | f64 | 预估工时（小时）（可选） |
| version | u64 | 版本号（可选） |
