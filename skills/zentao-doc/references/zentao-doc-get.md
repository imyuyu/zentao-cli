# doc get

获取文档详情。

## Command
```bash
zentao-cli doc get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Document ID |

## Examples

```bash
zentao-cli doc get 10
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
| content | string | Document content |
| added_by | string | Creator account |
| added_date | string | Creation date |
| edited_date | string | Last edit date |
| deleted | string | Deletion flag |
