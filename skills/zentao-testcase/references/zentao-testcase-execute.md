# testcase result

执行测试用例。

## Command
```bash
zentao-cli testcase result <id> --result <result> [--consumed <minutes>] [--remark <remark>] [--build <build_id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Testcase ID |
| `--result` | Yes | Execution result: pass/fail/blocked |
| `--consumed` | No | Time consumed (in minutes) |
| `--remark` | No | Execution remarks |
| `--build` | No | Related build/version ID |

## Examples

```bash
# Mark testcase as passed
zentao-cli testcase result 123 --result pass

# Mark testcase as failed with details
zentao-cli testcase result 123 --result fail --remark "实际结果与期望不符" --consumed 5

# Mark testcase as blocked
zentao-cli testcase result 123 --result blocked --build 1
```
