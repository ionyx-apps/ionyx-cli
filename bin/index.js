#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

/**
 * Ionyx CLI Wrapper
 * Now uses the new Rust CLI instead of bundled binaries
 */

const args = process.argv.slice(2);
const command = args[0] || 'dev';

// Convert npm-style commands to cargo
const cargoArgs = ['run', '--bin', 'ionyx'];

if (command === 'dev' || command === 'build' || command === 'create' || command === 'run') {
    cargoArgs.push(command);
    // Add remaining args
    cargoArgs.push(...args.slice(1));
} else {
    // Default to dev
    cargoArgs.push('dev');
}

console.log(`🚀 Starting Ionyx CLI: cargo ${cargoArgs.join(' ')}`);

const cargo = spawn('cargo', cargoArgs, {
    stdio: 'inherit',
    cwd: process.cwd(),
    env: { ...process.env, FORCE_COLOR: '1' }
});

cargo.on('exit', (code) => {
    process.exit(code);
});

cargo.on('error', (err) => {
    console.error('❌ Failed to start Ionyx CLI:', err.message);
    console.error('💡 Make sure Rust and Cargo are installed: https://rustup.rs');
    process.exit(1);
});
