# program create

创建新项目集。

## Command

```bash
zentao-cli program create --name <name> --code <code> [--desc <desc>] [--begin <begin>] [--end <end>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--name` | string | Yes | 项目集名称 |
| `--code` | string | Yes | 项目集代号（唯一） |
| `--desc` | string | No | 项目集描述 |
| `--begin` | string | No | 开始日期（YYYY-MM-DD） |
| `--end` | string | No | 结束日期（YYYY-MM-DD） |

## Examples

```bash
# 创建基本项目集
zentao-cli program create --name "产品研发项目集" --code "RD_PROGRAM"

# 创建带详细信息的项目集
zentao-cli program create \
  --name "2024年度产品规划" \
  --code "2024_PLAN" \
  --desc "本年度产品研发项目集" \
  --begin "2024-01-01" \
  --end "2024-12-31"
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 项目集 ID |
| name | string | 项目集名称 |
| code | string | 项目集代号 |
| status | string | 项目集状态 |
| type | string | 项目集类型 |
| desc | string | 项目集描述 |
| begin | string | 开始日期 |
| end | string | 结束日期 |
