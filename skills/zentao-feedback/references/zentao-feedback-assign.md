# feedback assign

指派反馈给指定人员。

## Command

```bash
zentao-cli feedback assign <id> --assigned-to <account> [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 反馈 ID |
| `--assigned-to` | Yes | 被指派人的账号 |
| `--comment` | No | 指派备注 |

## Examples

```bash
# 指派反馈给开发人员
zentao-cli feedback assign 1 --assigned-to developer

# 指派并添加备注
zentao-cli feedback assign 1 --assigned-to developer --comment "请尽快处理此问题"
```

## API Endpoint

```
POST /api.php/v1/feedbacks/{id}/assign
```
