# Security Policy

Clerk takes security seriously. This document outlines our security practices and how to report vulnerabilities.

**Version:** 1.0.0  
**Last Updated:** October 20, 2025

---

## 🔒 Security Architecture

### Encryption

- **Algorithm**: AES-256-GCM (authenticated encryption)
- **Key Derivation**: Argon2id (memory-hard, GPU-resistant)
- **Salt**: Unique 16-byte random salt per vault
- **Implementation**: Rust's `ring` crate (audited cryptography library)

### Data Storage

- **Database**: SQLite with encrypted values
- **Vault Location**: 
  - Windows: `%APPDATA%\com.clerk.app\`
- **Key Storage**: Windows Credential Manager (OS-level encryption)
- **Backup**: User-controlled, encrypted with same key

### Password Security

- **Hashing**: Argon2id with recommended parameters
- **Master Password**: Never stored, only password hash
- **Auto-Lock**: 5-60 minutes of inactivity (configurable)
- **Memory**: Zeroized immediately after use (Rust `zeroize` crate)

---

## 🛡️ Security Features

### Local-First Architecture

- ✅ **No cloud sync** - all data stays on your device
- ✅ **Fully offline** - no internet connection required
- ✅ **No telemetry** - zero data collection
- ✅ **No analytics** - complete privacy

### Audit Logging

- All vault operations logged with timestamps
- Logs stored encrypted in vault database
- View history in Settings → Audit Log

### Update Security

- Manual update checks (no automatic connections)
- Updates downloaded from GitHub Releases
- SHA-256 checksums provided for verification
- No auto-update (user controlled)

---

## 🔐 Threat Model

### What Clerk Protects Against

✅ **File System Access**
- Variables encrypted at rest
- Cannot be read without master password

✅ **Memory Dumps**
- Sensitive data zeroized after use
- Minimized exposure time in memory

✅ **Offline Attacks**
- Argon2id makes brute-force expensive
- Unique salt prevents rainbow tables

✅ **Shoulder Surfing**
- Auto-lock after inactivity
- Hidden values by default in UI

### What Clerk Does NOT Protect Against

❌ **Malware on Your Device**
- If your system is compromised, Clerk cannot protect you
- Keyloggers can capture master password
- Memory scrapers can extract decryption keys

❌ **Physical Access to Unlocked Vault**
- If attacker has access while vault is unlocked
- Use auto-lock and lock manually when done

❌ **Weak Master Passwords**
- Short or common passwords can be brute-forced
- Use strong, unique passwords (12+ characters, mixed case, numbers, symbols)

❌ **Social Engineering**
- Attacker tricking you into revealing password
- Never share your master password

---

## 🚨 Reporting Vulnerabilities

If you discover a security vulnerability, please report it responsibly:

### Do NOT:
- ❌ Open a public GitHub issue
- ❌ Post in discussions or forums
- ❌ Exploit the vulnerability

### DO:
- ✅ Email: **cemililkimteke5934@gmail.com**
- ✅ Subject: `[SECURITY] Clerk Vulnerability Report`
- ✅ Include:
  - Description of the vulnerability
  - Steps to reproduce
  - Potential impact
  - Suggested fix (if any)

### Response Time

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-3 days
  - High: 1-2 weeks
  - Medium: 2-4 weeks
  - Low: Next release

### Disclosure Policy

- We follow **responsible disclosure**
- Security advisories published after fix is released
- Credit given to reporters (unless anonymity requested)

---

## 🔧 Security Best Practices

### For Users

1. **Use a Strong Master Password**
   - 12+ characters
   - Mix of uppercase, lowercase, numbers, symbols
   - Unique (not used elsewhere)
   - Consider using a password manager

2. **Enable Auto-Lock**
   - Set reasonable timeout (5-15 minutes)
   - Lock manually when stepping away

3. **Regular Backups**
   - Backup vault regularly
   - Store backup securely (encrypted, separate location)
   - Test restore process

4. **Keep Software Updated**
   - Install updates promptly
   - Check for updates regularly (Settings → Software Updates)

5. **Secure Your System**
   - Use antivirus/anti-malware
   - Keep OS and all software updated
   - Use firewall
   - Don't install untrusted software

### For Developers

1. **Code Review**
   - All security-critical code reviewed
   - Focus on crypto, auth, data handling

2. **Dependency Management**
   - Use vetted, audited libraries
   - Regular dependency updates
   - Check for known vulnerabilities

3. **Build Security**
   - Reproducible builds
   - Signed releases (planned)
   - Checksum verification

---

## 📋 Security Checklist

When contributing security-sensitive code:

- [ ] Input validation and sanitization
- [ ] No hardcoded secrets or keys
- [ ] Proper error handling (fail securely)
- [ ] Memory cleanup (zeroize sensitive data)
- [ ] Audit logging for security operations
- [ ] No debug logging of sensitive data
- [ ] Tested against common attack vectors

---

## 🔍 Security Audits

### Current Status

- ✅ **Self-Audit**: October 2025 (v1.0.0)
- ⏳ **Independent Audit**: Not yet conducted
- ⏳ **Penetration Testing**: Not yet conducted

### Planned

- Professional security audit (when budget allows)
- Bug bounty program (future consideration)
- Regular code reviews

---

## 📚 Cryptography Details

### Algorithms Used

| Component | Algorithm | Library | Notes |
|-----------|-----------|---------|-------|
| Encryption | AES-256-GCM | ring | AEAD (authenticated) |
| Key Derivation | Argon2id | argon2 | Memory-hard |
| Password Hashing | Argon2id | argon2 | Same as KDF |
| Random | OS CSPRNG | ring | Cryptographically secure |

### Why These Choices?

- **AES-256-GCM**: Industry standard, hardware-accelerated, authenticated
- **Argon2id**: Winner of Password Hashing Competition, memory-hard (GPU-resistant)
- **ring**: Audited, used by Google, AWS, CloudFlare
- **argon2**: Reference implementation, widely trusted

---

## 🏗️ Secure Development

### Code Review Process

1. All PRs reviewed before merge
2. Security-critical changes require 2+ reviewers
3. Automated checks (linting, type checking)

### Testing

- Unit tests for crypto functions
- Integration tests for vault operations
- Manual testing of security features

### Dependencies

- Minimal dependencies
- Prefer audited, well-maintained libraries
- Regular `npm audit` and `cargo audit`

---

## 📊 Known Limitations

1. **Windows Only**
   - Currently only Windows is officially supported
   - macOS/Linux support is experimental

2. **Single User**
   - No multi-user or team features
   - One vault per system (by design)

3. **No Cloud Sync**
   - Intentional design choice
   - Manual backup/restore only

4. **CLI Security**
   - CLI stores password temporarily in session file
   - Session file readable only by current user
   - Deleted on logout or system restart

---

## 🔗 Resources

- **OWASP Cheat Sheets**: https://cheatsheetseries.owasp.org/
- **CWE Top 25**: https://cwe.mitre.org/top25/
- **Tauri Security**: https://tauri.app/v1/references/architecture/security/
- **Rust Security**: https://anssi-fr.github.io/rust-guide/

---

## 📞 Contact

- **Security Issues**: cemililkimteke5934@gmail.com
- **General Issues**: https://github.com/Cemililkim/Clerk/issues
- **Repository**: https://github.com/Cemililkim/Clerk

---

*Last updated: October 20, 2025*  
*Clerk v1.0.0 - Free & Open Source*
