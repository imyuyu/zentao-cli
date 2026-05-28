# task finish

完成任务。

## Command
```bash
zentao-cli task finish <id> --current-consumed <current_consumed> --finished-date <finished_date> [--assigned-to <assigned_to>] [--real-started <real_started>] [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--current-consumed` | Yes | 当前消耗工时 |
| `--finished-date` | Yes | 完成日期 |
| `--assigned-to` | No | 指派人 |
| `--real-started` | No | 实际开始时间 |
| `--comment` | No | 备注 |

## Examples

```bash
# 完成任务
zentao-cli task finish 456 --current-consumed 8 --finished-date 2024-01-15

# 完成任务并备注
zentao-cli task finish 456 --current-consumed 8 --finished-date 2024-01-15 --comment "已完成代码评审"
```

## Status Transition

- Changes task status from `doing` to `done`
