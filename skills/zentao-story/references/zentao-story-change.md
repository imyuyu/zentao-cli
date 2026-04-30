# story +change

变更需求状态。

## Command
```bash
zentao-cli story +change <id> --status <status>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Story ID |
| `--status` | Yes | New status (changed) |

## Examples

```bash
# Mark story as changed
zentao-cli story +change 123 --status changed
```

## Valid Status Transitions

| Current Status | Valid Next Status |
|---------------|-------------------|
| draft | active |
| active | changed, closed |
| changed | active, closed |
| closed | active |
