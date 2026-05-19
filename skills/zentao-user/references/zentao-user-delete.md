# user delete

删除用户。

## Command

```bash
zentao-cli user delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 用户 ID |

## Examples

```bash
# 删除用户
zentao-cli user delete 123
```

## API Endpoint

```
DELETE /api.php/v1/users/{id}
```
