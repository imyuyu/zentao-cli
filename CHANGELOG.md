# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.1-beta.8] - 2026-05-19

### Changed

- Updated SKILL references to match new API field definitions

## [0.0.1-beta.7] - 2026-05-18

### Added

- Full API audit: all API structs aligned with official ZenTao API documentation
- Bug: Added `resolve()` API with full parameter support (resolution, resolvedBuild, assignedTo, duplicateBug, resolvedDate, comment)
- Story: Added `source`, `sourceNote`, `module`, `keywords`, `category` fields to create/update
- Task: Added `estStarted`, `deadline`, `fromBug`, `estimate`, `consumed` fields to create/update
- ProductPlan: Fixed title/name field aliasing per API docs

### Changed

- Bug: CreateRequest/UpdateRequest restructured to match official API fields
- Story: CreateRequest/UpdateRequest restructured with complete field set
- Task: CreateRequest/UpdateRequest restructured with complete field set
- All list response structs standardized with proper pagination fields
- TUI pages updated to match changed API struct types

### Fixed

- `user list` pagination (Issue #2) - now fetches all pages
- `bug update --assigned-to` correctly routes to resolve endpoint (Issue #3)

## [0.0.1-beta.6] - 2026-05-18

### Added

- New `navigation.rs` module extracting navigation logic from browser.rs

### Changed

- TUI browser rendering refactored for improved readability

### Removed

- Doc (文档) module removed from API, service, and CLI cmd layers

### Fixed

- `user list` now fetches all pages instead of only first page (20 users)
- `bug update --assigned-to` now correctly sends `assignedTo` (camelCase) to ZenTao API
- `Release.build` field now properly skips serialization when None
- Multiple outdated unit test type mismatches fixed

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
