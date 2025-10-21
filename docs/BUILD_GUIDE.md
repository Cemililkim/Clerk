# Clerk Build Guide# Clerk Build Guide



How to build Clerk from source.Complete guide for building Clerk from source.



**Version:** 1.0.0  **Version:** 1.0.0  

**Last Updated:** October 20, 2025  **Last Updated:** October 20, 2025  

**Platform:** Windows (officially supported)**Platform Support:** Windows (officially supported), macOS/Linux (experimental)



------



## Prerequisites## Table of Contents



### Required Software1. [Prerequisites](#prerequisites)

2. [Build Commands](#build-commands)

1. **Node.js** (v18 or higher)3. [Platform-Specific Instructions](#platform-specific-instructions)

   - Download: https://nodejs.org/4. [Release Process](#release-process)

5. [Code Signing](#code-signing)

2. **pnpm** (v8 or higher)6. [Troubleshooting](#troubleshooting)

   ```powershell

   npm install -g pnpm---

   ```

## Prerequisites

3. **Rust** (latest stable)

   - Download: https://rustup.rs/### All Platforms

   - Or via winget:

   ```powershell- **Node.js**: v18 or higher

   winget install Rustlang.Rustup- **pnpm**: v8 or higher

   ```- **Rust**: Latest stable version (install via [rustup](https://rustup.rs/))

- **Git**: For version control

4. **Visual Studio Build Tools** or **Visual Studio 2022**

   - Install "Desktop development with C++" workloadInstall dependencies:

   - Download: https://visualstudio.microsoft.com/downloads/```bash

pnpm install

5. **Git**```

   - Download: https://git-scm.com/

### Windows

---

- **Visual Studio Build Tools** or **Visual Studio 2022**

## Quick Start  - Install "Desktop development with C++" workload

  - Download: https://visualstudio.microsoft.com/downloads/

1. **Clone the repository:**

   ```powershell- **WebView2** (usually pre-installed on Windows 10/11)

   git clone https://github.com/Cemililkim/Clerk.git  - If needed: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

   cd Clerk

   ```### macOS



2. **Install dependencies:**- **Xcode Command Line Tools**:

   ```powershell  ```bash

   pnpm install  xcode-select --install

   ```  ```



3. **Build for production:**- For **code signing**, you need:

   ```powershell  - Apple Developer account

   pnpm tauri build  - Valid Developer ID certificate

   ```

### Linux

4. **Find your installer:**

   - Location: `src-tauri\target\release\bundle\nsis\Clerk_1.0.0_x64-setup.exe`#### Debian/Ubuntu

   - Size: ~4 MB

```bash

---sudo apt update

sudo apt install -y \

## Development  libwebkit2gtk-4.0-dev \

  build-essential \

### Run in Dev Mode  curl \

  wget \

Hot reload enabled for rapid development:  file \

  libssl-dev \

```powershell  libgtk-3-dev \

pnpm tauri dev  libayatana-appindicator3-dev \

```  librsvg2-dev

```

This will:

- Start Vite dev server (React)#### Fedora

- Launch Tauri app in development mode

- Auto-reload on file changes```bash

sudo dnf install -y \

### Build Frontend Only  webkit2gtk4.0-devel \

  openssl-devel \

```powershell  curl \

pnpm build  wget \

```  file \

  libappindicator-gtk3-devel \

---  librsvg2-devel

```

## Build Commands

#### Arch Linux

### Production Build

```bash

```powershellsudo pacman -S --needed \

# Full production build  webkit2gtk \

pnpm tauri build  base-devel \

```  curl \

  wget \

Output files:  file \

- `src-tauri\target\release\bundle\nsis\Clerk_1.0.0_x64-setup.exe` (NSIS installer)  openssl \

- `src-tauri\target\release\app.exe` (Standalone executable)  appmenu-gtk-module \

  gtk3 \

### Debug Build  libappindicator-gtk3 \

  librsvg

For troubleshooting with debug symbols:```



```powershell---

pnpm tauri build --debug

```## Build Commands



---### Development Build



## Project Structure```bash

# Start dev server with hot reload

```pnpm tauri:dev

Clerk/```

├── src/                    # React frontend

│   ├── components/         # UI components### Production Build (All Platforms)

│   ├── contexts/          # React contexts

│   ├── hooks/             # Custom hooks```bash

│   ├── styles/            # CSS styles# Build for current platform

│   ├── types/             # TypeScript typespnpm build:all

│   └── utils/             # Utility functions```

│

├── src-tauri/             # Rust backendThis will:

│   ├── src/1. Run TypeScript compilation (`tsc`)

│   │   ├── commands/      # Tauri commands (API)2. Build React app with Vite

│   │   ├── crypto/        # Encryption logic3. Build Tauri app for your current platform

│   │   ├── database/      # SQLite operations4. Output installer in `src-tauri/target/release/bundle/`

│   │   ├── vault/         # Vault management

│   │   ├── keychain/      # OS keychain integration### Platform-Specific Builds

│   │   ├── cli.rs         # CLI implementation

│   │   ├── lib.rs         # Library entry point#### Windows

│   │   └── main.rs        # App entry point

│   │```bash

│   ├── Cargo.toml         # Rust dependencies# Build MSI and NSIS installers

│   ├── tauri.conf.json    # Tauri configurationpnpm build:windows

│   └── build.rs           # Build script```

│

├── package.json           # Node.js dependencies**Output:**

├── vite.config.ts         # Vite configuration- `src-tauri/target/release/bundle/msi/Clerk_0.1.0_x64_en-US.msi`

└── tsconfig.json          # TypeScript configuration- `src-tauri/target/release/bundle/nsis/Clerk_0.1.0_x64-setup.exe`

```

#### macOS

---

```bash

## Troubleshooting# Build universal binary (Intel + Apple Silicon)

pnpm build:macos

### Build Fails: "Rust not found"

# Or build for specific architecture:

**Solution:**pnpm build:macos:x64   # Intel Macs only

```powershellpnpm build:macos:arm   # Apple Silicon only

rustup update stable```

```

**Output:**

### Build Fails: "Missing Visual Studio Build Tools"- `src-tauri/target/release/bundle/dmg/Clerk_0.1.0_universal.dmg`

- `src-tauri/target/release/bundle/macos/Clerk.app`

**Solution:**

1. Install Visual Studio 2022 (Community Edition is free)#### Linux

2. Enable "Desktop development with C++" workload

3. Restart your terminal```bash

# Build AppImage and deb packages

### Build Fails: "WebView2 not found"pnpm build:linux



**Solution:**# For ARM64 (e.g., Raspberry Pi):

Windows 10/11 usually has WebView2 pre-installed. If not:pnpm build:linux:arm

- Download: https://developer.microsoft.com/microsoft-edge/webview2/```



### Slow Build Times**Output:**

- `src-tauri/target/release/bundle/appimage/clerk_0.1.0_amd64.AppImage`

**Tips:**- `src-tauri/target/release/bundle/deb/clerk_0.1.0_amd64.deb`

- First build is slow (~5 minutes) - downloads dependencies- `src-tauri/target/release/bundle/rpm/clerk-0.1.0-1.x86_64.rpm`

- Subsequent builds are faster (~30 seconds)

- Use `pnpm tauri dev` for development (much faster)### Debug Build



### "Permission Denied" ErrorsFor development/testing with debug symbols:



**Solution:**```bash

Run PowerShell as Administrator for first build.pnpm tauri:build:debug

```

---

---

## CLI Build

## Platform-Specific Instructions

The CLI is automatically included in the build. After building:

### Windows Build

```powershell

# Test CLI1. **Open PowerShell as Administrator** (first time only for Rust setup)

.\src-tauri\target\release\clerk.exe --help

```2. **Install Rust** (if not already installed):

   ```powershell

---   winget install Rustlang.Rustup

   ```

## Customization

3. **Build**:

### Change App Icon   ```powershell

   cd C:\Users\YourName\Desktop\ClerkApp\Clerk

1. Replace icons in `src-tauri/icons/`   pnpm install

2. Rebuild with `pnpm tauri build`   pnpm build:windows

   ```

### Change App Name

4. **Test installer**:

Update in these files:   - Navigate to `src-tauri\target\release\bundle\nsis\`

- `package.json` → `name`   - Run `Clerk_0.1.0_x64-setup.exe`

- `src-tauri/tauri.conf.json` → `productName`   - Or use MSI: `src-tauri\target\release\bundle\msi\Clerk_0.1.0_x64_en-US.msi`

- `src-tauri/Cargo.toml` → `name`

**Build time:** ~5-10 minutes (first build)

### Change Version

---

Update in:

- `package.json` → `version`### macOS Build

- `src-tauri/tauri.conf.json` → `version`

- `src-tauri/Cargo.toml` → `version`1. **Install Xcode Command Line Tools**:

   ```bash

---   xcode-select --install

   ```

## Release Process

2. **Install Rust**:

1. **Update version** in all files (see above)   ```bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. **Build production binary:**   ```

   ```powershell

   pnpm tauri build3. **Add ARM64 target** (for universal binary):

   ```   ```bash

   rustup target add aarch64-apple-darwin

3. **Test installer:**   ```

   - Run `Clerk_1.0.0_x64-setup.exe`

   - Verify all features work4. **Build**:

   - Check PATH integration   ```bash

   cd ~/Desktop/ClerkApp/Clerk

4. **Generate checksum:**   pnpm install

   ```powershell   pnpm build:macos

   Get-FileHash -Algorithm SHA256 .\src-tauri\target\release\bundle\nsis\Clerk_1.0.0_x64-setup.exe   ```

   ```

5. **Test**:

5. **Create GitHub Release:**   ```bash

   - Tag: `v1.0.0`   open src-tauri/target/release/bundle/dmg/Clerk_0.1.0_universal.dmg

   - Upload installer   ```

   - Add checksum to release notes

**Build time:** ~10-15 minutes (first build)

---

**Note:** Universal binary builds on Intel Macs require ARM64 cross-compilation. Use `build:macos:x64` if you encounter issues.

## Dependencies

---

### Frontend (Node.js)

### Linux Build

- **React** - UI framework

- **TypeScript** - Type safety#### Ubuntu/Debian Example

- **Vite** - Build tool

- **Tauri API** - Desktop integration1. **Install dependencies**:

- **Lucide React** - Icons   ```bash

   sudo apt update

### Backend (Rust)   sudo apt install -y \

     libwebkit2gtk-4.0-dev \

- **Tauri** - Desktop framework     build-essential \

- **rusqlite** - SQLite database     curl \

- **ring** - Cryptography (AES-256-GCM)     wget \

- **argon2** - Password hashing     file \

- **keyring** - OS keychain integration     libssl-dev \

- **winreg** - Windows registry (PATH management)     libgtk-3-dev \

- **clap** - CLI argument parsing     libayatana-appindicator3-dev \

     librsvg2-dev

---   ```



## Resources2. **Install Rust**:

   ```bash

- **Tauri Documentation**: https://tauri.app/   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- **Rust Book**: https://doc.rust-lang.org/book/   source $HOME/.cargo/env

- **React Documentation**: https://react.dev/   ```

- **Project Repository**: https://github.com/Cemililkim/Clerk

3. **Install Node.js and pnpm**:

---   ```bash

   # Install Node.js (via nvm recommended)

## Need Help?   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

   source ~/.bashrc

- **Issues**: https://github.com/Cemililkim/Clerk/issues   nvm install 18

- **Discussions**: https://github.com/Cemililkim/Clerk/discussions   

   # Install pnpm

---   npm install -g pnpm

   ```

*Last updated: October 20, 2025*  

*Clerk v1.0.0 - Free & Open Source*4. **Build**:

   ```bash
   cd ~/Desktop/ClerkApp/Clerk
   pnpm install
   pnpm build:linux
   ```

5. **Test AppImage**:
   ```bash
   chmod +x src-tauri/target/release/bundle/appimage/clerk_0.1.0_amd64.AppImage
   ./src-tauri/target/release/bundle/appimage/clerk_0.1.0_amd64.AppImage
   ```

6. **Install deb package**:
   ```bash
   sudo dpkg -i src-tauri/target/release/bundle/deb/clerk_0.1.0_amd64.deb
   ```

**Build time:** ~10-15 minutes (first build)

---

## Release Process

### 1. Pre-Release Checklist

- [ ] Update version in `package.json`
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Run full test suite
- [ ] Run security audit: `cargo audit && pnpm audit`
- [ ] Test on all platforms
- [ ] Update documentation if needed

### 2. Version Bump

```bash
# Example: Bumping to v1.0.0

# Update package.json
npm version 1.0.0 --no-git-tag-version

# Update src-tauri/Cargo.toml manually
# version = "1.0.0"

# Update src-tauri/tauri.conf.json manually
# "version": "1.0.0"
```

### 3. Build for All Platforms

You'll need access to each platform to build:

**On Windows:**
```powershell
pnpm build:windows
```

**On macOS:**
```bash
pnpm build:macos
```

**On Linux:**
```bash
pnpm build:linux
```

### 4. Create Release Assets

Organize build artifacts:

```
release/
├── windows/
│   ├── Clerk_1.0.0_x64-setup.exe
│   ├── Clerk_1.0.0_x64_en-US.msi
│   └── checksums.txt
├── macos/
│   ├── Clerk_1.0.0_universal.dmg
│   └── checksums.txt
└── linux/
    ├── clerk_1.0.0_amd64.AppImage
    ├── clerk_1.0.0_amd64.deb
    ├── clerk-1.0.0-1.x86_64.rpm
    └── checksums.txt
```

### 5. Generate Checksums

**Windows (PowerShell):**
```powershell
Get-FileHash Clerk_1.0.0_x64-setup.exe -Algorithm SHA256 | Format-List
Get-FileHash Clerk_1.0.0_x64_en-US.msi -Algorithm SHA256 | Format-List
```

**macOS/Linux:**
```bash
shasum -a 256 Clerk_1.0.0_universal.dmg > checksums.txt
shasum -a 256 clerk_1.0.0_amd64.AppImage >> checksums.txt
```

### 6. Create Git Tag

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### 7. Create GitHub Release

1. Go to: https://github.com/Cemililkim/Clerk/releases/new
2. Select tag: `v1.0.0`
3. Title: `Clerk v1.0.0`
4. Copy release notes from `CHANGELOG.md`
5. Upload all build artifacts:
   - Windows: `.exe`, `.msi`
   - macOS: `.dmg`
   - Linux: `.AppImage`, `.deb`, `.rpm`
   - All checksum files
6. Mark as pre-release if testing
7. Publish release

---

## Code Signing

### Windows Code Signing

**With Certificate:**

1. Obtain a code signing certificate (DigiCert, Sectigo, etc.)
2. Update `tauri.conf.json`:
   ```json
   "windows": {
     "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
     "timestampUrl": "http://timestamp.digicert.com"
   }
   ```
3. Build will automatically sign

**Self-Signed (Testing Only):**

Not recommended for distribution. Windows SmartScreen will warn users.

### macOS Code Signing

**With Apple Developer Account:**

1. Get Developer ID certificate from Apple
2. Update `tauri.conf.json`:
   ```json
   "macOS": {
     "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
   }
   ```
3. Build will automatically sign and notarize

**Without Certificate:**

App will work but show security warning on first launch. Users must right-click → Open.

### Linux

No code signing required. Package managers handle verification via GPG signatures.

---

## Troubleshooting

### Windows

**Error: `link.exe` not found**

Install Visual Studio Build Tools with C++ workload.

**Error: WebView2 not found**

Install WebView2 Runtime: https://developer.microsoft.com/microsoft-edge/webview2/

**Error: Rust compilation failed**

```powershell
rustup update stable
cargo clean
```

---

### macOS

**Error: `xcrun: error: invalid active developer path`**

Install Xcode Command Line Tools:
```bash
xcode-select --install
```

**Error: Cross-compilation failed (ARM64)**

Make sure target is installed:
```bash
rustup target add aarch64-apple-darwin
```

**Error: Code signing failed**

Check certificate:
```bash
security find-identity -v -p codesigning
```

---

### Linux

**Error: `webkit2gtk-4.0 not found`**

Install WebKitGTK:
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel

# Arch
sudo pacman -S webkit2gtk
```

**Error: `pkg-config` not found**

```bash
sudo apt install pkg-config
```

**AppImage doesn't run**

Make executable:
```bash
chmod +x clerk_0.1.0_amd64.AppImage
```

---

### General Issues

**Error: `pnpm` command not found**

Install pnpm:
```bash
npm install -g pnpm
```

**Error: Rust not found**

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Error: Build runs out of memory**

Increase Node.js memory:
```bash
export NODE_OPTIONS="--max-old-space-size=4096"
pnpm build:all
```

**Clean build (fixes most issues):**

```bash
# Clean everything
pnpm clean  # if script exists
cargo clean
rm -rf node_modules dist src-tauri/target
pnpm install
pnpm build:all
```

---

## Build Performance Tips

### Faster Builds

1. **Use SSD** for source code
2. **Increase RAM** allocation (4GB+ recommended)
3. **Enable incremental compilation** (Rust default)
4. **Use `cargo build --release`** directly for Rust-only changes
5. **Cache dependencies** (pnpm does this automatically)

### First Build vs Incremental

- **First build:** 5-15 minutes (downloads all dependencies)
- **Incremental:** 30 seconds - 2 minutes (only changed files)

### Parallel Builds

Rust builds in parallel by default. Control with:
```bash
# Use 4 cores
cargo build -j 4

# Use all cores
cargo build -j $(nproc)
```

---

## CI/CD Without GitHub Actions

### Manual Build Script

Create `scripts/build-all.sh`:

```bash
#!/bin/bash

VERSION=$(node -p "require('./package.json').version")
echo "Building Clerk v$VERSION for all platforms..."

# Build frontend
echo "Building frontend..."
pnpm build

# Build for current platform
echo "Building for current platform..."
pnpm tauri build

# Organize artifacts
mkdir -p release/v$VERSION
cp -r src-tauri/target/release/bundle/* release/v$VERSION/

# Generate checksums
cd release/v$VERSION
find . -type f -exec shasum -a 256 {} \; > checksums.txt

echo "Build complete! Artifacts in release/v$VERSION"
```

Usage:
```bash
chmod +x scripts/build-all.sh
./scripts/build-all.sh
```

### Alternative CI Solutions

If you want automated builds without GitHub Actions:

1. **GitLab CI** (free for public/private repos)
2. **Drone CI** (self-hosted, open source)
3. **Jenkins** (self-hosted, open source)
4. **CircleCI** (free tier available)
5. **Local build server** with cron jobs

---

## Distribution

### Without App Store

**Windows:**
- Host `.exe`/`.msi` on GitHub Releases
- Users download and install manually
- Consider code signing to avoid SmartScreen warnings

**macOS:**
- Host `.dmg` on GitHub Releases
- Users drag to Applications folder
- Without notarization, users must right-click → Open (first launch)

**Linux:**
- Host `.AppImage`, `.deb`, `.rpm` on GitHub Releases
- AppImage: No installation needed (portable)
- deb/rpm: Users install via package manager

### Update Distribution

For manual updates:
1. Post new release on GitHub
2. Update website/README with download links
3. Announce on social media/forums
4. Users download and install over old version

---

## Build Artifacts Reference

### Windows

| File | Type | Size | Description |
|------|------|------|-------------|
| `Clerk_0.1.0_x64-setup.exe` | NSIS Installer | ~80MB | Recommended installer |
| `Clerk_0.1.0_x64_en-US.msi` | MSI Installer | ~80MB | Enterprise-friendly |

### macOS

| File | Type | Size | Description |
|------|------|------|-------------|
| `Clerk_0.1.0_universal.dmg` | DMG Image | ~100MB | Intel + Apple Silicon |
| `Clerk_0.1.0_x64.dmg` | DMG Image | ~50MB | Intel only |
| `Clerk_0.1.0_aarch64.dmg` | DMG Image | ~50MB | Apple Silicon only |

### Linux

| File | Type | Size | Description |
|------|------|------|-------------|
| `clerk_0.1.0_amd64.AppImage` | AppImage | ~100MB | Portable, no install |
| `clerk_0.1.0_amd64.deb` | Debian Package | ~80MB | Ubuntu/Debian/Mint |
| `clerk-0.1.0-1.x86_64.rpm` | RPM Package | ~80MB | Fedora/RHEL/openSUSE |

---

## Resources

- **Tauri Documentation**: https://tauri.app/v1/guides/building/
- **Rust Installation**: https://rustup.rs/
- **Node.js Downloads**: https://nodejs.org/
- **pnpm Installation**: https://pnpm.io/installation
- **WebView2**: https://developer.microsoft.com/microsoft-edge/webview2/

---

**For More Information:**

- **User Guide**: See [USER_GUIDE.md](USER_GUIDE.md)
- **API Reference**: See [API_REFERENCE.md](API_REFERENCE.md)
- **Security Audit**: See [SECURITY_AUDIT.md](SECURITY_AUDIT.md)
- **Development Guide**: See [CODE_GUIDE.md](CODE_GUIDE.md)

---

*Last updated: October 18, 2025*  
*Clerk v1.0.0 - Free & Open Source*
