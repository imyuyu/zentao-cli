# ticket get

获取指定工单的详细信息。

## Command

```bash
zentao-cli ticket get <id>
```

## Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| id | u64 | Yes | 工单 ID |

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 工单 ID |
| title | string | 工单标题 |
| type | string | 工单类型 |
| status | string | 工单状态 |
| pri | u8 | 优先级 |
| severity | u8 | 严重程度 |
