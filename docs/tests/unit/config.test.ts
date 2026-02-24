/**
 * Configuration Validation Tests
 *
 * These tests validate the Astro and Starlight configuration for the Agentic
 * documentation site. They check site URL, sidebar structure, TypeDoc plugin
 * setup, and content file integrity without requiring a build.
 */

import { describe, it, expect } from 'vitest';
import { readFileSync, existsSync } from 'node:fs';
import { resolve, join } from 'node:path';

const DOCS_ROOT = resolve(import.meta.dirname, '../..');
const CONFIG_PATH = join(DOCS_ROOT, 'astro.config.mjs');
const CONTENT_DIR = join(DOCS_ROOT, 'src/content/docs');

/**
 * Read the raw config file as text for pattern matching.
 * We cannot dynamically import astro.config.mjs here because it depends
 * on Astro/Starlight runtime — instead we parse it as a string.
 */
function readConfig(): string {
  return readFileSync(CONFIG_PATH, 'utf-8');
}

// ---------------------------------------------------------------------------
// 1. Site Configuration
// ---------------------------------------------------------------------------

describe('Astro site configuration', () => {
  it('astro.config.mjs exists', () => {
    expect(existsSync(CONFIG_PATH)).toBe(true);
  });

  it('site URL is set to https://agentic.coach', () => {
    const config = readConfig();
    expect(config).toContain("site: 'https://agentic.coach'");
  });

  it('uses the starlight integration', () => {
    const config = readConfig();
    expect(config).toContain('starlight(');
  });

  it('site title is set to "Agentic"', () => {
    const config = readConfig();
    expect(config).toContain("title: 'Agentic'");
  });

  it('includes a description for the site', () => {
    const config = readConfig();
    expect(config).toContain('description:');
    expect(config).toContain('polyglot');
  });

  it('configures logo with light and dark variants', () => {
    const config = readConfig();
    expect(config).toContain('logo:');
    expect(config).toContain('logo-light.svg');
    expect(config).toContain('logo-dark.svg');
  });

  it('includes GitHub social link', () => {
    const config = readConfig();
    expect(config).toContain("icon: 'github'");
    expect(config).toContain('github.com/jbcom/agentic');
  });

  it('includes custom CSS reference', () => {
    const config = readConfig();
    expect(config).toContain('customCss');
    expect(config).toContain('custom.css');
  });

  it('includes og:image meta tag in head config', () => {
    const config = readConfig();
    expect(config).toContain("property: 'og:image'");
    expect(config).toContain('og-image.svg');
  });
});

// ---------------------------------------------------------------------------
// 2. Sidebar Structure
// ---------------------------------------------------------------------------

describe('Sidebar structure', () => {
  it('defines a "Getting Started" section', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Getting Started'");
  });

  it('defines a "Packages" section', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Packages'");
  });

  it('defines a "Guides" section', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Guides'");
  });

  it('defines an "Examples" section', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Examples'");
  });

  it('defines an "Integrations" section', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Integrations'");
  });

  it('defines an "API Reference" section', () => {
    const config = readConfig();
    expect(config).toContain("label: 'API Reference'");
  });

  it('includes typeDocSidebarGroup in sidebar', () => {
    const config = readConfig();
    expect(config).toContain('typeDocSidebarGroup');
  });

  it('includes Architecture and Troubleshooting guides', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Architecture'");
    expect(config).toContain("slug: 'guides/architecture'");
    expect(config).toContain("label: 'Troubleshooting'");
    expect(config).toContain("slug: 'guides/troubleshooting'");
  });

  it('includes crew and game-gen API Reference entries', () => {
    const config = readConfig();
    expect(config).toContain("label: 'Python (agentic-crew)'");
    expect(config).toContain("slug: 'api/crew'");
    expect(config).toContain("label: 'Rust (game-gen)'");
    expect(config).toContain("slug: 'api/game-gen'");
  });
});

// ---------------------------------------------------------------------------
// 3. Sidebar Slugs Have Corresponding Content Files
// ---------------------------------------------------------------------------

