#!/usr/bin/env node

/**
 * Clerk Release Helper
 * 
 * Detects platform and runs appropriate build script
 */

const { execSync } = require('child_process');
const os = require('os');
const fs = require('fs');
const path = require('path');

// Colors for terminal output
const colors = {
  reset: '\x1b[0m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function error(message) {
  log(`❌ ${message}`, 'red');
  process.exit(1);
}

function success(message) {
  log(`✓ ${message}`, 'green');
}

function info(message) {
  log(`ℹ ${message}`, 'blue');
}

function warning(message) {
  log(`⚠ ${message}`, 'yellow');
}

// Get package version
function getVersion() {
  const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  return packageJson.version;
}

// Check if file exists
function fileExists(filePath) {
  return fs.existsSync(filePath);
}

// Main function
function main() {
  const platform = os.platform();
  const version = getVersion();

  console.log('');
  log('╔════════════════════════════════════════════════╗', 'blue');
  log(`║       Clerk Release Helper v${version}         ║`, 'blue');
  log('╚════════════════════════════════════════════════╝', 'blue');
  console.log('');

  info(`Platform: ${platform}`);
  info(`Version: ${version}`);
  console.log('');

  // Check if scripts exist
  const windowsScript = path.join('scripts', 'build.ps1');
  const unixScript = path.join('scripts', 'build.sh');

  let scriptPath;
  let command;

  if (platform === 'win32') {
    if (!fileExists(windowsScript)) {
      error(`Windows build script not found: ${windowsScript}`);
    }
    scriptPath = windowsScript;
    command = `powershell -ExecutionPolicy Bypass -File "${scriptPath}"`;
  } else {
    if (!fileExists(unixScript)) {
      error(`Unix build script not found: ${unixScript}`);
    }
    scriptPath = unixScript;
    
    // Make script executable
    try {
      fs.chmodSync(unixScript, '755');
    } catch (err) {
      warning(`Could not make script executable: ${err.message}`);
    }
    
    command = `bash "${scriptPath}"`;
  }

  info(`Running: ${command}`);
  console.log('');

  try {
    execSync(command, { stdio: 'inherit' });
    console.log('');
    success('Release build completed successfully!');
  } catch (err) {
    console.log('');
    error('Release build failed!');
  }
}

// Run
main();
