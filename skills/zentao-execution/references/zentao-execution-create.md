# execution +create

创建执行。

## Command
```bash
zentao-cli execution +create --name <name> --project <id> --type <type> [--begin <date>] [--end <date>] [--days <days>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Execution name |
| `--project` | Yes | Project ID |
| `--type` | Yes | Execution type (iteration, milestone) |
| `--begin` | No | Start date (YYYY-MM-DD) |
| `--end` | No | End date (YYYY-MM-DD) |
| `--days` | No | Duration in days |

## Examples

```bash
# Create an iteration
zentao-cli execution +create --name "Sprint 1" --project 1 --type iteration

# Create with dates
zentao-cli execution +create --name "Q1 Milestone" --project 1 --type milestone --begin 2024-01-01 --end 2024-03-31
```
