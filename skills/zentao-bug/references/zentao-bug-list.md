# bug list

列出产品下的缺陷列表，支持分页和多种输出格式。

## Command

```bash
zentao-cli bug list --product <id> [--status <status>] [--assigned-to <user>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | Yes | 产品 ID |
| `--status` | No | 按状态筛选：active / resolved / closed |
| `--assigned-to` | No | 按指派人筛选 |
| `--format` | No | 输出格式：json / pretty / table / ndjson / csv（默认 table） |

## Examples

```bash
# List bugs for product 1
zentao-cli bug list --product 1

# List bugs with JSON output
zentao-cli bug list --product 1 --format json

# List bugs in CSV format (for Excel)
zentao-cli bug list --product 1 --format csv

# List bugs in NDJSON format (for pipeline processing)
zentao-cli bug list --product 1 --format ndjson
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Bug ID |
| title | string | Bug 标题 |
| product | u64 | 所属产品 ID |
| status | string | Bug 状态：active / resolved / closed |
| severity | u8 | 严重程度：1-4（1 最严重） |
| pri | u8 | 优先级：0-5 |
| type | string | Bug 类型：codeerror / interface / design / others（可选） |
| resolution | string | 解决方案：fixed / bydesign / duplicate（可选） |
| steps | string | 重现步骤（可选） |
| project | u64 | 所属项目 ID（可选） |
| story | u64 | 关联的需求 ID（可选） |
| assigned_to | string | 指派人（可选） |
| resolved_by | string | 解决者（可选） |
| resolved_date | string | 解决日期（可选） |

