# release create

创建新发布。

## Command

```bash
zentao-cli release create --product <product_id> --name <name> [--build <build_id>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--product` | u64 | Yes | 产品 ID |
| `--name` | string | Yes | 发布名称（如 v1.0.0） |
| `--build` | u64 | No | 关联的 Build ID |

## Examples

```bash
# 创建基本发布
zentao-cli release create --product 1 --name "v1.0.0"

# 创建带 Build 的发布
zentao-cli release create --product 1 --name "v1.0.0-beta" --build 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 发布 ID |
| name | string | 发布名称（如 v1.0.0） |
| product | u64 | 产品 ID |
| build | u64 | 关联的 Build ID |
| status | string | 发布状态：normal / closed |
| marker | string | 发布标记（如 stable、beta） |
| date | string | 发布日期 |
