# bug delete

删除 Bug。

## Command
```bash
zentao-cli bug delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |

## Examples

```bash
# 删除 Bug
zentao-cli bug delete 5703
```

## API Endpoint

```
DELETE /api.php/v1/bugs/{bug_id}
```

