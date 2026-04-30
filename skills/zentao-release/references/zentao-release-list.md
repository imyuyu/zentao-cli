# release list

列出某个产品下的发布列表。

## Command
```bash
zentao-cli release list [--product <id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | No | Product ID |

## Examples

```bash
# List all releases
zentao-cli release list

# List releases for product 1
zentao-cli release list --product 1
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
