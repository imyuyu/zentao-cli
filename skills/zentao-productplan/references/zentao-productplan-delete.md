# productplan delete

删除产品计划。

## Command

```bash
zentao-cli productplan delete <plan_id>
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `<plan_id>` | u64 | Yes | 计划 ID |

## Examples

```bash
# 删除产品计划
zentao-cli productplan delete 1
```

## Gotchas

1. **不可逆操作**：删除产品计划将同时删除计划下的所有关联数据，**此操作不可恢复**。

2. **权限要求**：需要产品管理员权限才能删除产品计划。

3. **前置检查**：删除前应确保计划下没有正在进行的任务或需求。
