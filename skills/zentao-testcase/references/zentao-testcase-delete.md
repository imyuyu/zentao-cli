# testcase delete

删除测试用例。

## Command
```bash
zentao-cli testcase delete <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Testcase ID |

## Examples

```bash
# Delete testcase
zentao-cli testcase delete 123
```

## API Endpoint

```
DELETE /api.php/v1/testcases/{id}
```

