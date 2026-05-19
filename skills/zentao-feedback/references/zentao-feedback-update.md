# feedback update

修改反馈信息。

## Command

```bash
zentao-cli feedback update <id> [--title <title>] [--type <type>] [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 反馈 ID |
| `--title` | No | 反馈标题 |
| `--type` | No | 反馈类型 |
| `--desc` | No | 反馈描述 |

## Examples

```bash
# 更新反馈标题
zentao-cli feedback update 1 --title "新标题"

# 更新反馈类型和描述
zentao-cli feedback update 1 --type "improvement" --desc "更新后的描述"
```

## API Endpoint

```
PUT /api.php/v1/feedbacks/{id}
```
