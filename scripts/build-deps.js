#!/usr/bin/env node

/**
 * Build git dependencies that require compilation
 *
 * This script runs after pnpm install to build any git-hosted packages
 * that need TypeScript compilation before they can be used.
 */

import { execSync } from 'node:child_process';
import { existsSync, readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

const pnpmDir = join(rootDir, 'node_modules', '.pnpm');

if (!existsSync(pnpmDir)) {
  console.log('No .pnpm directory found, skipping dependency build');
  process.exit(0);
}

// Find @jbcom/agentic-triage in .pnpm
const dirs = readdirSync(pnpmDir, { withFileTypes: true });
const triageDirs = dirs.filter((d) => d.isDirectory() && d.name.includes('@jbcom+agentic-triage'));

for (const dir of triageDirs) {
  const triagePath = join(pnpmDir, dir.name, 'node_modules', '@jbcom', 'agentic-triage');
  const distPath = join(triagePath, 'dist');
  const packagePath = join(triagePath, 'package.json');

  if (existsSync(packagePath) && !existsSync(distPath)) {
    console.log(`Building @jbcom/agentic-triage in ${triagePath}...`);
    try {
      execSync('pnpm run build', { cwd: triagePath, stdio: 'inherit' });
      console.log('Successfully built @jbcom/agentic-triage');
    } catch (error) {
      console.error('Failed to build @jbcom/agentic-triage:', error.message);
      process.exit(1);
    }
  } else if (existsSync(distPath)) {
    console.log('@jbcom/agentic-triage already built');
  }
}
