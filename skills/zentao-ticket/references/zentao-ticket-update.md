# ticket update

修改工单信息。

## Command

```bash
zentao-cli ticket update <id> [--title <title>] [--type <type>] [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 工单 ID |
| `--title` | No | 工单标题 |
| `--type` | No | 工单类型 |
| `--desc` | No | 工单描述 |

## Examples

```bash
# 更新工单标题
zentao-cli ticket update 1 --title "新标题"

# 更新工单类型和描述
zentao-cli ticket update 1 --type "improvement" --desc "更新后的描述"
```

## API Endpoint

```
PUT /api.php/v1/tickets/{id}
```
