# story +close

关闭需求。

## Command
```bash
zentao-cli story +close <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Story ID |

## Examples

```bash
# Close a story
zentao-cli story +close 123
```

## Valid Status Transitions

| Current Status | Valid Next Status |
|---------------|-------------------|
| draft | active |
| active | changed, closed |
| changed | active, closed |
| closed | active |
