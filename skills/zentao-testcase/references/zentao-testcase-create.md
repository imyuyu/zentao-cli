# testcase +create

创建新测试用例。

## Command
```bash
zentao-cli testcase +create --product <id> --title <title> [--type <type>] [--severity <level>] [--pri <priority>] [--steps <steps>] [--expectation <expectation>] [--story <story_id>] [--project <project_id>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--product` | Yes | Product ID |
| `--title` | Yes | Testcase title |
| `--type` | No | Testcase type: feature/performance/interface/security/concurrency/destructive/install/others |
| `--severity` | No | Severity level (1-4, 1 is most severe) |
| `--pri` | No | Priority (0-5) |
| `--steps` | No | Test steps |
| `--expectation` | No | Expected result |
| `--story` | No | Related story ID |
| `--project` | No | Project ID |

## Examples

```bash
# Basic testcase creation
zentao-cli testcase +create --product 1 --title "验证登录功能"

# Full testcase creation
zentao-cli testcase +create \
  --product 1 \
  --title "用户登录测试" \
  --type feature \
  --severity 2 \
  --pri 1 \
  --steps "1. 输入正确账号密码\n2. 点击登录" \
  --expectation "登录成功，跳转首页"
```

## API Endpoint

```
POST /api.php/v1/products/{productId}/testcases
```
