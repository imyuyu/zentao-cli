# program delete

删除项目集。

## Command

```bash
zentao-cli program delete <program_id>
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `<program_id>` | u64 | Yes | 项目集 ID |

## Examples

```bash
# 删除项目集
zentao-cli program delete 1
```

## Gotchas

1. **不可逆操作**：删除项目集将同时删除其下的所有项目，**此操作不可恢复**。

2. **权限要求**：需要项目集管理员权限才能删除项目集。

3. **前置检查**：删除前应确保项目集下没有正在进行的项目。
