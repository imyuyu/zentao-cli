# task get

获取任务详情。

## Command
```bash
zentao task get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 任务 ID |

## Examples

```bash
# Get task details
zentao task get 100
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 任务 ID |
| name | string | 任务名称 |
| project | u64 | 所属项目 ID |
| status | string | 任务状态：todo / in progress / done / closed |
| pri | u8 | 优先级：1-5（1 最高） |
| assigned_to | string | 指派人（可选） |
| estimate | f64 | 预估工时（小时）（可选） |
| consumed | f64 | 已消耗工时（小时）（可选） |
| left | f64 | 剩余工时（小时）（可选） |
