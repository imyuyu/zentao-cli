# bug update

更新缺陷信息。注意：状态变更（status）、解决方案（resolution）需要使用 `bug resolve` 命令。

## Command
```bash
zentao-cli bug update <id> [--title <title>] [--keywords <keywords>] [--severity <severity>] [--pri <pri>] [--type <type>] [--os <os>] [--browser <browser>] [--steps <steps>] [--task <task>] [--story <story>] [--deadline <deadline>] [--opened-build <opened_build>] [--branch <branch>] [--module <module>] [--execution <execution>] [--assigned-to <user>]
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
| `--assigned-to` | No | Assign to user (通过 confirm 接口) |

## Examples

```bash
# Update bug title and severity
zentao-cli bug update 123 --title "新标题" --severity 2

# 指派 Bug 给开发人员
zentao-cli bug update 123 --assigned-to developer-name
```

**注意**：要解决 Bug，请使用 `bug resolve` 命令。