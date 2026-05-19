# task create

创建新任务。

## Command
```bash
zentao-cli task create --name <name> --project <id> --assigned-to <user> --type <type> --est-started <date> --deadline <date> [--pri <pri>] [--estimate <estimate>] [--module <module>] [--story <story>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Task name |
| `--project` | Yes | Project ID |
| `--assigned-to` | Yes | Assign to user |
| `--type` | Yes | Task type |
| `--est-started` | Yes | Estimated start date |
| `--deadline` | Yes | Deadline |
| `--pri` | No | Priority (1-4) |
| `--estimate` | No | Estimated hours |
| `--module` | No | Module ID |
| `--story` | No | Related story ID |

## Examples

```bash
# Basic task creation
zentao-cli task create --name "实现用户注册接口" --project 1 --assigned-to developer --type task --est-started "2026-05-20" --deadline "2026-05-25"

# Full task creation
zentao-cli task create \
  --name "代码评审" \
  --project 1 \
  --assigned-to developer-name \
  --type task \
  --est-started "2026-05-20" \
  --deadline "2026-05-25" \
  --pri 2 \
  --estimate 4
```

