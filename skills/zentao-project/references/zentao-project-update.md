# project update

更新项目信息。

## Command
```bash
zentao-cli project update <id> [--name <name>] [--status <status>] [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Project ID |
| `--name` | No | New project name |
| `--status` | No | New status (wait, doing, closed, suspended) |
| `--desc` | No | New project description |
| `--pm` | No | 项目经理 |
| `--budget` | No | 预算 |
| `--budget-unit` | No | 币种 |
| `--days` | No | 工作日 |
| `--acl` | No | 访问控制 |

## Examples

```bash
# Update project name
zentao-cli project update 1 --name "New Project Name"

# Close a project
zentao-cli project update 1 --status closed

# Update multiple fields
zentao-cli project update 1 --name "Updated" --desc "New description"
```

