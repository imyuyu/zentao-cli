# build +get

获取指定版本的详细信息。

## Command
```bash
zentao-cli build +get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Build ID |

## Examples

```bash
# Get build details by ID
zentao-cli build +get 10

# Get build details by ID 5
zentao-cli build +get 5
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Build ID |
| name | string | Build name |
| product | u64 | Product ID |
| project | u64 | Project ID |
| branch | u64 | Branch/Platform ID |
| scm_path | string | SCM repository path |
| ci | string | CI job name |
| pkg | string | Package path |
| file_size | string | File size in bytes |
| generated_at | string | Build generation timestamp |
| deleted | string | Deletion flag (0/1) |
| editor | string | Last editor |
| created_by | string | Creator |
| created_date | string | Creation date |
| last_edited_by | string | Last editor |
| last_edited_date | string | Last edit date |
| consumed_cards | string | Number of consumed cards |
| stories | string | Number of linked stories |
| bugs | string | Number of linked bugs |
