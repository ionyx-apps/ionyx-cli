#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const binary = process.platform === 'win32' ? 'ionyx.exe' : 'ionyx';
const binaryPath = path.join(__dirname, binary);

const args = process.argv.slice(2);
const child = spawn(binaryPath, ['create', ...args], { stdio: 'inherit' });

child.on('close', (code) => {
  process.exit(code);
});
