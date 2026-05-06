# bug activate

激活 Bug。

## Command
```bash
zentao-cli bug activate <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |

## Examples

```bash
# 激活 Bug
zentao-cli bug activate 5703
```

## API Endpoint

```
POST /api.php/v1/bugs/{bug_id}/activate
```

