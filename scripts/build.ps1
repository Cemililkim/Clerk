# Clerk Build Script for Windows
# This script builds Clerk and organizes release artifacts

$ErrorActionPreference = "Stop"

# Get version from package.json
$VERSION = (Get-Content package.json | ConvertFrom-Json).version

Write-Host ""
Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Blue
Write-Host "║         Clerk Build Script v$VERSION          ║" -ForegroundColor Blue
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Blue
Write-Host ""

# Check prerequisites
Write-Host "[1/7] Checking prerequisites..." -ForegroundColor Yellow

$commands = @("node", "pnpm", "cargo")
foreach ($cmd in $commands) {
    if (!(Get-Command $cmd -ErrorAction SilentlyContinue)) {
        Write-Host "Error: $cmd not found" -ForegroundColor Red
        exit 1
    }
}
Write-Host "✓ All prerequisites found" -ForegroundColor Green

# Install dependencies
Write-Host "[2/7] Installing dependencies..." -ForegroundColor Yellow
pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "✓ Dependencies installed" -ForegroundColor Green

# Type check
Write-Host "[3/7] Running type check..." -ForegroundColor Yellow
pnpm type-check
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "✓ Type check passed" -ForegroundColor Green

# Lint check
Write-Host "[4/7] Running lint check..." -ForegroundColor Yellow
pnpm lint
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "✓ Lint check passed" -ForegroundColor Green

# Build frontend
Write-Host "[5/7] Building frontend..." -ForegroundColor Yellow
pnpm build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "✓ Frontend built" -ForegroundColor Green

# Build Tauri app
Write-Host "[6/7] Building Tauri app..." -ForegroundColor Yellow
Write-Host "This may take 5-15 minutes on first build..." -ForegroundColor Blue
pnpm tauri build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "✓ Tauri app built" -ForegroundColor Green

# Organize artifacts
Write-Host "[7/7] Organizing release artifacts..." -ForegroundColor Yellow

$RELEASE_DIR = "release\v$VERSION"
New-Item -ItemType Directory -Force -Path $RELEASE_DIR | Out-Null

# Copy Windows installers
Write-Host "Detected Windows build" -ForegroundColor Blue

# Copy NSIS installer
if (Test-Path "src-tauri\target\release\bundle\nsis\*.exe") {
    Copy-Item "src-tauri\target\release\bundle\nsis\*.exe" -Destination $RELEASE_DIR -Force
}

# Copy MSI installer
if (Test-Path "src-tauri\target\release\bundle\msi\*.msi") {
    Copy-Item "src-tauri\target\release\bundle\msi\*.msi" -Destination $RELEASE_DIR -Force
}

# Generate checksums
Write-Host "Generating checksums..." -ForegroundColor Blue
$checksumFile = Join-Path $RELEASE_DIR "checksums.txt"
Get-ChildItem $RELEASE_DIR -File | Where-Object { $_.Name -ne "checksums.txt" } | ForEach-Object {
    $hash = (Get-FileHash $_.FullName -Algorithm SHA256).Hash
    "$hash  $($_.Name)" | Out-File -Append -FilePath $checksumFile -Encoding UTF8
}
Write-Host "✓ Checksums generated" -ForegroundColor Green

# Print summary
Write-Host ""
Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║            Build Complete! 🎉                  ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Version: " -NoNewline -ForegroundColor Blue
Write-Host $VERSION
Write-Host "Platform: " -NoNewline -ForegroundColor Blue
Write-Host "Windows"
Write-Host "Artifacts: " -NoNewline -ForegroundColor Blue
Write-Host $RELEASE_DIR
Write-Host ""

# List generated files
Write-Host "Generated files:" -ForegroundColor Yellow
Get-ChildItem $RELEASE_DIR -File | Where-Object { $_.Name -ne "checksums.txt" } | ForEach-Object {
    Write-Host "  ✓ $($_.Name)" -ForegroundColor Green
}

Write-Host ""
Write-Host "Checksums:" -ForegroundColor Blue
Get-Content $checksumFile | ForEach-Object {
    Write-Host "  $_"
}

Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Test the installer in $RELEASE_DIR"
Write-Host "  2. Create a git tag: " -NoNewline
Write-Host "git tag -a v$VERSION -m 'Release v$VERSION'" -ForegroundColor Green
Write-Host "  3. Push the tag: " -NoNewline
Write-Host "git push origin v$VERSION" -ForegroundColor Green
Write-Host "  4. Create a GitHub release and upload artifacts"
Write-Host ""
