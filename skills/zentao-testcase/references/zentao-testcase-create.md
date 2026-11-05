# testcase create

创建新测试用例。

## Command
```bash
zentao-cli testcase create --product <id> --title <title> [--type <type>] [--pri <priority>] [--steps <steps>] [--story <story_id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | Yes | Product ID |
| `--title` | Yes | Testcase title |
| `--type` | No | Testcase type: feature/performance/interface/security/concurrency/destructive/install/others |
| `--pri` | No | Priority (0-5) |
| `--steps` | No | Test steps |
| `--story` | No | Related story ID |
| `--branch` | No | 分支 ID |
| `--module` | No | 模块 ID |
| `--stage` | No | 适用阶段 |
| `--precondition` | No | 前置条件 |
| `--keywords` | No | 关键词 |

## Examples

```bash
# Basic testcase creation
zentao-cli testcase create --product 1 --title "验证登录功能"

# Full testcase creation
zentao-cli testcase create \
  --product 1 \
  --title "用户登录测试" \
  --type feature \
  --pri 1 \
  --steps "1. 输入正确账号密码\n2. 点击登录" \
  --keywords "登录,冒烟"
```
