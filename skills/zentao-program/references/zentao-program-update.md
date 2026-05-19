# program update

修改项目集信息。

## Command

```bash
zentao-cli program update <program_id> [--name <name>] [--code <code>] [--desc <desc>] [--begin <begin>] [--end <end>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `<program_id>` | u64 | Yes | 项目集 ID |
| `--name` | string | No | 项目集名称 |
| `--code` | string | No | 项目集代号（唯一） |
| `--desc` | string | No | 项目集描述 |
| `--begin` | string | No | 开始日期（YYYY-MM-DD） |
| `--end` | string | No | 结束日期（YYYY-MM-DD） |

## Examples

```bash
# 修改项目集名称
zentao-cli program update 1 --name "新项目集名称"

# 修改项目集详细信息
zentao-cli program update 1 \
  --name "更新后的项目集" \
  --desc "修改后的描述" \
  --end "2025-12-31"
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
