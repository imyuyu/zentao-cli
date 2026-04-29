# Contributing to zentao-cli

Thank you for your interest in contributing to zentao-cli!

## How to Report Bugs

If you find a bug, please open an issue with the following information:
- Clear description of the problem
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Environment details (OS, Rust version, etc.)

## How to Submit Pull Requests

1. Fork the repository
2. Create a new branch for your feature or bugfix
3. Make your changes following the code conventions
4. Write tests for new functionality
5. Ensure all tests pass
6. Submit a pull request with a clear description of the changes

## Code Conventions

### Formatting

We use `cargo fmt` to format code. Please run it before committing:

```bash
cargo fmt
```

### Linting

We use `cargo clippy` for linting. Please ensure your code passes clippy:

```bash
cargo clippy
```

### General Guidelines

- Write clear, descriptive commit messages
- Keep functions small and focused
- Add documentation for public APIs
- Handle errors explicitly

## Testing Requirements

All new features must include tests. Please ensure all tests pass before submitting:

```bash
cargo test
```

Run tests with coverage:

```bash
cargo test -- --nocapture
```

## Branch Naming Conventions

Use the following prefix for branch names:

- `feature/` - for new features (e.g., `feature/add-story-export`)
- `fix/` - for bug fixes (e.g., `fix/auth-token-refresh`)
- `refactor/` - for code refactoring (e.g., `refactor/extract-api-client`)
- `docs/` - for documentation changes (e.g., `docs/update-readme`)
- `test/` - for adding or updating tests (e.g., `test/add-integration-tests`)

## Commit Message Format

We follow conventional commits:

```
<type>: <description>

[optional body]
```

Types: feat, fix, refactor, docs, test, chore, perf, ci

Example:
```
feat: add story export to CSV format

Add ability to export stories to CSV for reporting purposes.
Includes column selection and filtering options.
```

## Questions?

If you have questions, feel free to open an issue for discussion before making changes.
