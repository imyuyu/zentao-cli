# task pause

暂停任务。

## Command
```bash
zentao-cli task pause <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |

## Examples

```bash
# Pause a task
zentao-cli task pause 123
```

## Status Transition

- Changes task status from `doing` to `wait`

