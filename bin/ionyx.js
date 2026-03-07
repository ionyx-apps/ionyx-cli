#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * Ionyx CLI NPM Wrapper
 * This script delegates execution to the native Rust binary.
 */

const binaryName = process.platform === 'win32' ? 'ionyx.exe' : 'ionyx';

// 1. Try local binary (in the same bin folder)
let binaryPath = path.join(__dirname, binaryName);

// 2. Fallback to development path if missing (for local dev testing)
if (!fs.existsSync(binaryPath)) {
  const devPath = path.join(__dirname, '..', 'target', 'release', binaryName);
  if (fs.existsSync(devPath)) {
    binaryPath = devPath;
  }
}

if (!fs.existsSync(binaryPath)) {
  console.error(`\x1b[31mError: Ionyx CLI native binary not found at ${binaryPath}\x1b[0m`);
  console.error(`Please ensure the binary is built or downloaded correctly.`);
  console.error(`Run 'cargo build --release' in the Ionyx project root if you are in a dev environment.`);
  process.exit(1);
}

const args = process.argv.slice(2);
const child = spawn(binaryPath, args, { 
  stdio: 'inherit',
  env: {
    ...process.env,
    // Pass the absolute path to templates so the binary can find them
    IONYX_TEMPLATES_PATH: path.join(__dirname, '..', 'ionyx-cli', 'src', 'templates')
  }
});

child.on('error', (err) => {
  console.error(`\x1b[31mFailed to start Ionyx CLI: ${err.message}\x1b[0m`);
  process.exit(1);
});

child.on('close', (code) => {
  process.exit(code);
});
