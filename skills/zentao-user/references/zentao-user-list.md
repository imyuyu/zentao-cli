# user list

列出用户列表。

## Command
```bash
zentao-cli user list [--dept <id>] [--role <role>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--dept` | No | 按部门 ID 筛选 |
| `--role` | No | 按角色筛选 |

## Examples

```bash
# List all users
zentao-cli user list

# List users in a specific department
zentao-cli user list --dept 1

# List users with a specific role
zentao-cli user list --role dev
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 用户 ID |
| account | string | 登录账号 |
| realname | string | 真实姓名 |
| email | string | 邮箱（可选） |
| dept | u64 | 部门 ID（可选） |
| role | string | 角色（可选） |
