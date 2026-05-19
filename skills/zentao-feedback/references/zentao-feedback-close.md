# feedback close

关闭反馈。

## Command

```bash
zentao-cli feedback close <id> --closed-reason <reason> [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 反馈 ID |
| `--closed-reason` | Yes | 关闭原因 |
| `--comment` | No | 关闭备注 |

## Examples

```bash
# 关闭反馈
zentao-cli feedback close 1 --closed-reason "已处理完成"

# 关闭并添加备注
zentao-cli feedback close 1 --closed-reason "无法复现" --comment "经测试无法复现此问题"
```

## API Endpoint

```
POST /api.php/v1/feedbacks/{id}/close
```
