# user update

修改用户信息。

## Command

```bash
zentao-cli user update <id> [--account <account>] [--realname <realname>] [--role <role>] [--dept <dept>] [--mobile <mobile>] [--email <email>] [--phone <phone>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 用户 ID |
| `--account` | No | 登录账号名 |
| `--realname` | No | 真实姓名 |
| `--role` | No | 角色 |
| `--dept` | No | 部门 ID |
| `--mobile` | No | 手机 |
| `--email` | No | 邮箱 |
| `--phone` | No | 电话 |

## Examples

```bash
# 更新用户姓名
zentao-cli user update 123 --realname "新姓名"

# 更新用户角色和部门
zentao-cli user update 123 --role manager --dept 2
```

## API Endpoint

```
PUT /api.php/v1/users/{id}
```
