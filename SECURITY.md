# Security Policy

<div align="center">

**GhostKey takes security seriously.**

This document describes the security model, threat analysis, and vulnerability reporting process.

</div>

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes    |

## Reporting a Vulnerability

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead, please email **security@example.com** with:

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix (if any)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix release**: Depends on severity
  - Critical: Within 24 hours
  - High: Within 1 week
  - Medium: Within 1 month
  - Low: Next release

### Bug Bounty

We currently do not have a bug bounty program, but we deeply appreciate responsible disclosure and will credit reporters in our changelog.

## Security Architecture

### Encryption

#### Symmetric Encryption

- **Algorithm**: AES-256-GCM
- **Key size**: 256 bits
- **Nonce size**: 96 bits (12 bytes)
- **Authentication**: Built-in (GCM mode)

AES-GCM is an **authenticated encryption** mode that provides:
- **Confidentiality**: Data is encrypted
- **Integrity**: Any tampering is detected
- **Authenticity**: Data origin is verified

#### Key Derivation

- **Algorithm**: Argon2id
- **Memory cost**: 64 MB (65536 KB)
- **Time cost**: 3 iterations
- **Parallelism**: 4 threads
- **Output**: 256 bits

Argon2id is the **winner of the Password Hashing Competition** and provides:
- **Memory-hardness**: Resistant to GPU/ASIC attacks
- **Time-memory tradeoff resistance**: Cannot reduce memory without increasing time
- **Side-channel resistance**: Hybrid of Argon2i and Argon2d

### Data Storage

#### Vault File Structure

```
~/.ghostkey/vault.enc
├── Header
│   ├── Version (u32)
│   └── Salt (256 bits)
├── Encrypted Data
│   ├── Nonce (96 bits)
│   ├── Ciphertext
│   └── Authentication Tag (128 bits)
└── Checksum
```

#### Credential Storage

Each credential is encrypted individually:
- **Secret**: Encrypted with AES-256-GCM
- **Metadata**: Stored separately (tags, username, description)
- **Nonce**: Unique per credential

### Memory Safety

#### Rust's Guarantees

- **Ownership**: Prevents double-free
- **Borrowing**: Prevents use-after-free
- **Lifetimes**: Prevents dangling pointers
- **No null**: Option type enforces handling

#### Additional Protections

- **Zeroize crate**: Securely clears sensitive data from memory
- **Minimal lifetime**: Secrets are cleared as soon as possible
- **No string interning**: Passwords never enter String pool

```rust
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
struct SensitiveData {
    key: [u8; 32],
    password: Vec<u8>,
}
```

## Threat Model

### Threat 1: Local Attacker (Physical Access)

**Scenario**: Attacker gains access to your device

| Attack | Protection | Residual Risk |
|--------|------------|---------------|
| Read vault file | AES-256-GCM encryption | None (without password) |
| Brute-force password | Argon2id (memory-hard) | Weak passwords |
| Memory dump | Zeroize on drop | Cold boot attacks |
| Keylogger | None | Cannot protect |
| Hardware keylogger | None | Cannot protect |

**Mitigations**:
- Use a strong master password (12+ characters)
- Enable full disk encryption (BitLocker/FileVault/LUKS)
- Use a hardware security key for 2FA (future)

### Threat 2: Leaked Repository

**Scenario**: Vault file accidentally committed to Git

| Attack | Protection | Residual Risk |
|--------|------------|---------------|
| Read passwords | AES-256-GCM | None (without password) |
| Rainbow table attack | Random salt per vault | None |
| Brute-force | Argon2id | Weak passwords |

**Mitigations**:
- `.gitignore` excludes vault files
- Git hooks can warn about sensitive files (future)
- Each vault has unique salt

**What to do if vault is leaked**:
1. Change your master password immediately
2. Rotate all credentials in the vault
3. Check for unauthorized access

### Threat 3: Stolen Device

**Scenario**: Laptop is stolen

| Attack | Protection | Residual Risk |
|--------|------------|---------------|
| Offline brute-force | Argon2id (memory-hard) | Weak passwords |
| Data recovery | Full disk encryption | None |
| Quick access | Auto-lock (future) | Short timeout |

**Mitigations**:
- Enable full disk encryption
- Use a strong master password
- Enable auto-lock timeout (future)
- Use device encryption (BitLocker/FileVault)

### Threat 4: Malicious Software

**Scenario**: Malware on your device

