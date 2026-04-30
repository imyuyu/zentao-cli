# project get

获取项目详情。

## Command
```bash
zentao-cli project get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Project ID |

## Examples

```bash
zentao-cli project get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Project ID |
| name | string | Project name |
| code | string | Project code |
| status | string | Project status |
| desc | string | Project description |
| acl | string | Access control level |
| opened_by | string | Creator |
| opened_date | string | Creation date |
|PM | u64 | Project manager ID |
| team | string | Team name |
| users | array | Team members |
