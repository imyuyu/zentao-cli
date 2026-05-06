---
name: zentao-shared
version: 1.2.0
description: "禅道(ZenTao) CLI 共享基础：配置初始化、认证（token-based）、配置优先级、权限错误处理、更新检查、安全规则。当用户说：'配置 zentao'、'初始化配置'、'config init'、'认证'、'登录 zentao'、'禅道登录'、'token'、'查看配置'、'config show'、'config set'、'环境变量'、'ZENTAO_URL'、'ZENTAO_TOKEN'、'权限不足'、'auth status' 时触发。"
---

# zentao-cli 共享规则

本技能指导你如何通过zentao-cli操作ZenTao资源, 以及有哪些注意事项。

## 配置初始化

### 自动引导配置流程

**当用户未配置或认证失败时：**

1. **检测配置状态**
   ```bash
   zentao-cli auth status
   ```
   如果返回 `ZEN_AUTH_FAILED` 或 `ZEN_CONFIG_INVALID`，说明未配置或配置无效。

2. **询问用户配置信息和保存位置**
   
   **必须明确询问保存位置：全局还是当前项目？**
   
   向用户询问以下内容：
   - ZenTao 服务地址（例如 `http://172.168.80.30:8001/zentao`）
   - ZenTao 账号
   - ZenTao 密码
   - **保存位置**：全局（`~/.zentao-cli/config.toml`）还是当前项目（`.zentao-cli/config.toml`）？

   > ⚠️ **重要**：必须等用户明确回答保存位置后，才能执行配置命令。禁止在用户未选择的情况下默认使用全局配置。

3. **执行配置**

   全局保存：
   ```bash
   zentao-cli config set -g url "用户提供的地址"
   zentao-cli auth login -g --account 账号 --password 密码
   ```

   当前项目保存：
   ```bash
   zentao-cli config set url "用户提供的地址"
   zentao-cli auth login --account 账号 --password 密码
   ```

   > 注意：`config set` 只支持 `url`、`token`、`product_id`、`project_id`，不支持直接设置账号密码。必须分两步：先设置 URL，再用 `auth login` 通过账号密码获取 token。

4. **验证配置成功**
   ```bash
   zentao-cli auth status
   ```

### 快速检查

```bash
# 检查认证状态
zentao-cli auth status

# 验证配置
echo $ZENTAO_URL
echo $ZENTAO_TOKEN
```

### 获取 Token（手动）

1. 登录 ZenTao Web 界面
2. 进入个人设置 → API Token
3. 生成新 Token
4. 设置为 `ZENTAO_TOKEN` 环境变量

### 环境变量说明

| 变量 | 必需 | 说明 |
|------|------|------|
| `ZENTAO_URL` | 是 | ZenTao 服务器地址，例如 `https://zentao.example.com` |
| `ZENTAO_TOKEN` | 是 | API Token，用于认证 |
| `ZENTAO_PRODUCT_ID` | 否 | 默认产品 ID |
| `ZENTAO_PROJECT_ID` | 否 | 默认项目 ID |

## 认证

### Token-based 认证

ZenTao 使用 Token 进行认证，Token 通过 `ZENTAO_TOKEN` 环境变量传递。

```bash
# 验证认证状态
zentao-cli auth status
```

**错误响应格式**：

```json
{
  "status": "error",
  "error": {
    "code": "ZEN_AUTH_FAILED",
    "message": "Invalid token or token expired",
    "hint": "Verify ZENTAO_TOKEN is correct"
  }
}
```

### 常见错误码

| Code | Description | Resolution |
|------|-------------|------------|
| `ZEN_AUTH_FAILED` | Token 无效或过期 | 验证 ZENTAO_TOKEN 是否正确 |
| `ZEN_NOT_FOUND` | 资源不存在 | 检查 ID 后重试 |
| `ZEN_CONFIG_INVALID` | 配置无效 | 验证 ZENTAO_URL 和 ZENTAO_TOKEN |
| `ZEN_API_ERROR` | ZenTao API 返回错误 | 检查服务器状态或稍后重试 |
| `ZEN_PARAM_MISSING` | 缺少必需参数 | 提供所有必需参数 |

### 权限不足处理

遇到权限相关错误时：

1. 确认当前用户角色（ZenTao 中通常是 `admin`、`leader` 或 `member`）
2. 验证 Token 是否具有操作该资源所需的权限
3. 如果是管理员，请检查该用户是否被正确分配到相关项目/产品

## 配置优先级

CLI 参数优先级从高到低：

1. **命令行参数**（`--product`, `--project` 等）
2. **环境变量**（`ZENTAO_PRODUCT_ID`, `ZENTAO_PROJECT_ID`）
3. **默认配置**

```bash
# 命令行参数优先级最高
zentao-cli task list --project 5  # 使用 --project 参数

# 环境变量作为默认值
export ZENTAO_PROJECT_ID=5
zentao-cli task list              # 使用环境变量

# 默认值兜底
zentao-cli task list              # 使用配置中的默认值
```

## 通用选项

| 选项 | 说明 |
|------|------|
| `--format` | 输出格式：`json`、`pretty`、`table`、`ndjson`、`csv` |
| `--product` | 按产品 ID 过滤 |
| `--project` | 按项目 ID 过滤 |
| `--status` | 按状态筛选 |

## 输出格式

```bash
# JSON 输出（默认，用于脚本）
zentao-cli story list --product 1 --format json

# Pretty 打印输出（默认，用于终端）
zentao-cli story list --product 1 --format pretty

# 表格输出
zentao-cli story list --product 1 --format table
```

## 更新检查

执行 `zentao-cli` 命令后，如果检测到新版本，JSON 输出中会包含 `_notice.update` 字段（含 `message`、`command` 等）。

**当在输出中看到 `_notice.update` 时，完成当前请求后补充告知并提议更新**：

1. 告知当前版本和最新版本号
2. 提议执行：
   ```bash
   npm update -g zentao-cli && npx skills add zentao/cli -g -y
   ```
3. 提醒用户退出并重新打开 AI Agent 以加载最新 Skills

**规则**：不要静默忽略更新提示。即使当前任务与更新无关，也应在完成用户请求后补充告知。

## 安全规则

- **禁止输出密钥**（token、appSecret）到终端明文。
- **写入/删除操作前必须确认用户意图**。
- 用 `--dry-run` 预览危险请求。

