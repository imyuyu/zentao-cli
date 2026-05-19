# bug update

更新缺陷信息。注意：状态变更（status）、解决方案（resolution）、指派（assignedTo）需要使用 `bug resolve` 或 `bug confirm` 命令。

## Command
```bash
zentao-cli bug update <id> [--title <title>] [--keywords <keywords>] [--severity <severity>] [--pri <pri>] [--type <type>] [--os <os>] [--browser <browser>] [--steps <steps>] [--task <task>] [--story <story>] [--deadline <deadline>] [--opened-build <opened_build>] [--branch <branch>] [--module <module>] [--execution <execution>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--title` | No | New title |
| `--keywords` | No | Keywords |
| `--severity` | No | Severity level (1-5) |
| `--pri` | No | Priority (0-5) |
| `--type` | No | Bug type (codeerror/interface/design/others) |
| `--os` | No | Operating system |
| `--browser` | No | Browser |
| `--steps` | No | Steps to reproduce |
| `--task` | No | Related task ID |
| `--story` | No | Related story ID |
| `--deadline` | No | Deadline |
| `--opened-build` | No | Affected build |
| `--branch` | No | Branch ID |
| `--module` | No | Module ID |
| `--execution` | No | Execution ID |

## Examples

```bash
# Update bug title and severity
zentao-cli bug update 123 --title "新标题" --severity 2
```

**注意**：指派 Bug 使用 `bug confirm --assigned-to`，解决 Bug 使用 `bug resolve`。