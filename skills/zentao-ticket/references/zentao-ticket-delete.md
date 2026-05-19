# ticket delete

删除工单。

## Command

```bash
zentao-cli ticket delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 工单 ID |

## Examples

```bash
# 删除工单
zentao-cli ticket delete 1
```

## API Endpoint

```
DELETE /api.php/v1/tickets/{id}
```
