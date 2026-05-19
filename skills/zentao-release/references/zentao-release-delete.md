# release delete

删除发布。

## Command

```bash
zentao-cli release delete <release_id>
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `<release_id>` | u64 | Yes | 发布 ID |

## Examples

```bash
# 删除发布
zentao-cli release delete 1
```

## Gotchas

1. **不可逆操作**：删除发布将同时删除发布的历史记录，**此操作不可恢复**。

2. **权限要求**：需要产品管理员权限才能删除发布。

3. **前置检查**：删除前应确认没有重要的发布记录需要保留。
