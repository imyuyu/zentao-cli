# release update

修改发布信息。

## Command

```bash
zentao-cli release update <release_id> [--name <name>] [--build <build_id>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `<release_id>` | u64 | Yes | 发布 ID |
| `--name` | string | No | 发布名称 |
| `--build` | u64 | No | 关联的 Build ID |

## Examples

```bash
# 修改发布名称
zentao-cli release update 1 --name "v1.0.1"

# 修改发布关联的 Build
zentao-cli release update 1 --build 2
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
