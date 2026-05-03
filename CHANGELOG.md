# Changelog

All notable changes to GhostKey will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- SSH key management (`ghostkey ssh`)
- Environment variable management (`ghostkey env`)
- Clipboard with auto-clear timeout (`ghostkey clipboard`)
- Full-text search (`ghostkey search`)
- Shell completions (Bash, Zsh, Fish, PowerShell)
- Import/Export (JSON, CSV, env formats)
- Advanced CLI options (--json, --names-only, --type filter)
- Performance benchmarks
- Comprehensive test suite
- Pre-commit hooks
- Code coverage reporting
- Security auditing (cargo-audit, cargo-deny)

### Changed
- Improved CLI help messages
- Better error messages
- Enhanced documentation

### Security
- Zeroize sensitive data on drop
- Constant-time comparisons
- Secure random number generation

## [0.1.0] - 2024-01-01

### Added
- Initial release
- CLI interface with clap
- AES-256-GCM encryption
- Argon2id key derivation
- Credential management (add, get, list, delete)
- Tag-based organization
- Multiple credential types (password, API key, SSH key, token, env var)
- Cross-platform clipboard support
- Secure password input
- Vault initialization and unlocking
- Configuration management
- Error handling
- Unit tests
- Integration tests
- Documentation (README, SECURITY, CONTRIBUTING)
- GitHub Actions CI/CD
- License (MIT OR Apache-2.0)

### Security
- Zero plaintext storage
- Memory-safe Rust implementation
- Authenticated encryption (AES-256-GCM)
- Memory-hard key derivation (Argon2id)
- Random salt and nonce generation

## [0.0.1] - 2024-01-01

### Added
- Project scaffolding
- Cargo configuration
- Basic project structure
- README template

---

## Release Notes

### v0.1.0

**Initial Release**

This is the first public release of GhostKey, a developer-first credential management system.

**Key Features**:
- **CLI-first workflow**: Primary interface is the terminal, like git
- **Secure by default**: AES-256-GCM encryption, Argon2id key derivation
- **Local-first**: No cloud required, works fully offline
- **Tag-based organization**: Organize credentials your way
- **Multiple credential types**: Passwords, API keys, SSH keys, tokens, env vars

**Getting Started**:
```bash
cargo install ghostkey
ghostkey init
ghostkey add my-credential
```

**What's Next**:
- SSH key management
- Environment variable export
- Clipboard auto-clear
- Shell completions
- Import/Export functionality

**Feedback**:
We'd love to hear your feedback! Please open an issue on GitHub.

---

## Upgrade Guide

### From 0.0.x to 0.1.0

No breaking changes. Simply update and enjoy new features.

---

## Deprecation Policy

We follow semantic versioning:
- **Major version**: Breaking changes with deprecation warnings in previous minor version
- **Minor version**: New features, backward compatible
- **Patch version**: Bug fixes, backward compatible

Deprecated features will be removed in the next major version.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute.

---

## License

Licensed under either of:
- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
