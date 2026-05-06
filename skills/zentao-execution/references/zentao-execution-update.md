# execution update

更新执行信息。

## Command
```bash
zentao-cli execution update <id> [--name <name>] [--status <status>] [--begin <date>] [--end <date>] [--days <days>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Execution ID |
| `--name` | No | New execution name |
| `--status` | No | New status (wait, doing, suspended, closed) |
| `--begin` | No | New start date (YYYY-MM-DD) |
| `--end` | No | New end date (YYYY-MM-DD) |
| `--days` | No | New duration in days |

## Examples

```bash
# Update execution name
zentao-cli execution update 100 --name "Sprint 2"

# Update status
zentao-cli execution update 100 --status doing

# Update dates
zentao-cli execution update 100 --begin 2024-02-01 --end 2024-02-28
```

