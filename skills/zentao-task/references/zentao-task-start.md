# task start

开始任务。

## Command
```bash
zentao-cli task start <id> --left <left> [--consumed <consumed>] [--assigned-to <assigned_to>] [--real-started <real_started>] [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--left` | Yes | 剩余工时 |
| `--consumed` | No | 已消耗工时 |
| `--assigned-to` | No | 指派人 |
| `--real-started` | No | 实际开始时间 |
| `--comment` | No | 备注 |

## Examples

```bash
# 开始任务
zentao-cli task start 456 --left 8

# 开始任务并记录已消耗工时
zentao-cli task start 456 --left 6 --consumed 2 --comment "开始开发"
```

## Status Transition

- Changes task status from `wait` to `doing`
