# testtask list

列出测试单列表。

## Command

```bash
zentao-cli testtask list [--project <id>]
```

## Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| --project | u64 | Yes | 项目 ID |

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 测试单 ID |
| name | string | 测试单名称 |
| project | u64 | 所属项目 ID |
| execution | u64 | 所属执行/迭代 ID |
| status | string | 测试单状态：wait / doing / done / closed |
| type | string | 测试单类型 |
| product | u64 | 所属产品 ID |
