# Security Policy

Clerk takes security seriously. This document outlines our security practices and how to report vulnerabilities.

**Version:** 1.0.0  
**Last Updated:** October 20, 2025

---

## 🔒 Security Architecture

### Encryption

# Security Policy

Clerk takes security seriously. This document outlines our security practices and how to report vulnerabilities.

**Version:** 1.0.0  
**Last Updated:** October 21, 2025

---

## 🔒 Security Architecture

### Encryption

- **Algorithm**: AES-256-GCM (authenticated encryption)
- **Key Derivation**: Argon2id (memory-hard, GPU-resistant)
- **Salt**: Unique 16-byte random salt per vault
- **Implementation**: Rust's `ring` crate (audited cryptography library)

### Data Storage

- **Database**: SQLite with encrypted values
- **Vault Location (example)**: `%APPDATA%\Clerk\` (Windows)
- **Key Storage**: Windows Credential Manager (OS-level encryption)
- **Backup**: User-controlled, encrypted with same key

### Password Security

- **Hashing**: Argon2id with recommended parameters
- **Master Password**: Never stored, only password hash
- **Auto-Lock**: 5-60 minutes of inactivity (configurable)
- **Memory**: Zeroized immediately after use (Rust `zeroize` crate)

---

## 🛡️ Security Features

- ✅ **Local-First Architecture** — all data stays on device; no cloud sync by default
- ✅ **Audit Logging** — encrypted operation logs with timestamps
- ✅ **Update Verification** — releases are distributed via GitHub Releases with checksums

### Session & CLI notes

- CLI session cache is stored in a temporary file named like `.clerk_session-{hash}` in the OS temp directory. The file contains the vault-derived encryption context and is intended to be readable only by the current user. It is removed on explicit `clerk lock` or when the session expires.
- Avoid storing vault master passwords in scripts or CI. Use the CLI session flow for automation or provide environment variables to secure runners.

---

## 🔐 Threat Model

### What Clerk Protects Against

- **File System Access** — Variables encrypted at rest and require master password to decrypt
- **Memory Dumps** — Sensitive data zeroized after use; minimized exposure time in memory
- **Offline Attacks** — Argon2id slows brute force; unique salt prevents rainbow-table attacks

### What Clerk Does NOT Protect Against

- **Malware on Your Device** — If the host is compromised (keyloggers, rootkits), secrets may be exposed
- **Physical Access to an Unlocked Vault** — If the vault is left unlocked and an attacker has physical access
- **Weak Master Passwords or Social Engineering**

---

## 🚨 Reporting Vulnerabilities

If you discover a security vulnerability, please report it responsibly:

### DO NOT:
- Open a public GitHub issue or post in public forums

### DO:
- Email: **cemililkimteke5934@gmail.com**
- Subject: `[SECURITY] Clerk Vulnerability Report`
- Include: steps to reproduce, impact, suggested fix (optional)

### Response Targets
- **Acknowledgment**: within 48 hours
- **Initial assessment**: within 7 days

We follow a responsible disclosure policy and will publish advisories after fixes are available. Reporters will be credited unless they request anonymity.

---

## 🔧 Security Best Practices

### For Users

1. Use a strong, unique master password (12+ chars). Consider using a password manager.
2. Enable auto-lock and lock the vault when not in use.
3. Backup vaults securely and test restores.
4. Keep the OS and Clerk updated.

### For Developers

1. Review security-critical code (crypto, auth, data handling).
2. Keep dependencies updated and run `cargo audit` / `npm audit` regularly.
3. Avoid logging secrets or including hardcoded keys.

---

## 📋 Security Checklist (for PRs)

- Input validation and sanitization
- No hardcoded secrets
- Proper error handling (fail securely)
- Memory cleanup for sensitive data
- Audit logging for security operations
- No debug logging of secrets

---

## 🔍 Security Audits

- **Self-audit**: October 2025
- **Independent audit / pen test**: planned (timeline depends on budget)

---

## 📚 Cryptography Details

| Component | Algorithm | Library | Notes |
|-----------|-----------|---------|-------|
| Encryption | AES-256-GCM | ring | AEAD (authenticated) |
| Key Derivation | Argon2id | argon2 | Memory-hard |
| Random | OS CSPRNG | ring | Cryptographically secure |

---

## 📞 Contact

- **Security Issues**: cemililkimteke5934@gmail.com
- **General Issues**: https://github.com/Cemililkim/Clerk/issues

---

*Last updated: October 21, 2025*  
*Clerk v1.1.0 - Free & Open Source*
3. **Regular Backups**
