# story delete

删除需求。

## Command
```bash
zentao-cli story delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Story ID to delete |

## Examples

```bash
# Delete a story
zentao-cli story delete 123
```

## Notes

- Only stories in draft status can be deleted
- This action cannot be undone

