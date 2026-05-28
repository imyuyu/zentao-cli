# bug activate

激活 Bug。

## Command
```bash
zentao-cli bug activate <id> [--assigned-to <assigned_to>] [--opened-build <opened_build>] [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--assigned-to` | No | 指派给 |
| `--opened-build` | No | 影响版本（多个用逗号分隔） |
| `--comment` | No | 激活备注 |

## Examples

```bash
# 激活 Bug
zentao-cli bug activate 5703

# 激活 Bug 并指派
zentao-cli bug activate 5703 --assigned-to zhangsan

# 激活 Bug 并指定影响版本
zentao-cli bug activate 5703 --opened-build "trunk"
```
