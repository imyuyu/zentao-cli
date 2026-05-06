# bug resolve

解决 Bug。

## Command
```bash
zentao-cli bug resolve <id> --resolution <resolution> --resolved-build <build>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |
| `--resolution` | Yes | 解决方案 |
| `--resolved-build` | Yes | 解决版本（版本 ID 或 "trunk"） |

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

# 解决 Bug（使用版本 ID）
zentao-cli bug resolve 5703 --resolution fixed --resolved-build 1

# 设计如此
zentao-cli bug resolve 5703 --resolution bydesign --resolved-build trunk
```

## API Endpoint

```
POST /api.php/v1/bugs/{bug_id}/resolve
```

