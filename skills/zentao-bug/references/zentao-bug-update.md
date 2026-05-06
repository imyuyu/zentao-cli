# bug update

更新缺陷信息。

## Command
```bash
zentao-cli bug update <id> [--title <title>] [--status <status>] [--resolution <resolution>] [--resolved-build <id>] [--assigned-to <user>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--title` | No | New title |
| `--status` | No | New status (active, resolved, closed) |
| `--resolution` | No | Resolution (fixed, duplicate, notrepro, wonfix, bydesign) |
| `--resolved-build` | No | Resolved build/version ID (required when status=resolved) |
| `--assigned-to` | No | Assign to user |

## Examples

```bash
# Resolve a bug (requires --resolved-build)
zentao-cli bug update 123 --status resolved --resolution fixed --resolved-build 1

# Assign bug
zentao-cli bug update 123 --assigned-to developer-name

# Close a bug
zentao-cli bug update 123 --status closed
```

