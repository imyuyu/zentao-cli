# task estimate

记录任务工时。

## Command
```bash
zentao-cli task estimate <id> --consumed <consumed> --left <left> [--notes <notes>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--consumed` | Yes | Consumed hours |
| `--left` | Yes | Remaining hours |
| `--notes` | No | Estimate notes |

## Examples

```bash
# Record estimate
zentao-cli task estimate 123 --consumed 3 --left 5

# Record estimate with notes
zentao-cli task estimate 123 --consumed 8 --left 0 --notes "Finished implementation"
```

