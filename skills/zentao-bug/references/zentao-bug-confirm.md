# bug confirm

确认 Bug。

## Command
```bash
zentao-cli bug confirm <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |

## Examples

```bash
# 确认 Bug
zentao-cli bug confirm 5703
```

## API Endpoint

```
POST /api.php/v1/bugs/{bug_id}/confirm
```