| Attack | Protection | Residual Risk |
|--------|------------|---------------|
| Read vault | Encrypted at rest | None |
| Intercept password | Keylogger protection | Limited |
| Clipboard hijacking | Auto-clear timeout | 30-second window |
| Memory scraping | Zeroize on drop | Limited window |

**Mitigations**:
- Keep your OS and software updated
- Use antivirus software
- Don't run untrusted software
- Use clipboard auto-clear

### Threat 5: Insider Threat

**Scenario**: Someone with your password

| Attack | Protection | Residual Risk |
|--------|------------|---------------|
| Access vault | Password required | Password compromise |
| Export credentials | Audit logging (future) | None currently |

**Mitigations**:
- Don't share your master password
- Use a password manager for your master password
- Enable 2FA (future)

## Security Best Practices

### Master Password

**DO**:
- Use 12+ characters
- Mix uppercase, lowercase, numbers, symbols
- Use a unique password (not reused)
- Store in a password manager

**DON'T**:
- Use common passwords (password, 123456)
- Use personal information (birthday, name)
- Share with others
- Write down insecurely

### Vault Security

**DO**:
- Enable full disk encryption
- Lock your device when away
- Use auto-lock timeout
- Regular backups

**DON'T**:
- Commit vault to Git
- Store on shared computers
- Leave unlocked
- Skip updates

### Credential Management

**DO**:
- Use descriptive names
- Tag credentials appropriately
- Rotate credentials regularly
- Delete unused credentials

**DON'T**:
- Store in plaintext files
- Share via insecure channels
- Use same password everywhere
- Ignore security alerts

## Cryptographic Details

### AES-256-GCM

```
Key:    256 bits (32 bytes)
Nonce:  96 bits (12 bytes)
Tag:    128 bits (16 bytes)
```

**Why AES-256-GCM?**
- NIST approved
- Hardware acceleration (AES-NI)
- Authenticated encryption
- Industry standard

### Argon2id

```
Memory:  64 MB (65536 KB)
Time:    3 iterations
Threads: 4
Output:  256 bits (32 bytes)
```

**Why Argon2id?**
- Winner of Password Hashing Competition
- Memory-hard (resists GPU/ASIC)
- Side-channel resistant
- Recommended by OWASP

### Random Number Generation

- **Source**: Operating system CSPRNG
- **Rust crate**: `rand` with `OsRng`
- **Usage**: Salt, nonce, UUID generation

```rust
use rand::rngs::OsRng;
use rand::RngCore;

let mut salt = [0u8; 32];
OsRng.fill_bytes(&mut salt);
```

## Security Auditing

### Automated Checks

```bash
# Audit dependencies
cargo audit

# Check for vulnerabilities
cargo deny check

# Run security lints
cargo clippy -- -D warnings

# Generate coverage
cargo tarpaulin
```

### Manual Review Checklist

- [ ] No hardcoded secrets
- [ ] No plaintext password storage
- [ ] Proper error handling (no info leaks)
- [ ] Secure random number generation
- [ ] Proper key derivation
- [ ] Memory zeroing for sensitive data
- [ ] Input validation
- [ ] Output encoding

### Third-Party Audits

We welcome security researchers to audit our code. If you're interested, please contact us.

## Dependencies

### Security-Critical Dependencies

| Crate | Purpose | Audited |
|-------|---------|---------|
| aes-gcm | Encryption | ✅ |
| argon2 | Key derivation | ✅ |
| zeroize | Memory clearing | ✅ |
| rand | Random numbers | ✅ |
| subtle | Constant-time ops | ✅ |

### Dependency Management

- **Lock file**: `Cargo.lock` is committed
- **Updates**: Regular dependency updates
- **Auditing**: `cargo audit` in CI

## Incident Response

### If You Suspect a Breach

1. **Change your master password** immediately
2. **Rotate all credentials** in the vault
3. **Check for unauthorized access** to your accounts
4. **Report the incident** to security@example.com
5. **Monitor your accounts** for suspicious activity

### Post-Incident

- Review what happened
- Update security practices
- Consider using 2FA (future)
- Enable audit logging (future)

## Future Security Features

- [ ] Two-factor authentication
- [ ] Audit logging
- [ ] Auto-lock timeout
- [ ] Hardware security key support
- [ ] Secure enclave integration
- [ ] Biometric authentication

## Contact

For security concerns, please contact:

- **Email**: security@example.com
- **PGP Key**: [Available on Keybase](https://keybase.io/yourusername)

## Acknowledgments

We thank the security research community for their responsible disclosure.

## License

This security policy is licensed under [MIT](LICENSE-MIT).
