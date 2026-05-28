# bug close

关闭 Bug。

## Command
```bash
zentao-cli bug close <id> [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--comment` | No | 关闭备注 |

## Examples

```bash
# 关闭 Bug
zentao-cli bug close 5703

# 关闭 Bug 并添加备注
zentao-cli bug close 5703 --comment "已修复并验证"
```
