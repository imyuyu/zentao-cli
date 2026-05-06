# bug close

关闭 Bug。

## Command
```bash
zentao-cli bug close <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |

## Examples

```bash
# 关闭 Bug
zentao-cli bug close 5703
```

## API Endpoint

```
POST /api.php/v1/bugs/{bug_id}/close
```

