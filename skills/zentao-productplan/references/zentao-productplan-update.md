# productplan update

修改产品计划信息。

## Command

```bash
zentao-cli productplan update <plan_id> [--title <title>] [--code <code>] [--desc <desc>] [--begin <begin>] [--end <end>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `<plan_id>` | u64 | Yes | 计划 ID |
| `--title` | string | No | 计划标题 |
| `--code` | string | No | 计划代号 |
| `--desc` | string | No | 计划描述 |
| `--begin` | string | No | 开始日期（YYYY-MM-DD） |
| `--end` | string | No | 结束日期（YYYY-MM-DD） |

## Examples

```bash
# 修改计划标题
zentao-cli productplan update 1 --title "新计划标题"

# 修改计划详细信息
zentao-cli productplan update 1 \
  --title "更新后的计划" \
  --desc "修改后的描述" \
  --end "2025-06-30"
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 计划 ID |
| product | u64 | 所属产品 ID |
| name | string | 计划名称 |
| code | string | 计划代号 |
| status | string | 计划状态：wait / doing / done |
| type | string | 计划类型：ship / roadmap |
| begin | string | 开始日期 |
| end | string | 结束日期 |