describe('Sidebar slugs have corresponding content files', () => {
  /**
   * Map sidebar slug to expected content file path relative to content/docs/.
   * Files may be .md or .mdx.
   */
  const slugToFile: Record<string, string[]> = {
    'getting-started/introduction': ['getting-started/introduction.md', 'getting-started/introduction.mdx'],
    'getting-started/quick-start': ['getting-started/quick-start.md', 'getting-started/quick-start.mdx'],
    'getting-started/configuration': ['getting-started/configuration.md', 'getting-started/configuration.mdx'],
    'packages/control': ['packages/control.md', 'packages/control.mdx'],
    'packages/triage': ['packages/triage.md', 'packages/triage.mdx'],
    'packages/crew': ['packages/crew.md', 'packages/crew.mdx'],
    'packages/meshy-content-generator': ['packages/meshy-content-generator.md', 'packages/meshy-content-generator.mdx'],
    'packages/game-generator': ['packages/game-generator.md', 'packages/game-generator.mdx'],
    'guides/agent-spawning': ['guides/agent-spawning.md', 'guides/agent-spawning.mdx'],
    'guides/fleet-management': ['guides/fleet-management.md', 'guides/fleet-management.mdx'],
    'guides/orchestration-patterns': ['guides/orchestration-patterns.md', 'guides/orchestration-patterns.mdx'],
    'guides/ai-triage': ['guides/ai-triage.md', 'guides/ai-triage.mdx'],
    'guides/sandbox-execution': ['guides/sandbox-execution.md', 'guides/sandbox-execution.mdx'],
    'examples/typescript': ['examples/typescript.md', 'examples/typescript.mdx'],
    'examples/python': ['examples/python.md', 'examples/python.mdx'],
    'examples/cli-workflows': ['examples/cli-workflows.md', 'examples/cli-workflows.mdx'],
    'integrations/github-actions': ['integrations/github-actions.md', 'integrations/github-actions.mdx'],
    'integrations/crewai': ['integrations/crewai.md', 'integrations/crewai.mdx'],
    'integrations/langgraph': ['integrations/langgraph.md', 'integrations/langgraph.mdx'],
    'integrations/vercel-ai-sdk': ['integrations/vercel-ai-sdk.md', 'integrations/vercel-ai-sdk.mdx'],
    'integrations/mcp-server': ['integrations/mcp-server.md', 'integrations/mcp-server.mdx'],
    'api/fleet-management': ['api/fleet-management.md', 'api/fleet-management.mdx'],
    'api/triage-tools': ['api/triage-tools.md', 'api/triage-tools.mdx'],
    'api/token-management': ['api/token-management.md', 'api/token-management.mdx'],
    'api/configuration': ['api/configuration.md', 'api/configuration.mdx'],
    'api/crew': ['api/crew/index.md', 'api/crew/index.mdx', 'api/crew.md', 'api/crew.mdx'],
    'api/game-gen': ['api/game-gen/index.md', 'api/game-gen/index.mdx', 'api/game-gen.md', 'api/game-gen.mdx'],
    'guides/architecture': ['guides/architecture.md', 'guides/architecture.mdx'],
    'guides/troubleshooting': ['guides/troubleshooting.md', 'guides/troubleshooting.mdx'],
  };

  for (const [slug, possibleFiles] of Object.entries(slugToFile)) {
    it(`slug "${slug}" has a content file`, () => {
      const found = possibleFiles.some((file) =>
        existsSync(join(CONTENT_DIR, file))
      );
      expect(found).toBe(true);
    });
  }
});

// ---------------------------------------------------------------------------
// 4. Content Files Have Valid Frontmatter
// ---------------------------------------------------------------------------

describe('Content files have valid frontmatter', () => {
  const contentFiles = [
    'index.mdx',
    'getting-started/introduction.md',
    'getting-started/quick-start.md',
    'getting-started/configuration.md',
    'packages/control.md',
    'packages/triage.md',
    'guides/agent-spawning.md',
    'guides/fleet-management.md',
  ];

  for (const file of contentFiles) {
    const filePath = join(CONTENT_DIR, file);
    if (!existsSync(filePath)) continue;

    it(`${file} has frontmatter with title`, () => {
      const content = readFileSync(filePath, 'utf-8');
      // Frontmatter starts and ends with ---
      expect(content).toMatch(/^---\n/);
      expect(content).toMatch(/title:\s*.+/);
    });

    it(`${file} has frontmatter with description`, () => {
      const content = readFileSync(filePath, 'utf-8');
      expect(content).toMatch(/description:\s*.+/);
    });
  }
});

// ---------------------------------------------------------------------------
// 5. TypeDoc Plugin Configuration
// ---------------------------------------------------------------------------

