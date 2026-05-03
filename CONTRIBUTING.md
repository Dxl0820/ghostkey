# Contributing to GhostKey

<div align="center">

**Thank you for your interest in contributing to GhostKey!**

This document provides guidelines and information for contributors.

[Code of Conduct](#code-of-conduct) • [Getting Started](#getting-started) • [Development](#development) • [Pull Requests](#pull-requests) • [Security](#security)

</div>

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- [Git](https://git-scm.com/)
- [pre-commit](https://pre-commit.com/) (optional, but recommended)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:

```bash
git clone https://github.com/yourusername/ghostkey.git
cd ghostkey
```

3. Add upstream remote:

```bash
git remote add upstream https://github.com/yourusername/ghostkey.git
```

### Setup Development Environment

```bash
# Install dependencies
cargo build

# Install pre-commit hooks
pre-commit install

# Run tests to verify setup
make test
```

## Development

### Project Structure

```
ghostkey/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli/              # CLI commands
│   ├── vault/            # Core encryption logic
│   ├── models/           # Data models
│   ├── config/           # Configuration
│   └── utils/            # Utility functions
├── tests/                # Integration tests
├── benches/              # Benchmarks
└── docs/                 # Documentation
```

### Code Style

We follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/).

#### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check
```

#### Linting

```bash
# Run clippy
cargo clippy -- -D warnings

# Run all lints
make lint
```

### Writing Code

#### Commit Messages

Use present tense, imperative mood:

```
Add SSH key support

- Add SSH key credential type
- Implement SSH key storage
- Add SSH config helper

Closes #123
```

**Format**:
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

#### Code Comments

- Document public APIs with `///` comments
- Explain "why", not "what"
- Keep comments up-to-date

```rust
/// Derives a 256-bit key from a password using Argon2id.
///
/// # Arguments
///
/// * `password` - The password to derive from
/// * `salt` - Random salt for key derivation
///
/// # Returns
///
/// A 256-bit key suitable for AES-256-GCM encryption.
///
/// # Security
///
/// Uses Argon2id with 64MB memory, 3 iterations, and 4 threads.
/// This provides strong resistance against GPU and ASIC attacks.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    // Implementation
}
```

### Testing

#### Unit Tests

Unit tests are in `src/tests/` and inline with code.

```bash
# Run unit tests
cargo test --lib

# Run specific test
cargo test test_encrypt_decrypt

# Run with output
cargo test -- --nocapture
```

#### Integration Tests

Integration tests are in `tests/`.

```bash
# Run integration tests
cargo test --test vault_test

# Run all tests
cargo test
```

#### Benchmarks

```bash
# Run benchmarks
cargo bench
```

### Security

#### Security-Critical Code

All changes to cryptographic code require careful review:

- Encryption/decryption logic
- Key derivation
- Password handling
- Memory safety
- Random number generation

#### Security Checklist

- [ ] No hardcoded secrets
- [ ] No plaintext password storage
- [ ] Proper error handling (no info leaks)
- [ ] Secure random number generation
- [ ] Proper key derivation
- [ ] Memory zeroing for sensitive data
- [ ] Input validation
- [ ] Output encoding

#### Security Review

All PRs that touch security-critical code will be reviewed by a security-conscious maintainer.

## Pull Requests

### PR Process

1. **Create a branch** from `main`:

```bash
git checkout -b feature/my-feature
```

2. **Make your changes** following the guidelines above

3. **Write tests** for new functionality

4. **Update documentation** as needed

5. **Run checks**:

```bash
make check
```

6. **Push your branch**:

```bash
git push origin feature/my-feature
```

7. **Create a PR** on GitHub

### PR Template

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing performed

## Checklist

- [ ] Code follows style guidelines
- [ ] Self-review performed
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

### PR Guidelines

- **Keep PRs focused**: One feature or fix per PR
- **Write clear descriptions**: Explain what and why
- **Include tests**: All new code should have tests
- **Update docs**: Keep documentation current
- **Respond to feedback**: Address review comments

### Review Process

1. **Automated checks**: CI must pass
2. **Code review**: At least one maintainer approval
3. **Security review**: For security-critical changes
4. **Merge**: Squash and merge to main

## Types of Contributions

### Bug Reports

**Before submitting**:
- Check existing issues
- Reproduce the bug
- Gather relevant information

**Include**:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details
- Error messages

### Feature Requests

**Before submitting**:
- Check existing issues
- Consider if it fits the project scope
- Think about implementation

**Include**:
- Use case description
- Proposed solution
- Alternatives considered
- Mockups (if applicable)

### Documentation

- Fix typos
- Improve clarity
- Add examples
- Update outdated info

### Code

- Bug fixes
- New features
- Performance improvements
- Refactoring

## Development Tips

### Useful Commands

```bash
# Build
make build

# Test
make test

# Lint
make lint

# Format
make fmt

# Coverage
make coverage

# Security checks
make security

# All checks
make check
```

### Debugging

```bash
# Run with debug output
RUST_LOG=debug cargo run -- <command>

# Run with backtrace
RUST_BACKTRACE=1 cargo run -- <command>

# Run specific test with output
cargo test test_name -- --nocapture
```

### IDE Setup

**VS Code**:
- Install `rust-analyzer` extension
- Install `Even Better TOML` extension
- Configure `settings.json`:

```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.check.extraArgs": ["--", "-D", "warnings"]
}
```

**CLion/IntelliJ**:
- Install Rust plugin
- Configure Rust toolchain
- Enable clippy inspection

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release branch
4. Run all checks
5. Create GitHub release
6. Publish to crates.io

## Community

### Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Discord**: Real-time chat (if available)

### Communication

- Be respectful and inclusive
- Use clear, concise language
- Provide context and examples
- Be patient with responses

## License

By contributing, you agree that your contributions will be licensed under the project's MIT OR Apache-2.0 license.

## Acknowledgments

Thank you to all contributors who help make GhostKey better!

## Questions?

If you have questions about contributing, please:

1. Check the [documentation](docs/)
2. Search [existing issues](https://github.com/yourusername/ghostkey/issues)
3. Create a [new issue](https://github.com/yourusername/ghostkey/issues/new)

---

<div align="center">

**Thank you for contributing to GhostKey!**

</div>
