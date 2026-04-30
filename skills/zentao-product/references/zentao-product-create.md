# product +create

创建产品。

## Command
```bash
zentao-cli product +create --name <name> --code <code> [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Product name |
| `--code` | Yes | Product code (English identifier) |
| `--desc` | No | Product description |

## Examples

```bash
# Create a product
zentao-cli product +create --name "My Product" --code "my-product"

# Create with description
zentao-cli product +create --name "Enterprise ERP" --code "erp" --desc "Enterprise resource planning system"
```
