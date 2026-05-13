# department get

获取指定部门的详细信息。

## Command

```bash
zentao-cli department get <id>
```

## Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| id | u64 | Yes | 部门 ID |

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 部门 ID |
| name | string | 部门名称 |
| parent | u64 | 父部门 ID（0 表示根部门） |
| order | u64 | 排序序号 |
| path | string | 部门路径，如 `/1/2/` |
