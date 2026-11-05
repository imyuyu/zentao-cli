# build update

更新版本信息。

## Command
```bash
zentao-cli build update <id> [--name <name>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | Build ID |
| `--name` | No | New build name |
| `--desc` | No | 版本描述 |
| `--builder` | No | 构建者 |
| `--date` | No | 打包日期 |
| `--file-path` | No | 下载地址 |
| `--scm-path` | No | 源码地址 |

## Examples

```bash
# Update build name
zentao-cli build update 1 --name "v1.0.1"

# Update build status
zentao-cli build update 1 --name "v1.0.1" --desc "修复打包"
```

