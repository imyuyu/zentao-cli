# task close

关闭任务。

## Command
```bash
zentao-cli task close <id> [--comment <comment>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--comment` | No | 备注 |

## Examples

```bash
# 关闭任务
zentao-cli task close 456

# 关闭任务并备注
zentao-cli task close 456 --comment "需求已取消"
```

## Status Transition

- Changes task status from `done` to `closed`
