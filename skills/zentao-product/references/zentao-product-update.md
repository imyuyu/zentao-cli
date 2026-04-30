# product +update

更新产品信息。

## Command
```bash
zentao-cli product +update <id> [--name <name>] [--code <code>] [--status <status>] [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Product ID |
| `--name` | No | New product name |
| `--code` | No | New product code |
| `--status` | No | New status (normal, closed) |
| `--desc` | No | New product description |

## Examples

```bash
# Update product name
zentao-cli product +update 1 --name "New Product Name"

# Close a product
zentao-cli product +update 1 --status closed
```
