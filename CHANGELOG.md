# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1-beta.3] - 2026-04-30

### Fixed

- Broken pipe panic when CLI output is piped to other commands (e.g., `head`, `tail`, `jq`)
- CLI version string hardcoded as "0.0.3" - now reads from Cargo.toml
- Config save fallback behavior - when `global=false`, config is now saved to project directory instead of falling back to global
- Removed non-existent `--limit` and `--page` options from shared skill documentation

### Changed

- Updated `zentao-shared` skill with auto-configuration guidance
- Added config flow for asking user about save location (global vs project)

### Added

- Safe print functions (`safe_println`, `safe_print`) to handle broken pipe errors gracefully

## [0.2.0] - 2026-04-29

### Changed

- Rust 1.70 minimum version requirement
- Updated dependencies for security and compatibility

### Fixed

- cargo clippy warnings resolved
- Test suite passes (103 tests)
- Repository configuration improvements
- Updated .gitignore for proper Rust project coverage

### Added

- New package metadata and keywords

## [0.1.0] - 2026-04-27

### Added

- Initial release
- Story CRUD operations
- Bug CRUD operations
- Task CRUD operations (list, get, create, update)
- Product listing
- Project listing
- Token-based authentication
- Multiple output formats (table, json, pretty)
- TUI browser for bugs and stories
- Configuration management with profiles
- Diagnostic command (doctor)
