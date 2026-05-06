# testcase update

更新测试用例信息。

## Command
```bash
zentao-cli testcase update <id> [--title <title>] [--status <status>] [--pri <priority>] [--severity <level>] [--type <type>] [--steps <steps>] [--expectation <expectation>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Testcase ID |
| `--title` | No | New title |
| `--status` | No | New status: wait/normal/blocked/bypass |
| `--pri` | No | New priority (0-5) |
| `--severity` | No | New severity level (1-4) |
| `--type` | No | New type |
| `--steps` | No | New test steps |
| `--expectation` | No | New expected result |

## Examples

```bash
# Update testcase status
zentao-cli testcase update 123 --status normal

# Update testcase priority
zentao-cli testcase update 123 --pri 3 --severity 2

# Update testcase content
zentao-cli testcase update 123 --steps "1. 新步骤" --expectation "新期望结果"
```

## API Endpoint

```
PUT /api.php/v1/testcases/{id}
```

