# ticket create

创建新工单。

## Command

```bash
zentao-cli ticket create --title <title> --product <id> --type <type> [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--title` | Yes | 工单标题 |
| `--product` | Yes | 产品 ID |
| `--type` | Yes | 工单类型 |
| `--desc` | No | 工单描述 |

## Examples

```bash
# 创建工单
zentao-cli ticket create --title "无法登录系统" --product 1 --type "incident"

# 创建带描述的工单
zentao-cli ticket create --title "页面加载缓慢" --product 1 --type "performance" --desc "首页加载时间超过 5 秒"
```

## API Endpoint

```
POST /api.php/v1/tickets
```
