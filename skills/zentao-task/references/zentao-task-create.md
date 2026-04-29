# task create

创建新任务。

## Command
```bash
zentao task create --name <name> --project <id> --pri <priority> [--assigned-to <user>] [--estimate <hours>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Task name |
| `--project` | Yes | Project ID |
| `--pri` | Yes | Priority (1-4) |
| `--type` | No | Task type |
| `--assigned-to` | No | Assign to user |
| `--estimate` | No | Estimated hours |

## Examples

```bash
# Basic task creation
zentao task create --name "实现用户注册接口" --project 1 --pri 2

# Full task creation
zentao task create \
  --name "代码评审" \
  --project 1 \
  --pri 2 \
  --assigned-to developer-name \
  --estimate 4
```
