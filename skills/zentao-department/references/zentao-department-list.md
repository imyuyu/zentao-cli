# department list

列出所有部门列表。

## Command

```bash
zentao-cli department list
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 部门 ID |
| name | string | 部门名称 |
| parent | u64 | 父部门 ID（0 表示根部门） |
| order | u64 | 排序序号 |
| path | string | 部门路径，如 `/1/2/` |