describe('TypeDoc plugin configuration', () => {
  it('configures starlightTypeDoc plugin', () => {
    const config = readConfig();
    expect(config).toContain('starlightTypeDoc(');
  });

  it('configures control package TypeDoc output', () => {
    const config = readConfig();
    expect(config).toContain("output: 'api/control'");
  });

  it('configures triage package TypeDoc output', () => {
    const config = readConfig();
    expect(config).toContain("output: 'api/triage'");
  });

  it('configures meshy package TypeDoc output', () => {
    const config = readConfig();
    expect(config).toContain("output: 'api/meshy'");
  });

  it('configures providers package TypeDoc output', () => {
    const config = readConfig();
    expect(config).toContain("output: 'api/providers'");
  });

  it('configures vitest package TypeDoc output', () => {
    const config = readConfig();
    expect(config).toContain("output: 'api/vitest'");
  });

  it('all TypeDoc plugins have skipErrorChecking enabled', () => {
    const config = readConfig();
    // Count occurrences of starlightTypeDoc( — should match skipErrorChecking count
    const pluginCount = (config.match(/starlightTypeDoc\(/g) || []).length;
    const skipCount = (config.match(/skipErrorChecking:\s*true/g) || []).length;
    expect(skipCount).toBe(pluginCount);
  });

  it('all TypeDoc plugins reference entry points', () => {
    const config = readConfig();
    const pluginCount = (config.match(/starlightTypeDoc\(/g) || []).length;
    const entryPointCount = (config.match(/entryPoints:\s*\[/g) || []).length;
    expect(entryPointCount).toBe(pluginCount);
  });

  it('all TypeDoc plugins reference tsconfig files', () => {
    const config = readConfig();
    const pluginCount = (config.match(/starlightTypeDoc\(/g) || []).length;
    const tsconfigCount = (config.match(/tsconfig:\s*'/g) || []).length;
    expect(tsconfigCount).toBe(pluginCount);
  });

  it('TypeDoc sidebar labels are descriptive', () => {
    const config = readConfig();
    expect(config).toContain("label: '@jbcom/agentic API'");
    expect(config).toContain("label: '@jbcom/agentic-triage API'");
    expect(config).toContain("label: '@jbcom/agentic-meshy API'");
    expect(config).toContain("label: '@jbcom/agentic-providers API'");
    expect(config).toContain("label: '@jbcom/vitest-agentic API'");
  });
});

// ---------------------------------------------------------------------------
// 6. Asset Files Exist
// ---------------------------------------------------------------------------

describe('Required asset files exist', () => {
  it('light logo SVG exists', () => {
    expect(existsSync(join(DOCS_ROOT, 'src/assets/logo-light.svg'))).toBe(true);
  });

  it('dark logo SVG exists', () => {
    expect(existsSync(join(DOCS_ROOT, 'src/assets/logo-dark.svg'))).toBe(true);
  });

  it('houston hero image exists', () => {
    expect(existsSync(join(DOCS_ROOT, 'src/assets/houston.webp'))).toBe(true);
  });

  it('custom CSS file exists', () => {
    expect(existsSync(join(DOCS_ROOT, 'src/styles/custom.css'))).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// 7. Package.json Validation
// ---------------------------------------------------------------------------

describe('docs package.json', () => {
  it('has correct name', () => {
    const pkg = JSON.parse(readFileSync(join(DOCS_ROOT, 'package.json'), 'utf-8'));
    expect(pkg.name).toBe('agentic-docs');
  });

  it('has build script', () => {
    const pkg = JSON.parse(readFileSync(join(DOCS_ROOT, 'package.json'), 'utf-8'));
    expect(pkg.scripts.build).toBe('astro build');
  });

  it('has preview script', () => {
    const pkg = JSON.parse(readFileSync(join(DOCS_ROOT, 'package.json'), 'utf-8'));
    expect(pkg.scripts.preview).toBe('astro preview');
  });

  it('depends on @astrojs/starlight', () => {
    const pkg = JSON.parse(readFileSync(join(DOCS_ROOT, 'package.json'), 'utf-8'));
    expect(pkg.dependencies['@astrojs/starlight']).toBeDefined();
  });

  it('depends on starlight-typedoc', () => {
    const pkg = JSON.parse(readFileSync(join(DOCS_ROOT, 'package.json'), 'utf-8'));
    expect(pkg.dependencies['starlight-typedoc']).toBeDefined();
  });

  it('depends on typedoc', () => {
    const pkg = JSON.parse(readFileSync(join(DOCS_ROOT, 'package.json'), 'utf-8'));
    expect(pkg.dependencies['typedoc']).toBeDefined();
  });
});
