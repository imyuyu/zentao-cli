# bug create

创建新缺陷。

## Command
```bash
zentao-cli bug create --title <title> --product <id> --severity <level> [--pri <priority>] [--steps <steps>] [--branch <branch>] [--module <module>] [--execution <execution>] [--keywords <keywords>] [--os <os>] [--browser <browser>] [--deadline <deadline>] [--opened-build <opened_build>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--title` | Yes | Bug title |
| `--product` | Yes | Product ID |
| `--severity` | Yes | Severity level (1-5, 1=最严重) |
| `--pri` | No | Priority (0-5) |
| `--type` | No | Bug type (codeerror/interface/design/others) |
| `--steps` | No | Steps to reproduce |
| `--story` | No | Related story ID |
| `--branch` | No | Branch ID |
| `--module` | No | Module ID |
| `--execution` | No | Execution ID |
| `--keywords` | No | Keywords |
| `--os` | No | Operating system |
| `--browser` | No | Browser |
| `--deadline` | No | Deadline |
| `--opened-build` | No | Affected build (e.g., trunk) |

## Examples

```bash
# Basic bug creation
zentao-cli bug create --title "页面崩溃" --product 1 --severity 1

# Full bug creation
zentao-cli bug create \
  --title "用户头像上传失败" \
  --product 1 \
  --severity 2 \
  --pri 2 \
  --steps "1. 进入个人资料页\n2. 点击上传头像\n3. 选择图片后无响应" \
  --os "Windows" \
  --browser "Chrome"
```

