# product create

创建产品。

## Command
```bash
zentao-cli product create --name <name> --code <code> [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Product name |
| `--code` | Yes | Product code (English identifier) |
| `--desc` | No | Product description |
| `--program` | No | 所属项目集 |
| `--line` | No | 产品线 |
| `--po` | No | 产品负责人 |
| `--qd` | No | 测试负责人 |
| `--rd` | No | 研发负责人 |
| `--type` | No | 产品类型 |
| `--acl` | No | 访问控制 |
| `--whitelist` | No | 白名单 |

## Examples

```bash
# Create a product
zentao-cli product create --name "My Product" --code "my-product"

# Create with description
zentao-cli product create --name "Enterprise ERP" --code "erp" --desc "Enterprise resource planning system"
```

