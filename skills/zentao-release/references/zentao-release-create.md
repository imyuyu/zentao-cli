# release create

创建新发布。

> **必须带 `--project`**：禅道发布按项目维度创建（`POST /projects/{id}/releases`），
> 缺了会返回 200 + 空 body 但不落库。

## Command

```bash
zentao-cli release create --product <product_id> --project <project_id> --name <name> \
  [--build <build_id>] [--date <date>] [--desc <desc>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--product` | u64 | Yes | 产品 ID |
| `--project` | u64 | Yes | 项目 ID（缺失会静默失败） |
| `--name` | string | Yes | 发布名称（如 v1.0.0） |
| `--build` | u64 | No | 关联的 Build ID |
| `--date` | string | No | 发布日期（YYYY-MM-DD） |
| `--desc` | string | No | 描述 |

## Examples

```bash
# 创建基本发布
zentao-cli release create --product 1 --project 3 --name "v1.0.0"

# 创建带 Build 的发布
zentao-cli release create --product 1 --project 3 --name "v1.0.0-beta" \
  --build 1 --date 2026-11-05
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 发布 ID |
| name | string | 发布名称（如 v1.0.0） |
| product | u64 | 产品 ID |
| project | u64 | 项目 ID |
| build | u64 | 关联的 Build ID |
| status | string | 发布状态：normal / closed |
| marker | string | 发布标记（如 stable、beta） |
| date | string | 发布日期 |
