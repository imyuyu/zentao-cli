# task +restart

继续任务。

## Command
```bash
zentao-cli task +restart <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |

## Examples

```bash
# Restart a paused task
zentao-cli task +restart 123
```

## Status Transition

- Changes task status from `wait` to `doing`
