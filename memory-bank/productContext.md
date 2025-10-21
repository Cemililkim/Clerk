# Product Context: Clerk

## Why This Project Exists

### The Problem
Developers face significant challenges managing environment variables across multiple projects:

1. **Security Risks**
   - `.env` files stored in plaintext on disk
   - Accidentally committed to Git repositories
   - Shared via insecure channels (email, Slack, Discord)
   - No encryption at rest

2. **Organization Chaos**
   - Scattered `.env` files across dozens of projects
   - Multiple environments (dev/staging/prod) per project
   - Duplicate variables across projects
   - No central source of truth

3. **Developer Friction**
   - Manual copy-paste between projects
   - Forgotten where specific variables are stored
   - Time wasted searching through old projects
   - Context switching overhead

4. **Collaboration Challenges**
   - Team members need to share secrets securely
   - Onboarding requires manual secret transfer
   - No standardized secret management workflow

### The Solution: Clerk
A **local-first, encrypted vault** specifically designed for developers who need:
- Secure storage with military-grade encryption
- Fast access via CLI and GUI
- Hierarchical organization (Project → Environment → Variables)
- Zero dependency on cloud services
- Integration with existing workflows

## How It Should Work

### First-Time Setup
1. User launches Clerk desktop app
2. Creates master password (with strength indicator)
3. Vault is created in `~/.clerk/` directory
4. Salt and encrypted metadata stored in `vault.clerk` file

### Daily Workflow (GUI)
1. Open Clerk → Enter master password
2. Browse projects in sidebar
3. Select environment (dev/staging/prod)
4. View/edit variables in main panel
5. Search across all projects
6. Export `.env` file for specific environment

### Daily Workflow (CLI)
```bash
# In project directory
clerk init myapp          # Create project in vault
clerk set API_KEY xxx     # Store variable in current project
clerk get API_KEY         # Retrieve variable value
clerk export --env dev    # Generate .env file for dev environment
clerk list                # Show all variables in current project
```

### Security Model
```
Master Password
    ↓ (Argon2id)
Encryption Key (32 bytes)
    ↓ (AES-256-GCM)
Encrypted SQLite Database
    ├── Projects
    ├── Environments
    └── Variables (encrypted values)
```

## User Experience Goals

### Speed
- Vault unlock in <1 second
- Variable search results in <100ms
- CLI commands complete in <200ms
- GUI feels native and responsive

### Simplicity
- Onboarding in <5 minutes
- Intuitive hierarchy: Project → Environment → Variable
- Minimal configuration required
- Clear error messages

### Security
- Never stores plaintext passwords/keys
- Master password never written to disk
- Session tokens in OS keychain
- Automatic lock after inactivity
- Clipboard cleared after paste

### Reliability
- Data corruption recovery
- Automatic backups before destructive operations
- Undo for accidental deletions
- Import/export for migration

## User Stories

### Story 1: New User Setup
**As a** developer new to Clerk  
**I want to** create a secure vault quickly  
**So that** I can start managing my secrets safely

**Acceptance Criteria:**
- Master password creation with strength validation
- Clear guidance on password requirements
- Vault created successfully with confirmation message
- Immediate access to empty vault state

### Story 2: Daily Variable Access
**As a** developer working on multiple projects  
**I want to** quickly find and copy environment variables  
**So that** I can configure my applications without searching files

**Acceptance Criteria:**
- Search across all projects and environments
- Copy variable value with one click
- Keyboard shortcuts for power users
- Recent/favorites for frequently accessed variables

### Story 3: Environment Export
**As a** developer deploying to different environments  
**I want to** generate .env files for specific environments  
**So that** I can deploy with correct configuration

**Acceptance Criteria:**
- Select environment (dev/staging/prod)
- Export to .env file format
- Validate all required variables present
- CLI command for scripting

### Story 4: Secure Sharing
**As a** team lead onboarding new developers  
**I want to** export encrypted vault snapshots  
**So that** I can securely share secrets with team members

**Acceptance Criteria:**
- Export vault with separate password
- Import vault from file
- Merge imported projects into existing vault
- Audit log of imports/exports

## Design Principles

1. **Local-First**: User owns their data, no cloud dependencies
2. **Zero-Trust**: Encrypt everything, trust nothing
3. **Developer-Centric**: CLI-first design, GUI for convenience
4. **Unix Philosophy**: Do one thing well (secret management)
5. **Fast by Default**: Performance over features
6. **Fail-Safe**: Never lose data, always have recovery path

## Anti-Patterns to Avoid
- ❌ Cloud sync (adds complexity and attack surface)
- ❌ Web dashboard (introduces network vulnerabilities)
- ❌ Plugin system (increases maintenance burden)
- ❌ Complex permissions (single-user focus)
- ❌ Auto-update secrets (manual control preferred)
