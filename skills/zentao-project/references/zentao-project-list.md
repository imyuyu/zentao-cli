# project +list

列出所有项目。

## Command

```bash
zentao-cli project +list
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Project ID |
| name | string | Project name |
| code | string | Project code |
| status | string | Project status：wait / doing / closed / suspended |
| desc | string | Project description（可选） |
