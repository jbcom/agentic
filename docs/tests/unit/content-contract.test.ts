import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const DOCS_ROOT = resolve(import.meta.dirname, '../../src/content/docs');

function readDoc(relativePath: string): string {
  return readFileSync(join(DOCS_ROOT, relativePath), 'utf-8');
}

describe('Documentation contract', () => {
  const bannedFragments = [
    '@jbcom/agentic/tools',
    '@agentic/meshy',
    'vintage-game-generator',
    'vintage_game_generator::prelude::*',
    'QuickGenerator::new()',
    'VintageGamePlugin::new',
    '/packages/game-generator/',
    '/api/game-gen/',
    'game-asset-mcp',
  ];

  const docsToScan = [
    'index.mdx',
    'getting-started/introduction.md',
    'getting-started/quick-start.md',
    'integrations/vercel-ai-sdk.md',
    'packages/triage.md',
    'packages/meshy-content-generator.md',
    'examples/typescript.md',
  ];

  it('does not reference retired package names or fabricated APIs', () => {
    for (const doc of docsToScan) {
      const content = readDoc(doc);
      for (const fragment of bannedFragments) {
        expect(content).not.toContain(fragment);
      }
    }
  });

  it('uses the canonical triage package in public TypeScript examples', () => {
    const docs = [
      readDoc('packages/triage.md'),
      readDoc('integrations/vercel-ai-sdk.md'),
      readDoc('getting-started/quick-start.md'),
      readDoc('examples/typescript.md'),
      readDoc('index.mdx'),
    ];

    for (const content of docs) {
      expect(content).toContain('@jbcom/agentic-triage');
    }
  });

  it('uses the canonical meshy package name in product pages', () => {
    expect(readDoc('packages/meshy-content-generator.md')).toContain('@jbcom/agentic-meshy');
  });
});
