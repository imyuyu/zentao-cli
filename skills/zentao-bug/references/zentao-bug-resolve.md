# bug resolve

解决 Bug。

## Command
```bash
zentao-cli bug resolve <id> --resolution <resolution> --resolved-build <build> [--assigned-to <user>] [--duplicate-bug <id>] [--resolved-date <date>] [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--resolution` | Yes | 解决方案 |
| `--resolved-build` | Yes | 解决版本（版本 ID 或 "trunk"） |
| `--assigned-to` | No | 指派给用户 |
| `--duplicate-bug` | No | 重复Bug ID（当 resolution=duplicate 时使用） |
| `--resolved-date` | No | 解决日期 |
| `--comment` | No | 备注 |

## Resolution 值

| 值 | 说明 |
|----|------|
| `fixed` | 已解决 |
| `bydesign` | 设计如此 |
| `duplicate` | 重复bug |
| `external` | 外部原因 |
| `notrepro` | 无法重现 |
| `postponed` | 延期处理 |
| `willnotfix` | 不予解决 |
| `tostory` | 转需求 |

## Examples

```bash
# 解决 Bug（使用 trunk 主干）
zentao-cli bug resolve 5703 --resolution fixed --resolved-build trunk

# 解决 Bug 并指派
zentao-cli bug resolve 5703 --resolution fixed --resolved-build trunk --assigned-to developer-name

# 重复 Bug
zentao-cli bug resolve 5703 --resolution duplicate --resolved-build trunk --duplicate-bug 1234
```
