# release get

获取单个发布的详细信息。

## Command
```bash
zentao release get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Release ID |

## Examples

```bash
# Get release details
zentao release get 1
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
