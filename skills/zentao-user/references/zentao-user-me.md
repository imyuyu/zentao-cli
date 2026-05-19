# user me

获取当前登录用户信息。

## Command

```bash
zentao-cli user me
```

## Examples

```bash
# 获取当前用户信息
zentao-cli user me
```

## API Endpoint

```
GET /api.php/v1/user
```

## Output Fields

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 用户 ID |
| account | string | 登录账号名 |
| realname | string | 真实姓名 |
| email | string | 邮箱 |
| dept | u64 | 部门 ID |
| role | string | 角色 |
