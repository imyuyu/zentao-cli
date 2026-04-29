# build list

列出版本列表。

## Command
```bash
zentao build list [--project <id>] [--product <id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--project` | No | Filter by project ID |
| `--product` | No | Filter by product ID |

## Examples

```bash
# List all builds
zentao build list

# List builds for a specific project
zentao build list --project 5

# List builds for a specific product
zentao build list --product 1

# List builds with both filters
zentao build list --project 1 --product 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Build ID |
| name | string | Build name (e.g., "v1.0.0", "Build-2024-01-15") |
| product | u64 | Product ID |
| project | u64 | Project ID |
| branch | u64 | Branch/Platform ID |
| scm_path | string | SCM repository path |
| ci | string | CI job name |
| pkg | string | Package path |
| file_size | string | File size in bytes |
| generated_at | string | Build generation timestamp |
| stories | string | Number of linked stories |
| bugs | string | Number of linked bugs |
