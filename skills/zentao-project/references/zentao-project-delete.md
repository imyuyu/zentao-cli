# project delete

删除项目。

## Command
```bash
zentao-cli project delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Project ID to delete |

## Examples

```bash
# Delete a project
zentao-cli project delete 1
```

## Notes

- Only projects in wait status can be deleted
- This action cannot be undone

