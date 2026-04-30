# project +update

更新项目信息。

## Command
```bash
zentao-cli project +update <id> [--name <name>] [--code <code>] [--status <status>] [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Project ID |
| `--name` | No | New project name |
| `--code` | No | New project code |
| `--status` | No | New status (wait, doing, closed, suspended) |
| `--desc` | No | New project description |

## Examples

```bash
# Update project name
zentao-cli project +update 1 --name "New Project Name"

# Close a project
zentao-cli project +update 1 --status closed

# Update multiple fields
zentao-cli project +update 1 --name "Updated" --desc "New description"
```
