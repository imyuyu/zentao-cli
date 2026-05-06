# product list

列出所有有权限访问的产品列表，支持分页和多种输出格式。

## Command

```bash
zentao-cli product list
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 产品 ID |
| name | string | 产品名称 |
| code | string | 产品代号（英文标识） |
| status | string | 产品状态：normal（正常）/ closed（关闭） |
| desc | string | 产品描述（可选） |

