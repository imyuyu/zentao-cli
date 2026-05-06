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

## API Endpoint

```
GET /api.php/v1/projects/{projectId}/executions
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 执行 ID |
| name | string | 执行名称 |
| project | u64 | 所属项目 ID |
| status | string | 执行状态：wait / doing / suspended / closed |
| type | string | 执行类型：iteration（迭代）/ milestone（里程碑） |
| begin | string | 开始日期（可选） |
| end | string | 结束日期（可选） |
| days | u32 | 持续天数（可选） |
| desc | string | 执行描述（可选） |
| opened_by | string | 创建人（可选） |
| opened_date | string | 创建时间（可选） |

