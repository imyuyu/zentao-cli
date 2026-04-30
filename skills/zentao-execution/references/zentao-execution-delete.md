# execution +delete

删除执行。

## Command
```bash
zentao-cli execution +delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Execution ID to delete |

## Examples

```bash
# Delete an execution
zentao-cli execution +delete 100
```

## Notes

- Only executions in wait status can be deleted
- This action cannot be undone
