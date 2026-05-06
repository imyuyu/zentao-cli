# bug get

获取缺陷详情。

## Command
```bash
zentao-cli bug get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Bug ID |

## Examples

```bash
# Get bug details
zentao-cli bug get 123
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Bug ID |
| title | string | Bug 标题 |
| description | string | Bug 描述（可选） |
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

