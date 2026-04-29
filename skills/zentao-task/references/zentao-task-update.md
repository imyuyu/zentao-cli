# task update

更新任务信息。

## Command
```bash
zentao task update <id> [--name <name>] [--status <status>] [--pri <priority>] [--assigned-to <user>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--name` | No | New task name |
| `--status` | No | New status (wait, doing, done, closed) |
| `--pri` | No | New priority (1-4) |
| `--assigned-to` | No | Assign to user |

## Examples

```bash
# Mark task as done
zentao task update 456 --status done

# Start working on task
zentao task update 456 --status doing

# Reassign task
zentao task update 456 --assigned-to another-developer
```
