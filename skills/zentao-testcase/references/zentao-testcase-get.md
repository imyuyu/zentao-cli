# testcase get

获取单个测试用例的详细信息。

## Command
```bash
zentao testcase get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Testcase ID |

## Examples

```bash
# Get testcase details
zentao testcase get 123
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | Testcase ID |
| title | string | Testcase title |
| type | string | Testcase type |
| severity | u8 | Severity level (1-4) |
| pri | u8 | Priority (0-5) |
| status | string | Testcase status |
| steps | string | Test steps |
| expectation | string | Expected result |
| product | u64 | Product ID |
| project | u64 | Project ID |
| openedBy | string | Creator |
| version | u64 | Version number |
