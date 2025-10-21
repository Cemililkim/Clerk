# System Patterns: Clerk Architecture

## Architecture Overview

### High-Level Architecture
```
┌─────────────────────────────────────────────┐
│           Desktop Application               │
│  ┌─────────────┐        ┌────────────────┐  │
│  │   React UI  │ ←IPC→  │  Rust Backend  │  │
│  │ (TypeScript)│        │   (Tauri)      │  │
│  └─────────────┘        └────────────────┘  │
│                              ↓              │
│                    ┌──────────────────┐     │
│                    │ Encrypted SQLite │     │
│                    │  (~/.clerk/)     │     │
│                    └──────────────────┘     │
└─────────────────────────────────────────────┘
                    ↓
         ┌──────────────────┐
         │   CLI Binary     │
         │  (Separate Rust) │
         └──────────────────┘
```

## Core Components

### 1. Frontend (React + TypeScript)
**Location**: `src/`

**Key Components:**
- `App.tsx` - Root component, routing logic
- `components/` - Reusable UI components
  - `VaultCreation/` - Master password setup
  - `VaultUnlock/` - Authentication flow
  - `ProjectList/` - Sidebar navigation
  - `VariableEditor/` - Main editing interface
- `services/` - Business logic layer
  - `vaultService.ts` - Vault operations
  - `encryptionService.ts` - Crypto wrappers
- `hooks/` - React custom hooks
  - `useVault.ts` - Vault state management
  - `useAuth.ts` - Authentication state
- `types/` - TypeScript definitions

**Patterns:**
- **Component Architecture**: Functional components with hooks
- **State Management**: React Context + useReducer (no Redux)
- **Type Safety**: Strict TypeScript, no `any` types
- **Error Boundaries**: Graceful error handling
- **Code Splitting**: Lazy loading for routes

### 2. Backend (Rust + Tauri)
**Location**: `src-tauri/src/`

**Module Structure:**
```rust
src-tauri/src/
├── lib.rs           // Tauri setup, command registration
├── main.rs          // Binary entry point
├── commands/        // Tauri commands (IPC handlers)
│   ├── mod.rs
│   └── vault.rs     // create_vault, unlock_vault, etc.
├── crypto/          // Encryption implementation
│   ├── mod.rs
│   ├── encryption.rs    // AES-256-GCM
│   └── key_derivation.rs // Argon2id
├── vault/           // Business logic
│   ├── mod.rs
│   └── manager.rs   // VaultManager struct
└── db/              // Database layer
    ├── mod.rs
    ├── schema.rs    // SQLite schema
    └── queries.rs   // CRUD operations
```

**Patterns:**
- **Layered Architecture**:
  - Commands (IPC boundary)
  - Business Logic (vault operations)
  - Data Access (database queries)
  - Crypto (encryption primitives)
- **Error Handling**: `Result<T, E>` everywhere, thiserror for custom errors
- **Zero-Copy**: Minimize allocations, use references
- **Type Safety**: Strong typing, no unsafe code unless necessary

### 3. Cryptography Layer
**Responsibility**: Encrypt/decrypt all sensitive data

**Implementation:**
```rust
// Key Derivation
Master Password + Salt
    ↓ Argon2id (64MB memory, 3 iterations)
32-byte Encryption Key

// Encryption
Plaintext + Key + Nonce
    ↓ AES-256-GCM
Ciphertext + Authentication Tag

// Storage Format
[12-byte Nonce][Ciphertext][16-byte Tag]
```

**Key Functions:**
- `derive_key(password: &str, salt: &[u8; 16]) -> [u8; 32]`
- `encrypt(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8>`
- `decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Vec<u8>`
- `hash_password(password: &str) -> String` (PHC format)
- `verify_password(password: &str, hash: &str) -> bool`

**Security Properties:**
- Authenticated encryption (prevents tampering)
- Unique nonce per encryption (prevents replay)
- Zeroized keys on drop (clears memory)
- Constant-time operations (prevents timing attacks)

### 4. Database Layer
**Technology**: SQLite with `rusqlite` crate

