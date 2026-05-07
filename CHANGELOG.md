# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1-beta.5] - 2026-05-07

### Added

- Secure credential storage using system keyring (Windows Credential Manager, macOS Keychain, Linux libsecret)
- `zentao auth refresh` command to re-login from stored credentials
- `zentao auth whoami` command to show current logged-in user
- Token auto-refresh mechanism in API client

### Changed

- Credentials now stored with Local persistence (not Enterprise) for "本地计算机" scope
- Account display masked in `config show` and `auth status` commands
- Upgraded keyring from 2.0 to 4.0 with new `Entry::new_with_modifiers` API

### Fixed

- Credential username field now properly set in Windows Credential Manager

## [0.0.1-beta.4] - 2026-05-06

### Fixed

- Add binary_name for Windows and improve copy_native_binaries
- Correct fallback path in copy_native_binaries
- Resolve artifact path conflicts in release workflow
- Use correct platform-specific npm package name

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

## [0.0.1-beta.2] - 2026-04-30

### Fixed

- Config save behavior - project config directory is now created if it doesn't exist

### Changed

- Updated skill descriptions with "禅道(ZenTao)" for better AI matching
- Added Chinese trigger keywords to all skill descriptions

## [0.0.1-beta.1] - 2026-04-30

### Added

- Unified `product_id`/`project_id` config priority across all commands
- Bug `+list` now reads `product_id` from config file
- Config init wizard shows product/project IDs after selection
- Support for reading `product_id`/`project_id` from config file
- CRUD operations for all API modules
- Additional ZenTao API interfaces
- Bug `+resolve` command
- Correct API paths and missing interfaces
- `resolvedBuild` field in bug update
- Auth login interactive input and token verification fixes
- Multi-platform binary publishing workflow
