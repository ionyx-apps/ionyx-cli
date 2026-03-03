#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

/**
 * Stable Ionyx CLI Binary Wrapper
 * This script detects the host platform and executes the bundled pre-compiled binary.
 * It no longer attempts to build from source at runtime.
 */

const platform = process.platform;
const arch = process.arch;

// Determine the correct binary name based on platform
let binaryName = 'ionyx';
if (platform === 'win32') {
    binaryName = 'ionyx-win.exe';
} else if (platform === 'darwin') {
    binaryName = arch === 'arm64' ? 'ionyx-macos-arm64' : 'ionyx-macos-x64';
} else if (platform === 'linux') {
    binaryName = arch === 'arm64' ? 'ionyx-linux-arm64' : 'ionyx-linux-x64';
}

const binaryPath = path.join(__dirname, binaryName);

// Check if the binary exists in the package
if (!fs.existsSync(binaryPath)) {
    console.error(`\n❌ Ionyx CLI Error: Unsupported platform or binary missing.`);
    console.error(`   Platform: ${platform}-${arch}`);
    console.error(`   Expected path: ${binaryPath}\n`);
    console.error(`Please ensure you have installed the correct version of the 'ionyx' package.`);
    console.error(`If you are a developer, make sure to compile the binaries and place them in the 'bin/' folder before publishing.`);
    process.exit(1);
}

// Execution
const args = process.argv.slice(2);
const scriptPath = process.argv[1];
const scriptName = path.basename(scriptPath);

// Default to 'create' if called via npx create-ionyx-app or npx ionyx without args
if ((scriptName === 'create-ionyx-app' || scriptName === 'index.js') && args.length === 0) {
    args.push('create');
} else if (scriptName === 'create-ionyx-app' && args.length > 0 && args[0] !== 'create') {
    args.unshift('create');
}

const result = spawnSync(binaryPath, args, {
    stdio: 'inherit',
    shell: false,
    windowsHide: true
});

process.exit(result.status || 0);
