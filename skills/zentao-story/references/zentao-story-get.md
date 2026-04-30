# story +get

获取需求详情。

## Command
```bash
zentao-cli story +get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 需求 ID |

## Examples

```bash
# Get story details
zentao-cli story +get 123
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 需求 ID |
| title | string | 需求标题 |
| description | string | 需求描述/详细说明（可选） |
| product | u64 | 产品 ID |
| status | string | 需求状态：draft / active / closed / changed |
| pri | u8 | 优先级：0-5 |
| category | string | 需求类别（可选）：feature / requirement / bug / improvement |
| stage | string | 当前阶段（可选）：wait / plan / developed / 测试中 / released / closed |
| module | u64 | 所属模块 ID（可选） |
| assigned_to | string | 指派人（可选） |
| opened_by | string | 创建者（可选） |
| estimate | f64 | 预估工时（小时）（可选） |
| version | u64 | 版本号（可选，用于乐观锁） |
