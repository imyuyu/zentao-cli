# feedback delete

删除反馈。

## Command

```bash
zentao-cli feedback delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 反馈 ID |

## Examples

```bash
# 删除反馈
zentao-cli feedback delete 1
```

## API Endpoint

```
DELETE /api.php/v1/feedbacks/{id}
```
