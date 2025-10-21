# Contributing to Clerk# Contributing to Clerk



Thank you for your interest in contributing to Clerk! 🎉Thank you for your interest in Clerk! This document provides guidelines for contributing to the project.



------



## 🚀 Getting Started## 🎯 Project Philosophy



### PrerequisitesClerk is built on these core principles:

- Node.js 18+

- pnpm (package manager)1. **Security First**: Every decision prioritizes user data protection

- Rust 1.77+2. **Local-First**: User data lives on their machine by default

- Windows 10/11 (64-bit)3. **Developer Experience**: Tools should be invisible when they work perfectly

4. **Clean Architecture**: Code should be maintainable, testable, and understandable

### Setup5. **Incremental Improvement**: Ship small, working increments rather than large, risky changes

```bash

git clone https://github.com/Cemililkim/Clerk.git---

cd Clerk

pnpm install## 🏗️ Development Workflow

pnpm tauri dev

```### Setting Up Your Environment



See [BUILD_GUIDE.md](docs/BUILD_GUIDE.md) for detailed build instructions.1. **Prerequisites**:

   - Node.js 18+

---   - pnpm (package manager)

   - Rust and Cargo

## 📝 How to Contribute   - Platform-specific tools (see README.md)



### 1. **Report Bugs**2. **Clone and Install**:

