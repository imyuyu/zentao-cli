# build create

创建版本。

## Command
```bash
zentao-cli build create --name <name> --execution <id> --product <id> --builder <builder> \
  [--project <id>] [--branch <branch>] [--scm-path <path>] [--file-path <path>] [--date <date>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Build name (e.g., "v1.0.0", "Build-2024-01-15") |
| `--execution` | Yes | Execution ID（禅道按执行维度创建，缺失会静默失败） |
| `--product` | Yes | Product ID |
| `--builder` | Yes | 构建者账号 |
| `--project` | No | Project ID |
| `--branch` | No | 分支 ID |
| `--scm-path` | No | SCM repository path |
| `--file-path` | No | 下载地址 |
| `--date` | No | 打包日期（YYYY-MM-DD） |

## Examples

```bash
# 最小创建
zentao-cli build create --name "v1.0.0" --execution 5 --product 1 --builder admin

# 带源码与下载地址
zentao-cli build create --name "Build-001" --execution 5 --product 1 --builder admin \
  --scm-path "git@host:repo.git" --file-path "/pkg/v1.0.0.zip" --date 2026-11-05
```
