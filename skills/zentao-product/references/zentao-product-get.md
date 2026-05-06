# product get

获取产品详情。

## Command
```bash
zentao-cli product get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 产品 ID |

## Examples

```bash
# Get product details
zentao-cli product get 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 产品 ID |
| name | string | 产品名称 |
| code | string | 产品代号（英文标识） |
| status | string | 产品状态：normal（正常）/ closed（关闭） |
| desc | string | 产品描述（可选） |

