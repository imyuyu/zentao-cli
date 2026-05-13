# feedback get

获取指定反馈的详细信息。

## Command

```bash
zentao-cli feedback get <id>
```

## Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| id | u64 | Yes | 反馈 ID |

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 反馈 ID |
| title | string | 反馈标题 |
| type | string | 反馈类型 |
| status | string | 反馈状态：open / assigned / closed |
| pri | u8 | 优先级 |
| desc | string | 反馈描述 |
