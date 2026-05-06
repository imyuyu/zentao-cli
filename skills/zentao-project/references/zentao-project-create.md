# project create

创建项目。

## Command
```bash
zentao-cli project create --name <name> --code <code> [--desc <description>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Project name |
| `--code` | Yes | Project code |
| `--desc` | No | Project description |

## Examples

```bash
# Create a project
zentao-cli project create --name "My Project" --code "my-project"

# Create with description
zentao-cli project create --name "Q1 Development" --code "q1-dev" --desc "First quarter development tasks"
```

