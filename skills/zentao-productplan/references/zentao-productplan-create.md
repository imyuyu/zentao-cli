# productplan create

创建新产品计划。

## Command

```bash
zentao-cli productplan create --product <product_id> --title <title> [--code <code>] [--desc <desc>] [--begin <begin>] [--end <end>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--product` | u64 | Yes | 产品 ID |
| `--title` | string | Yes | 计划标题 |
| `--code` | string | No | 计划代号 |
| `--desc` | string | No | 计划描述 |
| `--begin` | string | No | 开始日期（YYYY-MM-DD） |
| `--end` | string | No | 结束日期（YYYY-MM-DD） |

## Examples

```bash
# 创建基本产品计划
zentao-cli productplan create --product 1 --title "Q1发布计划"

# 创建带详细信息的计划
zentao-cli productplan create \
  --product 1 \
  --title "2024年度规划" \
  --code "2024_PLAN" \
  --desc "本年度产品路线图" \
  --begin "2024-01-01" \
  --end "2024-12-31"
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
