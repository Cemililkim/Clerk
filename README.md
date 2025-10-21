# Clerk

**Secure, Local-First Environment Variable Manager**

Clerk is a modern, cross-platform desktop application that revolutionizes how developers manage sensitive environment variables and secrets. Built with security as the top priority, Clerk stores all your project secrets in a single encrypted vault file, eliminating the risks of scattered `.env` files and git leaks.

---

## 🎯 Vision

Create a free, open-source standard where developers never have to choose between security and productivity when managing sensitive project data.

## 🚀 Key Features

### Current Features ✅
- **🔐 Encrypted Local Vault**: All secrets stored in a single `vault.clerk` file, encrypted with AES-256-GCM
- **🔑 Master Password Protection**: Your vault is only decryptable with your master password
- **💾 OS Keychain Integration**: Seamless integration with macOS Keychain and Windows Credential Manager
- **⚡ Full-Featured CLI**: 19 commands with session management, aliases, and bulk operations
  - Session management (unlock, lock, status)
  - Variable operations (get, set, list, delete, copy)
  - Project/environment management (create, list, delete)
  - Bulk operations (import from .env, export, run with env)
  - Short aliases (g, s, ls, d, cp, pc, pl, pd, ec, el, ed, imp)
- **🛤️ PATH Management**: One-click CLI PATH integration from Settings (Windows)
- **🎨 Multi-Theme System**: 5 beautiful color themes (Purple, Blue, Green, Orange, Pink)
- **🌙 Dark Mode**: Comprehensive dark mode with 1200+ CSS overrides
- **⌨️ Keyboard Shortcuts**: 10 productivity shortcuts with reference panel
- **✅ Smart Validation**: Auto-validate URLs, emails, ports, and JSON with visual feedback
- **📦 Bulk Operations**: Multi-select, bulk delete, and bulk export capabilities
- **📤 Multi-Format Export**: Export to ENV, JSON, or CSV with customizable options
- **📊 Audit Log**: Complete activity tracking with filters, search, and export
- **⏱️ Auto-Lock**: Configurable inactivity timeout (5-60 minutes)
- **💾 Backup & Restore**: Complete vault backup and restore functionality
- **🔄 Software Updates**: Built-in update checker with GitHub releases integration
- **♾️ Unlimited Everything**: No restrictions on projects, environments, or variables
- **✨ Modern UI**: Built with React, smooth animations, loading skeletons, and enhanced empty states

---

## 🏗️ Technology Stack

- **Frontend**: React + TypeScript
- **Desktop Framework**: Tauri 2.8
- **Local Database**: SQLite (encrypted)
- **CLI**: Rust
- **Encryption**: AES-256-GCM + Argon2id

---

## 📥 Download

**Latest Version:** v1.1.0 (Unreleased)

