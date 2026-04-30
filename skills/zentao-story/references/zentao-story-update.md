# story update

更新需求信息。

## Command
```bash
zentao-cli story update <id> [--title <title>] [--status <status>] [--pri <priority>] [--assigned-to <user>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Story ID |
| `--title` | No | New title |
| `--status` | No | New status (draft, active, closed, changed) |
| `--pri` | No | New priority (1-4) |
| `--assigned-to` | No | Assign to user |

## Examples

```bash
# Close a story
zentao-cli story update 123 --status closed

# Update priority
zentao-cli story update 123 --pri 2

# Change status and assignee
zentao-cli story update 123 --status changed --assigned-to developer-name
```

## Valid Status Transitions

| Current Status | Valid Next Status |
|---------------|-------------------|
| draft | active |
| active | changed, closed |
| changed | active, closed |
| closed | active |
