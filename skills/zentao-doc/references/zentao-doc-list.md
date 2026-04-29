# doc list

列出文档库中的文档列表。

## Command
```bash
zentao doc list [--lib <id>] [--product <id>] [--project <id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--lib` | No | Document library ID |
| `--product` | No | Associated product ID |
| `--project` | No | Associated project ID |

## Examples

```bash
# List all documents
zentao doc list

# List documents in a specific library
zentao doc list --lib 1

# List documents for a product
zentao doc list --product 1
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
