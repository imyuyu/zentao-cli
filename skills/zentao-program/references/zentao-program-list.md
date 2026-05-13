# program list

列出所有项目集列表。

## Command

```bash
zentao-cli program list
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 项目集 ID |
| name | string | 项目集名称 |
| code | string | 项目集代号 |
| status | string | 项目集状态：doing / wait / closed |
| type | string | 项目集类型 |
| desc | string | 项目集描述 |
| parent | u64 | 父项目集 ID |
