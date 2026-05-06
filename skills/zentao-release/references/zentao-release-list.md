# release list

列出所有发布。

## Command
```bash
zentao-cli release list [--product <id>] [--project <id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | No | Filter by product ID |
| `--project` | No | Filter by project ID |

## Examples

```bash
# List all releases
zentao-cli release list

# List releases for a specific product
zentao-cli release list --product 1

# List releases for a specific project
zentao-cli release list --project 1
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

