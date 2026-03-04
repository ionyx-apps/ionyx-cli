#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * Ionyx CLI Wrapper
 * Uses bundled binary - no Rust/Cargo required!
 */

const args = process.argv.slice(2);

// Get platform-specific binary path
function getBinaryPath() {
    const platform = process.platform;
    const arch = process.arch;
    const binDir = path.join(__dirname, '..', 'bin');

    let binaryName;
    if (platform === 'win32') {
        binaryName = 'ionyx-win.exe';
    } else if (platform === 'linux') {
        binaryName = arch === 'x64' ? 'ionyx-linux-x64' : 'ionyx-linux-arm64';
    } else if (platform === 'darwin') {
        binaryName = arch === 'x64' ? 'ionyx-macos-x64' : 'ionyx-macos-arm64';
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
