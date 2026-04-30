# product +delete

删除产品。

## Command
```bash
zentao-cli product +delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Product ID to delete |

## Examples

```bash
# Delete a product
zentao-cli product +delete 1
```

## Notes

- Only products in normal status can be deleted
- This action cannot be undone
