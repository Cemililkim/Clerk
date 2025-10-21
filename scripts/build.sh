#!/bin/bash

# Clerk Build Script for All Platforms
# This script builds Clerk and organizes release artifacts

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from package.json
VERSION=$(node -p "require('./package.json').version")
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')

echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Clerk Build Script v$VERSION          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════╝${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}[1/7] Checking prerequisites...${NC}"
command -v node >/dev/null 2>&1 || { echo -e "${RED}Error: Node.js not found${NC}"; exit 1; }
command -v pnpm >/dev/null 2>&1 || { echo -e "${RED}Error: pnpm not found${NC}"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo -e "${RED}Error: Rust/Cargo not found${NC}"; exit 1; }
echo -e "${GREEN}✓ All prerequisites found${NC}"

# Install dependencies
echo -e "${YELLOW}[2/7] Installing dependencies...${NC}"
pnpm install --frozen-lockfile
echo -e "${GREEN}✓ Dependencies installed${NC}"

# Type check
echo -e "${YELLOW}[3/7] Running type check...${NC}"
pnpm type-check
echo -e "${GREEN}✓ Type check passed${NC}"

# Lint check
echo -e "${YELLOW}[4/7] Running lint check...${NC}"
pnpm lint
echo -e "${GREEN}✓ Lint check passed${NC}"

# Build frontend
echo -e "${YELLOW}[5/7] Building frontend...${NC}"
pnpm build
echo -e "${GREEN}✓ Frontend built${NC}"

# Build Tauri app
echo -e "${YELLOW}[6/7] Building Tauri app...${NC}"
echo -e "${BLUE}This may take 5-15 minutes on first build...${NC}"
pnpm tauri build
echo -e "${GREEN}✓ Tauri app built${NC}"

# Organize artifacts
echo -e "${YELLOW}[7/7] Organizing release artifacts...${NC}"

RELEASE_DIR="release/v$VERSION"
mkdir -p "$RELEASE_DIR"

# Detect platform and copy appropriate files
case "$PLATFORM" in
  linux)
    echo -e "${BLUE}Detected Linux build${NC}"
    cp -r src-tauri/target/release/bundle/appimage/*.AppImage "$RELEASE_DIR/" 2>/dev/null || true
    cp -r src-tauri/target/release/bundle/deb/*.deb "$RELEASE_DIR/" 2>/dev/null || true
    cp -r src-tauri/target/release/bundle/rpm/*.rpm "$RELEASE_DIR/" 2>/dev/null || true
    ;;
  darwin)
    echo -e "${BLUE}Detected macOS build${NC}"
    cp -r src-tauri/target/release/bundle/dmg/*.dmg "$RELEASE_DIR/" 2>/dev/null || true
    cp -r src-tauri/target/release/bundle/macos/*.app "$RELEASE_DIR/" 2>/dev/null || true
    ;;
  *)
    echo -e "${YELLOW}Unknown platform: $PLATFORM${NC}"
    echo -e "${YELLOW}Copying all artifacts...${NC}"
    cp -r src-tauri/target/release/bundle/* "$RELEASE_DIR/" 2>/dev/null || true
    ;;
esac

# Generate checksums
if [ -d "$RELEASE_DIR" ] && [ "$(ls -A $RELEASE_DIR)" ]; then
  cd "$RELEASE_DIR"
  echo -e "${BLUE}Generating checksums...${NC}"
  find . -type f ! -name "checksums.txt" -exec shasum -a 256 {} \; > checksums.txt
  cd - > /dev/null
  echo -e "${GREEN}✓ Checksums generated${NC}"
fi

# Print summary
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║            Build Complete! 🎉                  ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Version:${NC} $VERSION"
echo -e "${BLUE}Platform:${NC} $PLATFORM"
echo -e "${BLUE}Artifacts:${NC} $RELEASE_DIR"
echo ""

# List generated files
if [ -d "$RELEASE_DIR" ]; then
  echo -e "${YELLOW}Generated files:${NC}"
  find "$RELEASE_DIR" -type f -exec basename {} \; | grep -v checksums.txt | while read file; do
    echo -e "  ${GREEN}✓${NC} $file"
  done
  echo ""
  echo -e "${BLUE}Checksums:${NC}"
  cat "$RELEASE_DIR/checksums.txt" 2>/dev/null || echo "  (checksums.txt not generated)"
fi

echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  1. Test the installer in $RELEASE_DIR"
echo -e "  2. Create a git tag: ${GREEN}git tag -a v$VERSION -m 'Release v$VERSION'${NC}"
echo -e "  3. Push the tag: ${GREEN}git push origin v$VERSION${NC}"
echo -e "  4. Create a GitHub release and upload artifacts"
echo ""
