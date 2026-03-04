#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * Ionyx CLI Wrapper
 * Uses bundled binary - no Rust/Cargo required!
 */

let args = process.argv.slice(2);

// Handle npx create-ionyx-app <name> command
if (args.length === 1 && args[0] !== 'dev' && args[0] !== 'build' && args[0] !== 'create' && args[0] !== 'run') {
    // Assume it's a project name for create command
    args = ['create', args[0]];
}

// Get platform-specific binary path
function getBinaryPath() {
    const platform = process.platform;
    const arch = process.arch;
    const binDir = path.join(__dirname, '..', 'bin');

    let binaryName;
    if (platform === 'win32') {
        binaryName = 'cargo-ionyx.exe';
    } else if (platform === 'linux') {
        binaryName = 'cargo-ionyx';
    } else if (platform === 'darwin') {
        binaryName = 'cargo-ionyx';
    } else {
        throw new Error(`Unsupported platform: ${platform}-${arch}`);
    }

    const binaryPath = path.join(binDir, binaryName);

    if (!fs.existsSync(binaryPath)) {
        throw new Error(`Binary not found: ${binaryPath}. Please reinstall the package.`);
    }

    return binaryPath;
}

const binaryPath = getBinaryPath();

console.log(`🚀 Starting Ionyx CLI: ${path.basename(binaryPath)} ${args.join(' ')}`);

const child = spawn(binaryPath, args, {
    stdio: 'inherit',
    cwd: process.cwd(),
    env: { ...process.env, FORCE_COLOR: '1' }
});

child.on('exit', (code) => {
    process.exit(code);
});

child.on('error', (err) => {
    console.error('❌ Failed to start Ionyx CLI:', err.message);
    console.error('💡 Please report this issue: https://github.com/ionyx-framework/ionyx/issues');
    process.exit(1);
});
