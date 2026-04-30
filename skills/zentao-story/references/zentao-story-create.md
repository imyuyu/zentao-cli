# story +create

创建新需求。

## Command
```bash
zentao-cli story +create --title <title> --product <id> --pri <priority> [--type <type>] [--estimate <hours>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--title` | Yes | Story title |
| `--product` | Yes | Product ID |
| `--pri` | Yes | Priority (1-4) |
| `--type` | No | Story type (feature, enhance, bugfix, task, story) |
| `--estimate` | No | Estimated hours |
| `--spec` | No | Specification details |

## Examples

```bash
# Basic story creation
zentao-cli story +create --title "用户登录功能" --product 1 --pri 1

# Full story creation
zentao-cli story +create \
  --title "用户注册功能" \
  --product 1 \
  --pri 1 \
  --type feature \
  --estimate 8
```
