# user create

创建新用户。

## Command

```bash
zentao-cli user create --account <account> --password <password> --realname <realname> --role <role> [--dept <dept>] [--mobile <mobile>] [--email <email>] [--phone <phone>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--account` | Yes | 登录账号名 |
| `--password` | Yes | 密码 |
| `--realname` | Yes | 真实姓名 |
| `--role` | Yes | 角色 (dev/manager/qa/pd/op/admin) |
| `--dept` | No | 部门 ID |
| `--mobile` | No | 手机 |
| `--email` | No | 邮箱 |
| `--phone` | No | 电话 |

## Examples

```bash
# 创建开发用户
zentao-cli user create --account newdev --password "123456" --realname "新开发" --role dev

# 创建完整信息用户
zentao-cli user create --account newuser --password "123456" --realname "新用户" --role dev --dept 1 --email "user@example.com"
```

## API Endpoint

```
POST /api.php/v1/users
```
