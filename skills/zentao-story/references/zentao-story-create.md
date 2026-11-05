# story create

创建新需求。

## Command
```bash
zentao-cli story create --title <title> --product <id> --pri <priority> [--category <category>] [--spec <spec>] [--estimate <estimate>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--title` | Yes | Story title |
| `--product` | Yes | Product ID |
| `--pri` | Yes | Priority (1-4) |
| `--category` | No | Category (feature/requirement/bug/improvement) |
| `--spec` | No | Specification details |
| `--estimate` | No | Estimated hours |
| `--module` | No | 所属模块 ID |

## Examples

```bash
# Basic story creation
zentao-cli story create --title "用户登录功能" --product 1 --pri 1

# Full story creation
zentao-cli story create \
  --title "用户注册功能" \
  --product 1 \
  --pri 1 \
  --category feature \
  --estimate 8
```

