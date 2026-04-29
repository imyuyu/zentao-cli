#!/usr/bin/env bash
# ZenTao CLI Installer
# Usage:
#   curl -L https://raw.githubusercontent.com/zentao-cli/cli/main/install.sh | sh
#   curl -L https://raw.githubusercontent.com/zentao-cli/cli/main/install.sh | sh -s -- --version v0.1.0
#   curl -L https://raw.githubusercontent.com/zentao-cli/cli/main/install.sh | sh -s -- --install-skills

set -e

REPO="zentao-cli/cli"
BIN_NAME="zentao-cli"
INSTALL_DIR="${HOME}/.local/bin"
FORCE=false
VERSION=""
INSTALL_SKILLS=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --install-skills)
            INSTALL_SKILLS=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        -h|--help)
            echo "ZenTao CLI Installer"
            echo ""
            echo "Usage:"
            echo "  curl -L https://.../install.sh | sh"
            echo "  curl -L https://.../install.sh | sh -s -- --version v0.1.0"
            echo "  curl -L https://.../install.sh | sh -s -- --install-skills"
            echo ""
            echo "Options:"
            echo "  --version VERSION   Install specific version (default: latest)"
            echo "  --install-dir DIR  Installation directory (default: ~/.local/bin)"
            echo "  --install-skills   Install Claude Code skills"
            echo "  --force           Overwrite existing binary"
            echo "  -h, --help        Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "darwin";;
        MINGW*|MSYS*|CYGWIN*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64";;
        aarch64|arm64)  echo "aarch64";;
        *)              echo "unknown";;
    esac
}

# Get latest version
get_latest_version() {
    local version
    if command -v curl &> /dev/null; then
        version=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget &> /dev/null; then
        version=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    fi
    echo "$version"
}

# Download and install
download_and_install() {
    local os="$1"
    local arch="$2"
    local version="$3"
    local extension=""
    local archive_ext=".tar.gz"

    [[ "$os" == "windows" ]] && extension=".exe" && archive_ext=".zip"

    local filename="${BIN_NAME}-${os}-${arch}${archive_ext}"
    local download_url="https://github.com/${REPO}/releases/download/${version}/${filename}"

    echo "Downloading ${BIN_NAME} ${version} for ${os}-${arch}..."
    echo "URL: ${download_url}"

    # Create install directory if not exists
    mkdir -p "$INSTALL_DIR"

    local dest="${INSTALL_DIR}/${BIN_NAME}${extension}"

    if [[ -f "$dest" && "$FORCE" == "false" ]]; then
        echo "Binary already exists at ${dest}"
        echo "Use --force to overwrite"
        exit 1
    fi

    # Download to temp
    local tmpfile=$(mktemp)
    trap "rm -f '$tmpfile'" EXIT

    if command -v curl &> /dev/null; then
        curl -L --progress-bar -o "${tmpfile}" "$download_url"
    elif command -v wget &> /dev/null; then
        wget -q --show-progress -O "${tmpfile}" "$download_url"
    else
        echo "Error: curl or wget is required"
        exit 1
    fi

    # Extract
    if [[ "$os" == "windows" ]]; then
        powershell -Command "Expand-Archive -Path '${tmpfile}' -DestinationPath '${tmpfile}_dir'"
        local extracted_dir="${tmpfile}_dir"
    else
        tar -xzf "${tmpfile}" -C $(dirname "${tmpfile}")
        local extracted_dir=$(dirname "${tmpfile}")
    fi

    # Find and install binary
    local binary="${extracted_dir}/${BIN_NAME}${extension}"
    if [[ ! -f "$binary" ]]; then
        binary=$(find "${extracted_dir}" -name "${BIN_NAME}${extension}" -type f 2>/dev/null | head -1)
    fi

    if [[ -z "$binary" || ! -f "$binary" ]]; then
        echo "Error: Binary not found in archive"
        exit 1
    fi

    cp "${binary}" "${dest}"
    chmod +x "${dest}"

    echo ""
    echo "Installed ${BIN_NAME} to ${dest}"
}

# Install Claude Code skills
install_skills() {
    local version="$1"
    local skills_dir="${HOME}/.claude/skills"

    echo ""
    echo "Installing Claude Code skills..."

    # Create skills directory
    mkdir -p "${skills_dir}"

    # Download skills as tarball
    local skills_url="https://github.com/${REPO}/releases/download/${version}/skills.tar.gz"
    local tmpfile=$(mktemp)
    trap "rm -f '$tmpfile'" EXIT

    if command -v curl &> /dev/null; then
        curl -sL -o "${tmpfile}" "${skills_url}"
    elif command -v wget &> /dev/null; then
        wget -q -O "${tmpfile}" "${skills_url}"
    else
        echo "Error: curl or wget is required"
        exit 1
    fi

    # Extract skills
    if [[ -s "${tmpfile}" ]]; then
        tar -xzf "${tmpfile}" -C "${skills_dir}"
        echo "Skills installed to ${skills_dir}"
    else
        echo "Warning: Skills download failed, skipping skills installation"
    fi
}

# Main
main() {
    local os=$(detect_os)
    local arch=$(detect_arch)
    local version="${VERSION}"

    if [[ "$os" == "unknown" ]]; then
        echo "Error: Unsupported operating system"
        exit 1
    fi

    if [[ "$arch" == "unknown" ]]; then
        echo "Error: Unsupported architecture"
        exit 1
    fi

    if [[ -z "$version" ]]; then
        echo "Fetching latest version..."
        version=$(get_latest_version)
        if [[ -z "$version" ]]; then
            echo "Error: Could not fetch latest version"
            exit 1
        fi
    fi

    echo "ZenTao CLI Installer"
    echo "===================="
    echo "OS: ${os}"
    echo "Arch: ${arch}"
    echo "Version: ${version}"
    echo "Install dir: ${INSTALL_DIR}"
    echo ""

    download_and_install "$os" "$arch" "$version"

    # Install skills if requested
    if [[ "$INSTALL_SKILLS" == "true" ]]; then
        install_skills "$version"
    fi

    # Add to PATH hint
    if [[ "$INSTALL_DIR" == "${HOME}/.local/bin" ]]; then
        if [[ ":$PATH:" != *":${HOME}/.local/bin:"* ]]; then
            echo ""
            echo "IMPORTANT: Add ${INSTALL_DIR} to your PATH if not already present:"
            echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
            echo ""
            echo "Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.)"
        fi
    fi

    echo ""
    echo "Run '${BIN_NAME} --version' to verify installation"
}

main
