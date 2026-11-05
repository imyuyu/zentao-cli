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
| `--begin` | Yes | 开始日期 |
| `--end` | Yes | 结束日期 |
| `--products` | Yes | 关联产品 ID（逗号分隔） |
| `--model` | No | 项目模型 |

## Examples

```bash
# Create a project
zentao-cli project create --name "My Project" --code "my-project"

# Create with description
zentao-cli project create --name "Q1 Development" --code "q1-dev" --desc "First quarter development tasks"
```

