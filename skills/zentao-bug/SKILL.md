---
name: zentao-bug
version: 1.1.1
description: "禅道(ZenTao) Bug (缺陷) 管理 - 查询 bug 列表、查看 bug 详情、创建 bug、更新 bug状态、解决 bug、关闭 bug。当用户说：'查询 bug'、'有哪些 bug'、'bug 列表'、'查看 bug'、'创建缺陷'、'报告 bug'、'登记 bug'、'缺陷'、'bug 数量'、'待处理 bug'、'bug 状态'、'禅道 bug' 时触发。"
metadata:
  requires:
    bins: ["zentao-cli"]
    envs: ["ZENTAO_URL", "ZENTAO_TOKEN"]
  cliHelp: "zentao-cli bug --help"
---

# Bug (v2)

**CRITICAL — 开始前 MUST 先用 Read 工具读取 [`../zentao-shared/SKILL.md`](../zentao-shared/SKILL.md)，其中包含认证、环境变量、错误处理和状态值定义。**

> **严重性判断技巧**：ZenTao Bug 的 severity (严重程度) 分 1-5 级，其中 1 为最严重（系统崩溃、数据丢失），5 为最低（功能建议）。创建 Bug 时应根据实际情况选择正确级别：
> - severity=1：系统崩溃、功能完全不可用、数据丢失
> - severity=2：主要功能失效、有明确 workaround
> - severity=3：普通缺陷，有替代方案
> - severity=4：轻微问题，界面/体验类
> - severity=5：功能建议/优化项
>
> **状态与解决方案**：Bug 有三个主状态 `active`(激活) / `resolved`(已解决) / `closed`(已关闭)，以及对应的 resolution。resolved 状态的 Bug 需要指定 resolution（fixed/duplicate/notrepro/wonfix/bysdesign），closed 状态需要先 resolved 再关闭。
>
> **指派与流转**：Bug 创建后可以通过 `bug update` 指派给开发人员。当 Bug 被修复后，状态变为 resolved；当 Bug 被确认关闭后，状态变为 closed。
>
> **友好输出**：在输出 Bug 详情时，建议同时输出 Bug 的 URL 链接（ZenTao web UI 地址），便于用户直接点击查看。

## Bug Lifecycle

```
active (激活) ──[修复]──> resolved (已解决) ──[确认]──> closed (已关闭)
    │                         │
    │<────[重打开]─────────────┘
    │
    └──[转为设计/需求]──> changed (已变更)
```

### 状态流转规则

| 当前状态 | 可转向 | 操作 |
|---------|-------|------|
| active | resolved | 修复完成，设置 resolution |
| active | closed | 直接关闭（wonfix/bysdesign 等情况） |
| resolved | active | 重打开 Bug |
| resolved | closed | 确认修复，关闭 Bug |
| closed | active | 重新激活 |

## Severity Levels

| Level | Name | Description | Example |
|-------|------|-------------|---------|
| 1 | Critical | 系统崩溃、数据丢失、功能完全不可用 | 页面打不开、关键数据丢失 |
| 2 | Major | 主要功能失效，有 workaround | 某核心按钮点击无响应 |
| 3 | Normal | 普通缺陷，有替代方案 | 某非关键功能异常 |
| 4 | Minor | 轻微问题，界面/体验类 | 文字显示不全 |
| 5 | Wishlist | 功能建议/优化项 | 希望增加某功能 |

## Commands

- [`bug list`](./references/zentao-bug-list.md) — List bugs for a product
- [`bug get`](./references/zentao-bug-get.md) — Get bug details
- [`bug create`](./references/zentao-bug-create.md) — Create a new bug
- [`bug update`](./references/zentao-bug-update.md) — Update bug fields
- [`bug resolve`](./references/zentao-bug-resolve.md) — Resolve a bug
- [`bug confirm`](./references/zentao-bug-confirm.md) — Confirm a bug
- [`bug close`](./references/zentao-bug-close.md) — Close a bug
- [`bug activate`](./references/zentao-bug-activate.md) — Activate a bug
- [`bug delete`](./references/zentao-bug-delete.md) — Delete a bug

## Common Use Cases

### 场景 1：查询产品的所有激活 Bug

```bash
# 按产品 ID 列出所有激活状态的 Bug
zentao-cli bug list --product 1 --status active
```

### 场景 2：创建新 Bug

```bash
# 创建严重性为 1 的紧急 Bug
zentao-cli bug create --title "用户登录页面崩溃" --product 1 --severity 1 --pri 1

# 创建带有详细复现步骤的 Bug
zentao-cli bug create \
  --title "上传头像失败" \
  --product 1 \
  --severity 2 \
  --pri 2 \
  --steps "1. 进入个人资料页\n2. 点击上传头像\n3. 选择图片后无响应"
```

### 场景 3：更新 Bug 状态和解决方案

```bash
# 标记 Bug 为已修复
zentao-cli bug update 123 --status resolved --resolution fixed

# 指派给开发人员
zentao-cli bug update 123 --assigned-to developer-name

# 重打开 Bug
zentao-cli bug update 123 --status active
```

### 场景 4：按指派人筛选 Bug

```bash
# 查看指定人员的待办 Bug
zentao-cli bug list --product 1 --assigned-to developer-name
```

## Error Handling

### 常见错误

| 错误码 | 说明 | 解决方案 |
|--------|------|----------|
| `ZEN_AUTH_FAILED` | Token 无效或过期 | 检查 ZENTAO_TOKEN 配置是否正确 |
| `ZEN_NOT_FOUND` | Bug 不存在 | 检查 Bug ID 是否正确 |
| `ZEN_CONFIG_INVALID` | 配置无效 | 验证 ZENTAO_URL 和 ZENTAO_TOKEN |
| `ZEN_API_ERROR` | ZenTao API 返回错误 | 检查服务器状态或稍后重试 |
| `ZEN_PARAM_MISSING` | 缺少必需参数 | 确保提供所有必需参数 |

### 错误处理示例

```bash
# 遇到 auth 错误时检查认证状态
zentao-cli auth status

# 验证配置
echo $ZENTAO_URL
echo $ZENTAO_TOKEN
```

## Examples

```bash
# 列出产品 1 下的所有激活状态的 Bug
zentao-cli bug list --product 1 --status active

# 创建 Bug
zentao-cli bug create --title "用户登录页面崩溃" --product 1 --severity 1 --pri 1

# 标记 Bug 为已修复
zentao-cli bug update 123 --status resolved --resolution fixed

# 查看指定人员的待办 Bug
zentao-cli bug list --product 1 --assigned-to developer-name
```

## Gotchas

1. **severity vs pri**：severity 表示缺陷严重程度（1-5），pri 表示优先级（1-4）。两者概念不同，创建 Bug 时需要分别设置。

2. **resolution 依赖 status**：只有在 status=resolved 时才需要设置 resolution。fixed/duplicate/notrepro/wonfix/bysdesign 是常见的解决方案。

3. **closed 状态不能直接进入**：Bug 不能从 active 直接变为 closed，必须先 resolved 再 closed。但 wonfix/bysdesign 等情况可以例外。

4. **steps 字段格式**：复现步骤建议使用 `\n` 分隔多行，便于阅读和解析。

5. **关联需求和项目**：创建 Bug 时可以通过 `--story` 和 `--project` 关联到已有的需求或项目。

