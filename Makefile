.PHONY: build build-release install test lint clean tag-release package

# Default target
all: build

# Build debug version
build:
	cargo build

# Build release version
build-release:
	cargo build --release

# Run tests
test:
	cargo test --verbose

# Run linter
lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt --all

# Clippy checks
clippy:
	cargo clippy -- -D warnings

# Install to ~/.local/bin (for local development)
install-local: build-release
	install -Dm755 target/release/zentao-cli ${HOME}/.local/bin/zentao-cli
	@echo "Installed to ~/.local/bin/zentao-cli"

# Create a release tag
tag-release:
	@if [ -z "$$VERSION" ]; then \
		echo "Usage: VERSION=v0.1.0 make tag-release"; \
		exit 1; \
	fi
	git tag -a "v$$VERSION" -m "Release v$$VERSION"
	git push origin "v$$VERSION"

# Build all platforms (requires cross)
build-all:
	cross build --release --target x86_64-unknown-linux-musl
	cross build --release --target aarch64-unknown-linux-musl
	cross build --release --target x86_64-apple-darwin
	cross build --release --target aarch64-apple-darwin
	cross build --release --target x86_64-pc-windows-msvc

# Package for release
package:
	@mkdir -p release
	cd target/release && \
	tar czf ../../release/zentao-cli-linux-x86_64.tar.gz zentao-cli && \
	tar czf ../../release/zentao-cli-linux-aarch64.tar.gz zentao-cli && \
	cp zentao-cli.exe ../../release/ 2>/dev/null || true
	@echo "Packages created in release/"

# Package skills
package-skills:
	@mkdir -p release
	tar czf release/skills.tar.gz skills/
	@echo "Skills packaged"