### Windows
- [**Clerk_1.1.0_x64-setup.exe**](https://github.com/Cemililkim/Clerk/releases/download/v1.1.0/Clerk_1.1.0_x64-setup.exe) (Planned)

> **Note:** macOS and Linux builds are planned for future releases.

[**View All Releases →**](https://github.com/Cemililkim/Clerk/releases)

---

## What's new (short)

- Session persistence fixes and improved CLI compatibility with GUI
- New full-featured CLI (19 commands) and CLI Quick Start Guide inside the app
- Encryption interoperability fixes (AAD alignment)

For full details see `RELEASE_NOTES.md` and `CHANGELOG.md`.

---

## Release checklist (for maintainers)

Before publishing a release, follow this checklist:

- [ ] Update `CHANGELOG.md` with any final notes under Unreleased
- [ ] Bump versions in `package.json` / `Cargo.toml`
- [ ] Build release artifacts: `pnpm build && pnpm tauri build`
- [ ] Verify checksums and code signing (if applicable)
- [ ] Create Git tag `vX.Y.Z` and push to GitHub
- [ ] Publish GitHub Release with `RELEASE_NOTES.md` summary and attach installers
- [ ] Announce release in project README and docs

See `RELEASE_NOTES.md` for a concise summary of the latest release.

---

## �🔐 Security Model

Security is non-negotiable in Clerk. Here's how we protect your secrets:

1. **Master Password**: Derived using strong KDF (Key Derivation Function)
2. **AES-256-GCM Encryption**: Industry-standard, authenticated encryption
3. **Zero Plaintext on Disk**: Environment variables are never written as plaintext
4. **Memory Protection**: Decrypted data is wiped from memory after use
5. **End-to-End Encryption (Pro)**: Even in the cloud, only you can decrypt your data

---

## 🛠️ Development Setup

### Prerequisites
- Node.js 18+ and pnpm
- Rust (for Tauri)
- Platform-specific requirements:
  - **Windows**: WebView2, Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: webkit2gtk, dependencies per distro

### Installation

```bash
# Clone the repository
git clone https://github.com/cemililkim/clerk.git
cd clerk

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Project Structure

```
clerk/
├── src/                    # React frontend source
├── src-tauri/              # Tauri backend (Rust)
├── cli/                    # Node.js CLI tool
├── memory-bank/            # Project documentation & context
└── docs/                   # Additional documentation
```

---

## 📖 Usage

### Desktop App

1. **Launch Clerk** - Opens directly to unlock screen (or vault creation on first run)
2. **Create your master password** on first run with strength validation
3. **Add your projects and environment variables** - Organize by project and environment
4. **Manage secrets** with powerful features:
   - 🔍 **Search & Filter** - Find variables instantly across keys and values
   - 📋 **Click to Copy** - One-click copying on variable keys and values
   - 👁️ **Show/Hide Values** - Toggle visibility for security
   - 📥 **Multi-Format Import/Export** - ENV, JSON, CSV support with options
   - 🎨 **5 Theme Colors** - Purple, Blue, Green, Orange, Pink
   - 🌙 **Dark Mode** - Smooth transitions with 1200+ CSS overrides
   - ⚡ **Keyboard Shortcuts** - 10 shortcuts for power users (Ctrl+N, Ctrl+E, etc.)
   - ✅ **Variable Validation** - Auto-validate URLs, emails, ports, JSON with visual feedback
   - 📦 **Bulk Operations** - Multi-select, bulk delete, bulk export
   - 💀 **Loading Skeletons** - Smooth shimmer animations during data loads
   - 📝 **Enhanced Empty States** - Helpful guidance when starting or no results
   - ⚡ **Shift+Click Shortcuts** - Skip confirmations for power users
   - 📊 **Audit Log** - Track all changes with filtering and export
   - ⏱️ **Auto-lock** - Configurable inactivity timeout (5-60 min)
   - ⚙️ **Settings** - Customize appearance, theme, and security preferences

#### Remember Me Feature

Clerk integrates with your operating system's secure credential storage for seamless auto-unlock:

- **Windows**: Uses Windows Credential Manager

**How it works:**
1. When unlocking your vault, check the **"Remember me"** checkbox
2. Your encryption key is securely stored in the OS keychain
3. Next time you open Clerk, it automatically unlocks without prompting for a password
4. Click the **Lock** button to clear the stored key and require password again

**Security Notes:**
- The encryption key (not your master password) is stored in the OS keychain
- Your OS authentication (Windows login etc.) protects the keychain
- Locking the vault completely removes the key from the keychain
- This feature is optional - you can always unlock without checking the box

### CLI Tool

The Clerk CLI provides powerful command-line access to your encrypted vault for automation, scripting, and DevOps workflows.

#### Installation

**Option 1: Automatic (Windows)**
1. Open Clerk Settings (⚙️ or `Ctrl+,`)
2. Go to "CLI Integration" section
3. Click "Add to PATH"
4. Open a new terminal and use `clerk --help`

**Option 2: Manual**
After building, the CLI binary is located at `src-tauri/target/release/clerk.exe` (Windows).

#### Core Features

- 🔐 **Session Management**: Unlock once, run multiple commands without re-entering password
- 📦 **Project/Environment Management**: Full CRUD operations for organizing your secrets
- 🔄 **Variable Operations**: Get, set, list, delete, and copy variables
- 📥 **Bulk Import**: Import from .env files with overwrite control
- 🎯 **Short Aliases**: Speed up your workflow with `g`, `s`, `ls`, `d`, `cp`, `pc`, `pl`, etc.

#### Available Commands

**Session Management:**
```bash
clerk unlock                     # Unlock vault (caches password for session)
clerk unlock -S                  # Skip session cache (always prompt)
clerk lock                       # Clear session cache
clerk status                     # Check session status
```

**Variable Operations:**
```bash
# Get a variable (alias: g)
clerk get DATABASE_URL -p my-app -e prod
clerk g DATABASE_URL -p my-app -e prod

# Set a variable (alias: s)
clerk set API_KEY sk-123... -p my-app -e staging -d "OpenAI API Key"
clerk s PORT 3000 -p my-app -e dev

# List variables (alias: ls)
clerk list -p my-app -e prod --show-values
clerk ls -p my-app

# Delete a variable (alias: d)
clerk delete DATABASE_URL -p my-app -e staging --force
clerk d API_KEY -p my-app -e dev -f

# Copy between environments (alias: cp)
clerk copy DATABASE_URL --from-project my-app --from-env staging \
                       --to-project my-app --to-env prod --overwrite
```

**Project Management:**
```bash
# Create project (alias: pc)
clerk project-create my-app -d "My awesome application"
clerk pc backend-api

# List projects (alias: pl)
clerk project-list
clerk pl

# Delete project (alias: pd)
clerk project-delete my-app --force
clerk pd old-project -f
```

**Environment Management:**
```bash
# Create environment (alias: ec)
clerk env-create production -p my-app -d "Production environment"
clerk ec staging -p my-app

# List environments (alias: el)
clerk env-list -p my-app
clerk el -p backend-api

# Delete environment (alias: ed)
clerk env-delete staging -p my-app --force
clerk ed dev -p my-app -f
```

**Bulk Operations:**
```bash
# Export to .env file
clerk export -p my-app -e prod --output .env.production

# Import from .env file (alias: imp)
clerk import .env.local -p my-app -e dev
clerk imp .env.staging -p my-app -e staging --overwrite

# Initialize new project
clerk init my-new-project -d "New project description"

# Run command with injected variables
clerk run -p my-app -e dev npm start
clerk run -p my-app -e prod python app.py
```

**Global Options:**
```bash
-S, --no-session              # Skip session cache (always prompt for password)
-D, --vault-dir <PATH>        # Use custom vault directory
-h, --help                    # Show help
-V, --version                 # Show version
```

#### Session Management

By default, Clerk caches your password in a temporary file (`TEMP/.clerk_session-{PID}`) for the duration of your terminal session. This allows you to run multiple commands without re-entering your password.

**Benefits:**
- ✅ Each terminal has its own isolated session (process ID)
- ✅ Sessions are automatically cleaned up when terminal closes
- ✅ Invalid passwords automatically clear the session
- ✅ Use `clerk lock` to manually clear session

**Example Workflow:**
```bash
# Unlock once
clerk unlock
# Password: ********
# ✅ Vault unlocked successfully!
# 💾 Session saved for this terminal

# Run multiple commands without password
clerk ls -p my-app
clerk g DATABASE_URL -p my-app -e prod
clerk s NEW_VAR value -p my-app -e dev

# When done, lock the session
clerk lock
# 🔒 Session cleared. You'll need to enter your password for the next command.
```

### Desktop App

1. Launch Clerk
2. Create your master password on first run
3. Add your projects and environment variables
4. Use the intuitive UI to manage secrets across environments


---

## 🤝 Contributing

Contributions are welcome! This is an open-source project built to help the developer community. Whether it's:

- 🐛 Bug reports
- 💡 Feature suggestions
- 📝 Documentation improvements
- 🔧 Code contributions

Feel free to open an issue or submit a pull request. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## 📝 License

MIT License

Copyright © 2025 Cemil İlkim Teke

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

---

## 📧 Contact & Support

**Cemil İlkim Teke**
- 📧 Email: cemililkimteke5934@gmail.com
- 💼 GitHub: [@cemililkim](https://github.com/cemililkim)
- 🐛 Issues: [GitHub Issues](https://github.com/cemililkim/clerk/issues)

---

## ⭐ Show Your Support

If Clerk helps you manage your secrets more securely, please consider:
- ⭐ Starring this repository
- 🐛 Reporting bugs and suggesting features
- 🤝 Contributing to the codebase
- 📢 Sharing with other developers

---

**Built with ❤️ for the developer community**

*Free forever. No subscriptions. No limits.*
