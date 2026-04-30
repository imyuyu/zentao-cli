# product list

列出所有有权限访问的产品列表，支持分页和多种输出格式。

## Command

```bash
zentao-cli product list
zentao-cli shortcuts products [--page-limit <n>] [--page-delay <ms>] [--page-all]
```

## Shortcuts (AI Agent Friendly)

```bash
zentao-cli shortcuts +products
zentao-cli shortcuts +products --page-limit 50
zentao-cli shortcuts +products --page-all
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
# List all products (default: table format)
zentao-cli product list

# List products with JSON output
zentao-cli shortcuts +products --format json

# List products in CSV format (for Excel)
zentao-cli shortcuts +products --format csv

# List products in NDJSON format (for pipeline processing)
zentao-cli shortcuts +products --format ndjson

# List first 50 products
zentao-cli shortcuts +products --page-limit 50

# Get all products (paginated, with 200ms delay between requests)
zentao-cli shortcuts +products --page-all --page-delay 200
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 产品 ID |
| name | string | 产品名称 |
| code | string | 产品代号（英文标识） |
| status | string | 产品状态：normal（正常）/ closed（关闭） |
| desc | string | 产品描述（可选） |
