# task +list

列出项目下的任务列表。

## Command
```bash
zentao-cli task +list --project <id> [--assigned-to <user>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--project` | Yes | 项目 ID |
| `--assigned-to` | No | 按指派人筛选 |

## Examples

```bash
# List all tasks for project 1
zentao-cli task +list --project 1

# List tasks assigned to a specific user
zentao-cli task +list --project 1 --assigned-to developer-name
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
