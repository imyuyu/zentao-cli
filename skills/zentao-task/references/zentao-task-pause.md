# task pause

暂停任务。

## Command
```bash
zentao-cli task pause <id> [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--comment` | No | 备注 |

## Examples

```bash
# 暂停任务
zentao-cli task pause 456

# 暂停任务并备注
zentao-cli task pause 456 --comment "等待设计确认"
```

## Status Transition

- Changes task status from `doing` to `wait`
