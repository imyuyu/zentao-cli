# feedback create

创建新反馈。

## Command

```bash
zentao-cli feedback create --title <title> --product <id> --type <type> [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--title` | Yes | 反馈标题 |
| `--product` | Yes | 产品 ID |
| `--type` | Yes | 反馈类型 |
| `--desc` | No | 反馈描述 |

## Examples

```bash
# 创建反馈
zentao-cli feedback create --title "界面显示问题" --product 1 --type "bug"

# 创建带描述的反馈
zentao-cli feedback create --title "建议增加导出功能" --product 1 --type "improvement" --desc "希望增加数据导出到 Excel 的功能"
```

## API Endpoint

```
POST /api.php/v1/feedbacks
```
