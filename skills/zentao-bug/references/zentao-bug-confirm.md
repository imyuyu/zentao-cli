# bug confirm

确认 Bug（可同时指派、修改类型、优先级等）。

## Command
```bash
zentao-cli bug confirm <id> [--assigned-to <user>] [--type <type>] [--pri <priority>] [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--assigned-to` | No | 指派给用户 |
| `--type` | No | Bug 类型 (codeerror/config/install/security/performance/standard/automation/designdefect/others) |
| `--pri` | No | 优先级 (0-5) |
| `--comment` | No | 备注 |

## Examples

```bash
# 确认 Bug
zentao-cli bug confirm 5703

# 确认并指派
zentao-cli bug confirm 5703 --assigned-to developer-name

# 确认并修改优先级
zentao-cli bug confirm 5703 --pri 1 --comment "确认高优先级"
```

## API Endpoint

```
POST /api.php/v1/bugs/{bug_id}/confirm
```

