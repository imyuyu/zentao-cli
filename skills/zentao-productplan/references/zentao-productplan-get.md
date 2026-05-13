# productplan get

获取指定产品计划的详细信息。

## Command

```bash
zentao-cli productplan get <id>
```

## Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| id | u64 | Yes | 产品计划 ID |

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 计划 ID |
| product | u64 | 所属产品 ID |
| name | string | 计划名称 |
| code | string | 计划代号 |
| status | string | 计划状态：wait / doing / done |
| type | string | 计划类型：ship / roadmap |
