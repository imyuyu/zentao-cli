# build create

创建版本。

## Command
```bash
zentao-cli build create --name <name> --product <id> [--project <id>] [--execution <id>] [--scm-path <path>] [--ci <ci>] [--pkg <package>]
```

## Options

| Option | Required | Description |
|--------|----------|-------------|
| `--name` | Yes | Build name (e.g., "v1.0.0", "Build-2024-01-15") |
| `--product` | Yes | Product ID |
| `--project` | No | Project ID |
| `--execution` | No | Execution ID |
| `--scm-path` | No | SCM repository path |
| `--ci` | No | CI job name |
| `--pkg` | No | Package path |

## Examples

```bash
# Create a build
zentao-cli build create --name "v1.0.0" --product 1

# Create with project
zentao-cli build create --name "Build-001" --product 1 --project 5
```

