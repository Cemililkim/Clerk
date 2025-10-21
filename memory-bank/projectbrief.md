# Project Brief: Clerk

## Project Overview
**Clerk** is a secure, local-first desktop application for managing environment variables across multiple projects and environments. Built with Tauri (Rust + React + TypeScript), it provides military-grade encryption and a developer-friendly interface.

## Core Requirements

### Primary Goals
1. **Security First**: All data encrypted with AES-256-GCM
2. **Local-First**: No cloud dependencies, data stays on user's machine
3. **Developer UX**: Fast, intuitive interface for daily workflows
4. **Cross-Platform**: Windows, macOS, Linux support via Tauri

### Key Features
1. **Encrypted Vault**
   - Master password protection
   - Argon2id key derivation
   - AES-256-GCM encryption for all stored data
   - OS keychain integration for session tokens

2. **Hierarchical Organization**
   - Projects (e.g., "MyApp")
   - Environments per project (dev, staging, production)
   - Variables per environment (KEY=value pairs)
   - Support for nested/grouped variables

3. **CLI Tool**
   - `clerk init` - Initialize vault in project directory
   - `clerk get VAR_NAME` - Retrieve variable value
   - `clerk set VAR_NAME value` - Store variable
   - `clerk export --env production` - Export .env file
   - Integration with shell scripts and CI/CD

4. **Desktop GUI**
   - Visual vault management
   - Search and filter capabilities
   - Bulk import/export (.env files)
   - Variable history and versioning

### Technical Constraints
- **Framework**: Tauri 2.x (Rust backend + React frontend)
- **Language**: TypeScript (frontend), Rust (backend)
- **Database**: SQLite (encrypted)
- **Encryption**: ring crate for AES-256-GCM
- **Key Derivation**: Argon2id with OWASP parameters
- **Testing**: Jest (frontend), Rust built-in tests (backend)
- **Code Quality**: ESLint, Prettier, Clippy

### Non-Goals (Out of Scope)
- Cloud sync or backup features
- Multi-user collaboration
- Web-based interface
- Mobile app versions
- Integration with external secret managers (HashiCorp Vault, AWS Secrets Manager)

## Success Criteria
1. User can create encrypted vault with master password
2. User can manage projects/environments/variables through GUI
3. CLI tool can read/write variables for scripting
4. All sensitive data encrypted at rest
5. Application passes security audit (no plaintext storage)
6. Test coverage ≥80% for critical paths

## User Persona
**Primary User: Full-Stack Developer**
- Manages 3-10 projects simultaneously
- Switches between dev/staging/production environments frequently
- Uses both GUI (for browsing) and CLI (for automation)
- Security-conscious, wants local control of secrets
- Frustrated with .env files scattered across projects

## Project Context
- **Project Name**: Clerk
- **Developer**: Cemil İlkim Teke (@cemililkim)
- **Email**: cemililkimteke5934@gmail.com
- **Version**: 0.1.0 (Initial Development)
- **License**: MIT
- **Status**: Active Development (Phase 1: Core Infrastructure)

## Development Phases
1. **Phase 1: Foundation** (Current)
   - Encryption infrastructure
   - Vault creation/unlock
   - Basic CRUD operations
   
2. **Phase 2: GUI Development**
   - React components for vault management
   - Project/environment hierarchy UI
   - Search and filtering
   
3. **Phase 3: CLI Tool**
   - Command-line interface
   - Shell integration
   - CI/CD compatibility
   
4. **Phase 4: Polish & Release**
   - Comprehensive testing
   - Security audit
   - Documentation
   - Installer/packaging
