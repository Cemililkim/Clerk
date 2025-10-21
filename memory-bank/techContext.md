# Technical Context: Clerk

## Technology Stack

### Frontend
- **Framework**: React 18.3.1
- **Language**: TypeScript 5.9.3 (strict mode)
- **Build Tool**: Vite 6.4.0
- **Styling**: CSS Modules (no CSS-in-JS)
- **Testing**: Jest 29.7.0 + React Testing Library
- **Linting**: ESLint 9.37.0 with security plugin
- **Formatting**: Prettier 3.6.2

### Backend
- **Framework**: Tauri 2.8.4
- **Language**: Rust (Edition 2021, MSRV 1.77.2)
- **Database**: SQLite via rusqlite 0.32.1
- **Encryption**: ring 0.17 (AES-256-GCM)
- **Key Derivation**: argon2 0.5 (Argon2id)
- **Memory Safety**: zeroize 1.7
- **Date/Time**: chrono 0.4
- **Error Handling**: thiserror 2.0

### Development Tools
- **Package Manager**: pnpm 10.18.3
- **Node Version**: v20+ (LTS)
- **Rust Version**: 1.77.2+
- **Git**: Version control
- **VSCode**: Recommended IDE

## Development Environment Setup

### Prerequisites
```bash
# Node.js (via nvm or direct install)
node --version  # v20+

# pnpm
npm install -g pnpm

# Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version  # 1.77.2+

# Tauri CLI (optional, pnpm handles it)
cargo install tauri-cli
```

### Project Setup
```bash
# Clone repository
git clone https://github.com/cemililkim/clerk.git
cd clerk

# Install dependencies
pnpm install

# Development server (Vite + Tauri)
pnpm tauri:dev

# Type checking
pnpm type-check

# Linting
pnpm lint

# Testing
pnpm test

# Production build
pnpm tauri:build
```

### Directory Structure
```
clerk/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── hooks/              # Custom React hooks
│   ├── services/           # Business logic
│   ├── types/              # TypeScript definitions
│   ├── utils/              # Helper functions
│   ├── styles/             # Global CSS
│   ├── App.tsx             # Root component
│   └── main.tsx            # Entry point
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands
│   │   ├── crypto/         # Encryption logic
│   │   ├── vault/          # Business logic
│   │   ├── db/             # Database layer
│   │   ├── lib.rs          # Library entry
│   │   └── main.rs         # Binary entry
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── build.rs            # Build script
├── tests/                  # Frontend tests
│   ├── unit/
│   └── integration/
├── memory-bank/            # Cline's memory files
├── docs/                   # Documentation
├── package.json            # Node dependencies
├── tsconfig.json           # TypeScript config
├── vite.config.ts          # Vite config
├── eslint.config.js        # ESLint config
├── .prettierrc             # Prettier config
└── README.md               # Project overview
```

## Configuration Files

### package.json
```json
{
  "name": "clerk",
  "version": "0.1.0",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "test": "jest",
    "lint": "eslint src --ext ts,tsx",
    "format": "prettier --write \"src/**/*.{ts,tsx,css}\"",
    "type-check": "tsc --noEmit"
  }
}
```

### tsconfig.json
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "paths": {
      "@/*": ["./src/*"],
      "@components/*": ["./src/components/*"],
      "@services/*": ["./src/services/*"],
      "@hooks/*": ["./src/hooks/*"],
      "@types/*": ["./src/types/*"],
      "@utils/*": ["./src/utils/*"]
    }
  }
}
```

### Cargo.toml
```toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"
rust-version = "1.77.2"

[dependencies]
tauri = { version = "2.8.5", features = [] }
tauri-plugin-log = "2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ring = "0.17"
argon2 = "0.5"
zeroize = "1.7"
base64 = "0.22"
rusqlite = { version = "0.32", features = ["bundled"] }
thiserror = "2.0"
chrono = "0.4"
log = "0.4"
```

### tauri.conf.json
```json
{
  "productName": "Clerk",
  "version": "0.1.0",
  "identifier": "com.clerk.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420"
  },
  "app": {
    "windows": [{
      "title": "Clerk - Secure Environment Manager",
      "width": 1200,
      "height": 800,
      "minWidth": 800,
      "minHeight": 600
    }]
  }
}
```

## Technical Constraints

### Performance Requirements
- **Vault Unlock**: <1 second on modern hardware
- **Search**: <100ms for 10,000 variables
- **Memory**: <50MB idle, <200MB under load
- **Startup**: <2 seconds cold start

### Security Requirements
- **Encryption**: AES-256-GCM (NIST approved)
- **Key Derivation**: Argon2id (OWASP recommended)
  - Memory: 64MB
  - Iterations: 3
  - Parallelism: 4 lanes
- **Password Policy**: Minimum 8 characters (enforced in UI)
- **Session**: Auto-lock after 15 minutes inactivity
- **Memory**: Zero sensitive data on drop (zeroize crate)

### Platform Support
- **Windows**: 10+ (x64, ARM64)
- **macOS**: 10.15+ (Intel, Apple Silicon)
- **Linux**: Ubuntu 20.04+, Debian 11+, Fedora 36+

## Build & Release Process

### Development Build
```bash
pnpm tauri:dev
# Hot reload enabled
# DevTools available (F12)
# Debug symbols included
```

### Production Build
```bash
pnpm tauri:build
# Optimized binary
# Code signing (platform-specific)
# Installer generation
# Output: src-tauri/target/release/bundle/
```

### Testing Workflow
```bash
# Frontend unit tests
pnpm test

# Frontend with coverage
pnpm test --coverage

# Rust unit tests
cd src-tauri && cargo test

# Rust with coverage
cargo tarpaulin --out Html

# Type checking
pnpm type-check

# Linting
pnpm lint
```

## Dependency Management

### Frontend Dependencies
- **Production**: React, @tauri-apps/api
- **Development**: Vite, TypeScript, ESLint, Prettier, Jest
- **Update Strategy**: Patch updates weekly, minor updates monthly

### Backend Dependencies
- **Critical**: ring, argon2, rusqlite (pin versions)
- **Non-Critical**: chrono, thiserror (allow minor updates)
- **Audit**: `cargo audit` on every PR

## Known Limitations

### Technical Debt
1. **Vault Format**: Version 1 schema (plan migrations for v2)
2. **CLI Binary**: Separate from GUI (code duplication risk)
3. **Testing**: <80% coverage on crypto module (WIP)

### Platform-Specific Issues
- **Windows**: Requires WebView2 runtime (bundled in installer)
- **macOS**: Requires codesigning for distribution
- **Linux**: Multiple packaging formats (AppImage, deb, rpm)

### Performance Bottlenecks
- **Large Vaults**: SQLite query time increases with >10k variables
- **Bulk Import**: Single-threaded encryption (parallelize in v2)
- **Argon2id**: CPU-intensive (consider caching derived keys)

## Tool Usage Patterns

### Git Workflow
```bash
# Feature branch
git checkout -b feature/vault-export

# Commit with conventional commits
git commit -m "feat(vault): add export command"

# Pull request to main
```

### Code Review Checklist
- [ ] TypeScript strict mode errors: 0
- [ ] ESLint warnings: 0
- [ ] Tests pass: All green
- [ ] Test coverage: ≥80% for new code
- [ ] Rust clippy warnings: 0
- [ ] Documentation: Updated for new features

### Release Checklist
1. Update version in package.json and Cargo.toml
2. Update CHANGELOG.md
3. Run full test suite
4. Build for all platforms
5. Test installers on fresh VMs
6. Tag release in Git
7. Publish GitHub release with binaries
