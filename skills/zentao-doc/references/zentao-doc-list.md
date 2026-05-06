# doc list

列出文档库中的文档列表。

## Command
```bash
zentao-cli doc list
```

## Options

无参数（列出所有文档）

## Examples

```bash
# List all documents
zentao-cli doc list
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Document ID |
| title | string | Document title |
| product | u64 | Associated product ID |
| project | u64 | Associated project ID |
| lib | u64 | Document library ID |
| type | string | Document type |
| size | string | Document size |
| added_by | string | Creator account |
| added_date | string | Creation date |
| edited_date | string | Last edit date |
| deleted | string | Deletion flag |

