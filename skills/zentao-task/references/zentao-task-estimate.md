# task +estimate

记录任务工时。

## Command
```bash
zentao-cli task +estimate <id> --hours <hours> [--consumed <consumed>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--hours` | Yes | Estimated hours |
| `--consumed` | No | Consumed hours |

## Examples

```bash
# Record estimate
zentao-cli task +estimate 123 --hours 8

# Record with consumed hours
zentao-cli task +estimate 123 --hours 8 --consumed 3
```
