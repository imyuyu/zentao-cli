# user get

获取用户详情。

## Command
```bash
zentao-cli user get <id>
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `<id>` | Yes | 用户 ID |

## Examples

```bash
# Get user details
zentao-cli user get 1
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

