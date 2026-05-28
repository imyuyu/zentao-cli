# execution list

列出执行（迭代/里程碑）列表。

## Command
```bash
zentao-cli execution list [--project <id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--project` | No | 按项目 ID 筛选执行列表（使用 `/projects/{id}/executions` 接口） |

## Examples

```bash
# List executions for a specific project (推荐)
zentao-cli execution list --project 1

# List all executions (可能需要较大权限)
zentao-cli execution list
```
