# task estimate

批量添加任务工时日志。

## Command
```bash
zentao-cli task estimate <id> --dates <dates> --work <work> --consumed <consumed> --left <left>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Task ID |
| `--dates` | Yes | 日期列表（多个用逗号分隔） |
| `--work` | Yes | 工作内容列表（多个用逗号分隔） |
| `--consumed` | Yes | 消耗工时列表（多个用逗号分隔） |
| `--left` | Yes | 剩余工时列表（多个用逗号分隔） |

## Examples

```bash
# 添加单条工时日志
zentao-cli task estimate 456 --dates "2024-01-15" --work "编码" --consumed "4" --left "4"

# 批量添加工时日志
zentao-cli task estimate 456 --dates "2024-01-15,2024-01-16" --work "编码,代码评审" --consumed "4,2" --left "4,2"
```