**Schema:**
```sql
-- Vault metadata
CREATE TABLE vault_metadata (
    id INTEGER PRIMARY KEY,
    version INTEGER NOT NULL,
    salt BLOB NOT NULL,          -- 16 bytes for Argon2id
    password_hash TEXT NOT NULL, -- PHC string format
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Projects
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Environments
CREATE TABLE environments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,  -- dev, staging, production
    created_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, name)
);

-- Variables (encrypted values)
CREATE TABLE variables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    environment_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    encrypted_value BLOB NOT NULL,  -- [nonce][ciphertext][tag]
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    UNIQUE(environment_id, key)
);

-- Indices for performance
CREATE INDEX idx_environments_project ON environments(project_id);
CREATE INDEX idx_variables_environment ON variables(environment_id);
CREATE INDEX idx_variables_key ON variables(key);
```

**Patterns:**
- **Connection Pooling**: Single connection per app lifecycle
- **Transactions**: ACID guarantees for multi-table operations
- **Prepared Statements**: SQL injection prevention
- **Foreign Keys**: Referential integrity enabled
- **Encryption**: Only `variables.encrypted_value` is encrypted

## Key Design Decisions

### 1. Why Tauri?
- **Small Bundle Size**: ~3MB vs 100MB+ Electron
- **Memory Efficient**: Rust backend vs Node.js
- **Security**: Native OS webview, no embedded Chromium
- **Performance**: Rust speed for crypto operations
- **Cross-Platform**: Single codebase for Win/Mac/Linux

### 2. Why SQLite?
- **Local-First**: No server required
- **ACID**: Data integrity guarantees
- **Portable**: Single file database
- **Fast**: In-process queries, no network latency
- **Proven**: Battle-tested for 20+ years

### 3. Why Argon2id?
- **Memory-Hard**: Resistant to GPU/ASIC attacks
- **Configurable**: Tune memory/time parameters
- **Winner**: Password Hashing Competition (2015)
- **Standard**: OWASP recommendation for sensitive data

### 4. Why React (not Svelte/Vue)?
- **Ecosystem**: Mature libraries and tooling
- **TypeScript**: First-class support
- **Team Familiarity**: Developer expertise
- **Component Model**: Reusable, testable components

## Critical Implementation Paths

### Path 1: Vault Unlock Flow
```
1. User enters password in UI
2. React calls invoke('unlock_vault', { password })
3. Tauri command reads vault.clerk metadata
4. Rust verifies password hash
5. Rust derives encryption key (Argon2id)
6. Rust caches key in memory (Zeroizing)
7. Rust opens SQLite connection
8. Return success to React
9. React navigates to main interface
```

### Path 2: Variable Retrieval
```
1. User selects project/environment in UI
2. React calls invoke('get_variables', { env_id })
3. Rust queries encrypted_value from SQLite
4. Rust decrypts each value with cached key
5. Rust returns plaintext key-value pairs
6. React displays in VariableEditor component
```

### Path 3: Variable Update
```
1. User edits variable in UI
2. React calls invoke('set_variable', { env_id, key, value })
3. Rust encrypts value with cached key
4. Rust updates/inserts into SQLite
5. Rust commits transaction
6. Return success to React
7. React updates local state
```

## Testing Strategy

### Unit Tests
- **Crypto Functions**: Encrypt/decrypt round-trips
- **Key Derivation**: Deterministic outputs
- **Database Queries**: CRUD operations
- **React Components**: Snapshot tests

### Integration Tests
- **Vault Lifecycle**: Create → Unlock → Lock
- **Variable CRUD**: Create → Read → Update → Delete
- **Error Handling**: Invalid passwords, corrupted data

### Security Tests
- **Memory Leaks**: Zeroization verification
- **Timing Attacks**: Constant-time verification
- **SQL Injection**: Parameterized queries
- **XSS**: Input sanitization

## Performance Targets
- Vault unlock: <1 second
- Variable search: <100ms
- Bulk operations: 1000 variables in <500ms
- Memory footprint: <50MB idle
- Startup time: <2 seconds cold start
