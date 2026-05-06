# task delete

删除任务。

## Command
```bash
zentao-cli task delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID to delete |

## Examples

```bash
# Delete a task
zentao-cli task delete 123
```

## Notes

- Only tasks in wait status can be deleted
- This action cannot be undone

