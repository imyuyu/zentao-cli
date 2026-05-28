# task restart

继续一个已暂停的任务。

## Command
```bash
zentao-cli task restart <id> --left <left> [--consumed <consumed>] [--assigned-to <assigned_to>] [--real-started <real_started>] [--comment <comment>]
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
# 继续任务
zentao-cli task restart 456 --left 4

# 继续任务并更新指派人
zentao-cli task restart 456 --left 4 --assigned-to developer
```

## Status Transition

- Changes task status from `wait` to `doing`
