# 🔐 GhostKey

<div align="center">

[![CI](https://github.com/yourusername/ghostkey/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/ghostkey/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/ghostkey.svg)](https://crates.io/crates/ghostkey)
[![Downloads](https://img.shields.io/crates/d/ghostkey.svg)](https://crates.io/crates/ghostkey)

**A developer-first credential management system**

*Stop juggling API keys, tokens, and passwords. GhostKey keeps your secrets secure and accessible from the terminal.*

[Features](#features) • [Quick Start](#quick-start) • [Installation](#installation) • [Documentation](#documentation) • [Security](#security) • [Contributing](#contributing)

</div>

---

## Why GhostKey?

Managing credentials is painful. You have API keys in `.env` files, tokens in password managers, SSH keys scattered everywhere, and cloud credentials in various config files. GhostKey brings it all together in one secure, CLI-first tool.

**Built for developers who live in the terminal.**

### Key Differentiators

| Feature | GhostKey | Traditional Password Managers |
|---------|----------|-------------------------------|
| CLI-first workflow | ✅ Primary interface | ❌ GUI-focused |
| API keys & tokens | ✅ First-class support | ⚠️ Limited |
| Tag-based organization | ✅ Built-in | ⚠️ Folder-based |
| Environment variable export | ✅ Native | ❌ Manual |
| SSH config management | ✅ Integrated | ❌ Separate tools |
| Scriptable | ✅ JSON output | ⚠️ Limited |
| Local-first | ✅ No cloud required | ⚠️ Cloud-synced |

## Features

### Core

- 🔒 **Secure by default** - AES-256-GCM encryption, Argon2id key derivation
- 🖥️ **CLI-first** - Primary interface is the terminal, like git
- 🏠 **Local-first** - No cloud required, works fully offline
- 🏷️ **Tag-based organization** - Organize credentials your way
- 🔑 **Multiple credential types** - Passwords, API keys, SSH keys, tokens, env vars

### Developer Experience

- 📋 **Clipboard integration** - Copy secrets with auto-clear timeout
- 🌍 **Environment variable export** - Generate `.env` files instantly
- 🔍 **Full-text search** - Find credentials by name, tag, or description
- 📦 **Import/Export** - JSON, CSV, and env file formats
- 🐚 **Shell completions** - Bash, Zsh, Fish, PowerShell

### Security

- 🛡️ **Zero-knowledge** - Secrets never stored in plaintext
- 🔐 **Memory-safe** - Rust's ownership system prevents leaks
- ⏰ **Auto-clear clipboard** - Configurable timeout
- 🔒 **Master password** - Single password to rule them all

## Quick Start

### 1. Initialize your vault

```bash
ghostkey init
```

### 2. Add your first credential

```bash
# Add a GitHub token
ghostkey add github-token --type apikey --username myuser

# Add an AWS key
ghostkey add aws-key --type apikey --tags cloud,aws

# Add a database URL
ghostkey add database-url --type env
```

### 3. Use your credentials

```bash
# List all credentials
ghostkey list

# Get a credential
ghostkey get github-token --show

# Copy to clipboard (auto-clears in 30s)
ghostkey get github-token --clipboard

# Export to .env file
ghostkey export --format env > .env
```

## Installation

### Using Cargo (recommended)

```bash
cargo install ghostkey
```

### From source

```bash
git clone https://github.com/yourusername/ghostkey.git
cd ghostkey
cargo install --path .
```

### Quick install (Linux/macOS)

```bash
curl -sSfL https://raw.githubusercontent.com/yourusername/ghostkey/main/install.sh | sh
```

### Package managers

```bash
# Homebrew (macOS/Linux)
brew install ghostkey

# Arch Linux
yay -S ghostkey

# Windows (Scoop)
scoop install ghostkey
```

## Usage Examples

### Managing API Keys

```bash
# Add API keys
ghostkey add github-token --type apikey --username myuser --tags github,dev
ghostkey add aws-key --type apikey --tags cloud,aws,production
ghostkey add stripe-key --type apikey --tags payments

# List by tag
ghostkey list --tag cloud
ghostkey list --tag production

# Search
ghostkey search github
ghostkey search aws --json
```

### SSH Key Management

```bash
# Add SSH keys
ghostkey ssh add my-server

# Generate SSH config
ghostkey ssh config > ~/.ssh/config.d/ghostkey

# List all SSH keys
ghostkey ssh list
```

### Environment Variables

```bash
# Export all env vars
source <(ghostkey env export)

# Export for fish shell
ghostkey env export --shell fish

# Export to file
ghostkey env export > .env
```

### Clipboard Security

```bash
# Copy with 30-second timeout (default)
ghostkey clipboard github-token

# Copy with custom timeout
ghostkey clipboard github-token --timeout 60
```

### Import/Export

```bash
# Export to JSON
ghostkey export --format json > backup.json

# Export filtered by tag
ghostkey export --format env --tag production > prod.env

# Import from .env file
ghostkey import --format env .env

# Import from JSON
ghostkey import --format json backup.json
```

### Scripting

```bash
# Get secret in a script
SECRET=$(ghostkey get github-token --json --show | jq -r '.secret')

# Use in Docker
ghostkey env export > .env
docker run --env-file .env myapp

# CI/CD integration
eval "$(ghostkey env export)"
echo "::add-mask::$API_KEY"
```

## Commands Reference

### Core Commands

| Command | Description |
|---------|-------------|
| `ghostkey init` | Initialize a new vault |
| `ghostkey add <name>` | Add a new credential |
| `ghostkey get <name>` | Get a credential |
| `ghostkey list` | List all credentials |
| `ghostkey delete <name>` | Delete a credential |
| `ghostkey search <query>` | Search credentials |
| `ghostkey passwd` | Change master password |
| `ghostkey status` | Show vault status |

### Developer Commands

| Command | Description |
|---------|-------------|
| `ghostkey ssh <action>` | SSH key management |
| `ghostkey env <action>` | Environment variable management |
| `ghostkey clipboard <name>` | Copy to clipboard with auto-clear |
| `ghostkey completion <shell>` | Generate shell completions |

### Import/Export Commands

| Command | Description |
|---------|-------------|
| `ghostkey export` | Export credentials |
| `ghostkey import` | Import credentials |

### Tag Commands

| Command | Description |
|---------|-------------|
| `ghostkey tag list` | List all tags |
| `ghostkey tag add <credential> <tag>` | Add tag to credential |
| `ghostkey tag remove <credential> <tag>` | Remove tag from credential |

## Documentation

- [Advanced Usage Guide](docs/advanced-usage.md) - SSH config, env vars, scripting
- [Security Policy](SECURITY.md) - Encryption details and threat model
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Changelog](CHANGELOG.md) - Version history

## Security

### Encryption

- **Algorithm**: AES-256-GCM (authenticated encryption)
- **Key derivation**: Argon2id (memory-hard, GPU-resistant)
- **Salt**: 256-bit random salt per vault
- **Nonce**: 96-bit random nonce per encryption

### Key Derivation Parameters

- **Memory cost**: 64 MB
- **Time cost**: 3 iterations
- **Parallelism**: 4 threads

### Threat Model

| Threat | Protection | Mitigation |
|--------|------------|------------|
| Local attacker | AES-256-GCM | Strong master password |
| Leaked repository | Encrypted vault | .gitignore excludes vault |
| Stolen device | Argon2id | Full disk encryption |
| Memory dump | Rust safety | Zeroize on drop |

### Security Auditing

```bash
# Audit dependencies
cargo audit

# Run security checks
make security

# Generate coverage report
make coverage
```

See [SECURITY.md](SECURITY.md) for detailed security information.

## Shell Completions

### Bash

```bash
# Add to ~/.bashrc
eval "$(ghostkey completion bash)"
```

### Zsh

```bash
# Add to ~/.zshrc
autoload -Uz compinit
compinit
eval "$(ghostkey completion zsh)"
```

### Fish

```bash
ghostkey completion fish > ~/.config/fish/completions/ghostkey.fish
```

### PowerShell

```powershell
# Add to $PROFILE
Invoke-Expression -Command $(ghostkey completion powershell | Out-String)
```

## Configuration

GhostKey stores its data in `~/.ghostkey/`:

```
~/.ghostkey/
├── config.json     # User preferences
└── vault.enc       # Encrypted vault file
```

## Performance

GhostKey is fast. Benchmarks on modern hardware:

| Operation | Time |
|-----------|------|
| Encrypt 1KB | < 1ms |
| Decrypt 1KB | < 1ms |
| Key derivation | ~500ms |
| Encrypt 1MB | ~10ms |

## Comparison

### vs. Password Managers (1Password, LastPass)

- ✅ CLI-first (not GUI-first)
- ✅ Native API key support
- ✅ Environment variable export
- ✅ SSH config management
- ✅ Fully offline
- ❌ No browser integration

### vs. .env Files

- ✅ Encrypted storage
- ✅ Single source of truth
- ✅ Tag-based organization
- ✅ Search functionality
- ❌ Requires unlock step

### vs. Cloud Secret Managers (AWS Secrets Manager, HashiCorp Vault)

- ✅ No cloud dependency
- ✅ No ongoing costs
- ✅ Faster access
- ❌ No team sharing
- ❌ No audit logging

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/ghostkey.git
cd ghostkey

# Install pre-commit hooks
pre-commit install

# Run tests
make test

# Run lints
make lint
```

### Quick Commands

```bash
make build          # Build project
make test           # Run all tests
make lint           # Run lints
make coverage       # Generate coverage
make security       # Run security checks
make help           # Show all commands
```

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Acknowledgments

- [clap](https://crates.io/crates/clap) - Command line argument parser
- [aes-gcm](https://crates.io/crates/aes-gcm) - AES-GCM encryption
- [argon2](https://crates.io/crates/argon2) - Key derivation
- [zeroize](https://crates.io/crates/zeroize) - Secure memory clearing

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=yourusername/ghostkey&type=Date)](https://star-history.com/#yourusername/ghostkey&Date)

---

<div align="center">

**[Documentation](https://docs.rs/ghostkey)** •
**[Crates.io](https://crates.io/crates/ghostkey)** •
**[GitHub](https://github.com/yourusername/ghostkey)**

Made with ❤️ by developers, for developers.

</div>