- Check [existing issues](https://github.com/Cemililkim/Clerk/issues) first   ```bash

- Use bug report template   git clone <repository-url>

- Include steps to reproduce   cd clerk

- Add screenshots if applicable   pnpm install

   ```

### 2. **Suggest Features**

- Open an issue with `enhancement` label3. **Verify Setup**:

- Explain the use case   ```bash

- Describe expected behavior   pnpm tauri dev

   ```

### 3. **Submit Code**

### Branch Strategy

**Before starting:**

- Check existing issues/PRs- `main`: Production-ready code

- Discuss major changes first- `develop`: Integration branch for features

- `feature/*`: Individual feature branches

**Development process:**- `fix/*`: Bug fix branches

```bash- `security/*`: Security-related fixes (highest priority)

# Create feature branch

git checkout -b feature/your-feature### Commit Conventions



# Make changesWe follow [Conventional Commits](https://www.conventionalcommits.org/):

# Test thoroughly

pnpm tauri dev```

<type>(<scope>): <subject>

# Build and verify

pnpm tauri build<body>



# Commit with clear message<footer>

git commit -m "feat: add your feature"```



# Push and create PR**Types**:

git push origin feature/your-feature- `feat`: New feature

```- `fix`: Bug fix

- `security`: Security-related change

---- `refactor`: Code refactoring without behavior change

- `test`: Adding or updating tests

## 🎯 Code Guidelines- `docs`: Documentation changes

- `chore`: Maintenance tasks

### Commit Messages- `perf`: Performance improvement

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature**Examples**:

- `fix:` - Bug fix```

- `docs:` - Documentation changesfeat(vault): implement AES-256-GCM encryption

- `style:` - Code formattingfix(cli): resolve environment variable injection race condition

- `refactor:` - Code restructuringsecurity(crypto): patch key derivation vulnerability

- `test:` - Adding teststest(vault): add unit tests for encryption module

- `chore:` - Maintenance tasksdocs(readme): update installation instructions

```

**Examples:**

```---

feat: add CLI PATH management

fix: resolve update checker download issue## 🧪 Testing Requirements

docs: update README installation steps

```**All code must be tested before merging.**



### Code Style### Test Coverage Goals

- **TypeScript/React**: Use ESLint configuration- **Critical paths** (encryption, decryption, key management): 100%

- **Rust**: Run `cargo fmt` and `cargo clippy`- **Core features**: 90%+

- **Commits**: Keep focused and atomic- **UI components**: 80%+

- **Utilities**: 85%+

### Testing

- Test your changes thoroughly### Running Tests

- Ensure no regressions

- Include test steps in PR description```bash

# All tests

---pnpm test



## 🔒 Security# Unit tests only

pnpm test:unit

**Found a security vulnerability?**

# Integration tests

**DO NOT** open a public issue!pnpm test:integration



Instead:# With coverage report

1. Email: cemililkimteke5934@gmail.compnpm test:coverage

2. Include:

   - Description of vulnerability# Watch mode during development

   - Steps to reproducepnpm test:watch

   - Potential impact```

3. Wait for response before disclosure

### Writing Tests

See [SECURITY.md](SECURITY.md) for details.

1. **Unit Tests**: Test individual functions/classes in isolation

---2. **Integration Tests**: Test interactions between modules

3. **E2E Tests**: Test complete user workflows (Tauri + CLI)

## 📦 Pull Request Process

**Example**:

1. **Update Documentation**```typescript

   - Update README if needed// Good: Isolated, deterministic, fast

   - Add CHANGELOG entrydescribe('VaultEncryption', () => {

   - Update relevant docs  it('should encrypt and decrypt data correctly', () => {

    const plaintext = 'sensitive-data';

2. **Test Your Changes**    const password = 'strong-password';

   - Run `pnpm tauri dev`    

   - Test all affected features    const encrypted = encrypt(plaintext, password);

   - Build successfully: `pnpm tauri build`    const decrypted = decrypt(encrypted, password);

    

3. **Submit PR**    expect(decrypted).toBe(plaintext);

   - Clear title and description    expect(encrypted).not.toContain(plaintext);

   - Link related issues  });

   - Add screenshots/videos for UI changes});

```

4. **Review Process**

   - Address review comments---

   - Keep PR updated with main branch

   - Be patient and respectful## 🔐 Security Guidelines



---### Critical Rules



## 💡 Development Tips1. **Never log sensitive data** (passwords, keys, decrypted secrets)

2. **Always validate input** from users and external sources

### Project Structure3. **Use timing-safe comparisons** for cryptographic operations

```4. **Wipe sensitive data from memory** after use

Clerk/5. **Follow OWASP guidelines** for encryption and key management

├── src/              # Frontend (React + TypeScript)

├── src-tauri/        # Backend (Rust + Tauri)### Security Review Checklist

│   ├── src/

│   │   ├── commands/ # Tauri commandsBefore submitting code that touches security:

│   │   ├── crypto/   # Encryption logic

│   │   ├── database/ # SQLite operations- [ ] No plaintext secrets written to disk

│   │   └── vault/    # Vault management- [ ] Sensitive data cleared from memory after use

└── docs/             # Documentation- [ ] Input validation on all external data

```- [ ] Constant-time comparisons for security-critical checks

- [ ] No sensitive data in logs or error messages

### Useful Commands- [ ] Encryption uses approved algorithms (AES-256-GCM)

```bash- [ ] Key derivation uses proper KDF (PBKDF2/Argon2)

# Development

pnpm dev              # Start Vite dev server### Reporting Security Issues

pnpm tauri dev        # Run Tauri app in dev mode

**DO NOT open public issues for security vulnerabilities.**

# Building

pnpm build            # Build frontendContact: cemililkimteke5934@gmail.com with:

pnpm tauri build      # Build full application- Description of the vulnerability

- Steps to reproduce

# Testing- Potential impact

cargo test            # Run Rust tests- Suggested fix (if any)

cargo clippy          # Run linter

cargo fmt             # Format code---

```

## 🎨 Code Style

### Debugging

- **Frontend**: Open DevTools with `F12`### General Principles

- **Backend**: Check console output in terminal

- **Logs**: Located in `%APPDATA%\com.clerk.app\logs\`- **Readability over cleverness**: Code is read more than written

- **Explicit over implicit**: Clear intent beats brevity

---- **Type safety**: Leverage TypeScript's type system

- **No magic numbers**: Use named constants

## 🎓 Resources- **Pure functions**: Prefer pure, deterministic functions



- [Tauri Documentation](https://tauri.app/v1/guides/)### TypeScript/JavaScript

- [React Documentation](https://react.dev/)

- [Rust Book](https://doc.rust-lang.org/book/)```typescript

- [SQLite Documentation](https://www.sqlite.org/docs.html)// ✅ Good

const MAX_RETRY_ATTEMPTS = 3;

---

async function decryptVault(

## 📜 License  encryptedData: Uint8Array,

  password: string

By contributing to Clerk, you agree that your contributions will be licensed under the same license as the project.): Promise<DecryptedVault> {

  validatePassword(password);

---  const key = await deriveKey(password);

  return decrypt(encryptedData, key);

## 🙏 Thank You!}



Every contribution helps make Clerk better. Whether it's code, documentation, bug reports, or feature suggestions - we appreciate your time and effort! ❤️// ❌ Avoid

async function dec(d: any, p: any) {

**Questions?** Feel free to open an issue or reach out!  return decrypt(d, deriveKey(p, 3));

}

---```



**Happy Coding!** 🚀### Rust (Tauri Backend)


```rust
// ✅ Good
pub fn encrypt_vault(plaintext: &[u8], key: &Key) -> Result<Vec<u8>, VaultError> {
    validate_key(key)?;
    // ... encryption logic
}

// ❌ Avoid
pub fn enc(d: &[u8], k: &Key) -> Vec<u8> {
    // ... encryption logic, no error handling
}
```

### File Naming

- React components: `PascalCase.tsx` (e.g., `VaultManager.tsx`)
- Utilities: `camelCase.ts` (e.g., `cryptoUtils.ts`)
- Tests: `*.test.ts` or `*.spec.ts`
- Types: `types.ts` or `interfaces.ts`

---

## 📁 Project Structure Guidelines

```
src/
├── components/          # React UI components
│   ├── vault/          # Vault-related components
│   ├── settings/       # Settings UI
│   └── common/         # Reusable components
├── services/           # Business logic layer
│   ├── vaultService.ts
│   ├── cryptoService.ts
│   └── keychainService.ts
├── hooks/              # Custom React hooks
├── utils/              # Pure utility functions
├── types/              # TypeScript type definitions
└── tests/              # Test files

src-tauri/
├── src/
│   ├── vault/          # Vault management (Rust)
│   ├── crypto/         # Cryptography (Rust)
│   └── commands.rs     # Tauri commands
└── Cargo.toml

cli/
├── src/
│   ├── commands/       # CLI command implementations
│   ├── utils/          # CLI utilities
│   └── index.ts        # Entry point
└── package.json
```

---

## 🚀 Pull Request Process

1. **Create a feature branch** from `develop`
2. **Write code** following guidelines above
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Run all tests** locally: `pnpm test`
6. **Run linter**: `pnpm lint`
7. **Commit** with conventional commit messages
8. **Push** and create Pull Request to `develop`

### PR Checklist

- [ ] Code follows style guidelines
- [ ] Tests added and passing
- [ ] No decrease in test coverage
- [ ] Documentation updated (if applicable)
- [ ] Security checklist completed (if applicable)
- [ ] Self-review completed
- [ ] No console.log or debug code left

### Review Process

- All PRs require review before merging
- Security-related PRs require extra scrutiny
- CI/CD must pass (tests, linting, build)

---

## 📚 Additional Resources

- [Project Memory Bank](./memory-bank/): Context and architectural decisions
- [Code Guide](./CODE_GUIDE.md): Detailed coding patterns and examples
- [Tauri Docs](https://tauri.app/): Tauri framework documentation
- [React Docs](https://react.dev/): React library documentation

---

## ❓ Questions?

Contact Cemil İlkim Teke:
- Email: cemililkimteke5934@gmail.com
- GitHub: @cemililkim

---

**Remember**: Every line of code is a commitment to our users' security. Code carefully, test thoroughly, and never compromise on security.
