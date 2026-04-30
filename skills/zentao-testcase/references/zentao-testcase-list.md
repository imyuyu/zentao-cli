# testcase list

列出某个产品或项目下的测试用例列表。

## Command
```bash
zentao-cli testcase list --product <id> [--project <id>] [--type <type>] [--status <status>] [--severity <level>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | No | Product ID |
| `--project` | No | Project ID |
| `--type` | No | Filter by type: feature/performance/interface/security/concurrency/destructive/install/others |
| `--status` | No | Filter by status: wait/normal/blocked/bypass |
| `--severity` | No | Filter by severity (1-4) |

## Examples

```bash
# List all testcases for product 1
zentao-cli testcase list --product 1

# List normal status testcases
zentao-cli testcase list --product 1 --status normal

# List testcases by project
zentao-cli testcase list --product 1 --project 5

# List critical severity testcases
zentao-cli testcase list --product 1 --severity 1
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Testcase ID |
| title | string | Testcase title |
| product | u64 | Product ID |
| project | u64 | Project ID |
| type | string | Testcase type |
| status | string | Testcase status |
| severity | u8 | Severity level (1-4) |
| pri | u8 | Priority (0-5) |
| openedBy | string | Creator |
| version | u64 | Version number |
