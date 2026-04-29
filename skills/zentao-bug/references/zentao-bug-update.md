# bug update

更新缺陷信息。

## Command
```bash
zentao bug update <id> [--title <title>] [--status <status>] [--resolution <resolution>] [--assigned-to <user>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--title` | No | New title |
| `--status` | No | New status (active, resolved, closed) |
| `--resolution` | No | Resolution (fixed, duplicate, notrepro, wonfix, bysdesign) |
| `--pri` | No | New priority (1-4) |
| `--assigned-to` | No | Assign to user |

## Examples

```bash
# Resolve a bug
zentao bug update 123 --status resolved --resolution fixed

# Assign bug
zentao bug update 123 --assigned-to developer-name

# Close a bug
zentao bug update 123 --status closed
```
