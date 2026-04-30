# task +finish

完成任务。

## Command
```bash
zentao-cli task +finish <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |

## Examples

```bash
# Finish a task
zentao-cli task +finish 123
```

## Status Transition

- Changes task status from `doing` to `done`
