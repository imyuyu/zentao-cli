# task +close

关闭任务。

## Command
```bash
zentao-cli task +close <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |

## Examples

```bash
# Close a task
zentao-cli task +close 123
```

## Status Transition

- Changes task status from `done` to `closed`
