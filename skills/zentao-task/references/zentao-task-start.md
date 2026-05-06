# task start

开始任务。

## Command
```bash
zentao-cli task start <id> [--estimate <hours>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--estimate` | No | Estimated hours to complete |

## Examples

```bash
# Start a task
zentao-cli task start 123

# Start with estimate
zentao-cli task start 123 --estimate 4
```

## Status Transition

- Changes task status from `wait` to `doing`

