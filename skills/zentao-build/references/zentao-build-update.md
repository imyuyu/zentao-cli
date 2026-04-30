# build +update

更新版本信息。

## Command
```bash
zentao-cli build +update <id> [--name <name>] [--status <status>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Build ID |
| `--name` | No | New build name |
| `--status` | No | New status |

## Examples

```bash
# Update build name
zentao-cli build +update 1 --name "v1.0.1"

# Update build status
zentao-cli build +update 1 --status released
```
