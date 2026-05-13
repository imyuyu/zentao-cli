# feedback list

列出反馈列表。

## Command

```bash
zentao-cli feedback list
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 反馈 ID |
| title | string | 反馈标题 |
| type | string | 反馈类型 |
| status | string | 反馈状态：open / assigned / closed |
| pri | u8 | 优先级 |
| desc | string | 反馈描述 |
