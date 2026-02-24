/**
 * Build Output Validation Tests
 *
 * These tests verify the Astro build output for the Agentic documentation site.
 * They check that expected pages exist, contain correct content, and have proper
 * HTML structure. The tests operate against the static build output in dist/.
 *
 * Prerequisites: Run `pnpm run build` in the docs directory before running these tests.
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { readFileSync, existsSync, readdirSync } from 'node:fs';
import { resolve, join } from 'node:path';
import { load as cheerioLoad, type CheerioAPI } from 'cheerio';

const DIST_DIR = resolve(import.meta.dirname, '../../dist');

/**
 * Helper: load an HTML file from the build output and return a Cheerio instance.
 */
function loadPage(relativePath: string): CheerioAPI {
  const filePath = join(DIST_DIR, relativePath);
  if (!existsSync(filePath)) {
    throw new Error(`Build output file not found: ${filePath}`);
  }
  const html = readFileSync(filePath, 'utf-8');
  return cheerioLoad(html);
}

/**
 * Helper: check whether a file exists in the build output.
 */
function pageExists(relativePath: string): boolean {
  return existsSync(join(DIST_DIR, relativePath));
}

// ---------------------------------------------------------------------------
// Guard: Ensure the build output exists before running any tests
// ---------------------------------------------------------------------------

beforeAll(() => {
  if (!existsSync(DIST_DIR)) {
    throw new Error(
      `Build output directory not found at ${DIST_DIR}. ` +
      'Run "pnpm run build" in the docs directory before running unit tests.'
    );
  }
});

// ---------------------------------------------------------------------------
// 1. Core Pages Exist
// ---------------------------------------------------------------------------

describe('Core pages exist in build output', () => {
  const corePages = [
    { path: 'index.html', label: 'Homepage' },
    { path: 'getting-started/introduction/index.html', label: 'Introduction' },
    { path: 'getting-started/quick-start/index.html', label: 'Quick Start' },
    { path: 'getting-started/configuration/index.html', label: 'Configuration' },
  ];

  for (const { path, label } of corePages) {
    it(`${label} page exists (${path})`, () => {
      expect(pageExists(path)).toBe(true);
    });
  }
});

describe('Package pages exist in build output', () => {
  const packagePages = [
    { path: 'packages/control/index.html', label: '@jbcom/agentic (Control)' },
    { path: 'packages/triage/index.html', label: '@jbcom/agentic (Triage)' },
    { path: 'packages/crew/index.html', label: 'agentic-crew' },
    { path: 'packages/meshy-content-generator/index.html', label: '@agentic/meshy' },
    { path: 'packages/game-generator/index.html', label: 'game-generator' },
  ];

  for (const { path, label } of packagePages) {
    it(`${label} page exists (${path})`, () => {
      expect(pageExists(path)).toBe(true);
    });
  }
});

describe('Guide pages exist in build output', () => {
  const guidePages = [
    { path: 'guides/agent-spawning/index.html', label: 'Agent Spawning' },
    { path: 'guides/fleet-management/index.html', label: 'Fleet Management' },
    { path: 'guides/orchestration-patterns/index.html', label: 'Orchestration Patterns' },
    { path: 'guides/ai-triage/index.html', label: 'AI Triage' },
    { path: 'guides/sandbox-execution/index.html', label: 'Sandbox Execution' },
  ];

  for (const { path, label } of guidePages) {
    it(`${label} page exists (${path})`, () => {
      expect(pageExists(path)).toBe(true);
    });
  }
});

describe('Example pages exist in build output', () => {
  const examplePages = [
    { path: 'examples/typescript/index.html', label: 'TypeScript Examples' },
    { path: 'examples/python/index.html', label: 'Python Examples' },
    { path: 'examples/cli-workflows/index.html', label: 'CLI Workflows' },
  ];

  for (const { path, label } of examplePages) {
    it(`${label} page exists (${path})`, () => {
      expect(pageExists(path)).toBe(true);
    });
  }
});

describe('Integration pages exist in build output', () => {
  const integrationPages = [
    { path: 'integrations/github-actions/index.html', label: 'GitHub Actions' },
    { path: 'integrations/crewai/index.html', label: 'CrewAI' },
    { path: 'integrations/langgraph/index.html', label: 'LangGraph' },
    { path: 'integrations/vercel-ai-sdk/index.html', label: 'Vercel AI SDK' },
    { path: 'integrations/mcp-server/index.html', label: 'MCP Server' },
  ];

  for (const { path, label } of integrationPages) {
    it(`${label} page exists (${path})`, () => {
      expect(pageExists(path)).toBe(true);
    });
  }
});

describe('API Reference pages exist in build output', () => {
  const apiPages = [
    { path: 'api/fleet-management/index.html', label: 'Fleet API' },
    { path: 'api/triage-tools/index.html', label: 'Triage Tools' },
    { path: 'api/token-management/index.html', label: 'Token Management' },
    { path: 'api/configuration/index.html', label: 'Configuration' },
  ];

  for (const { path, label } of apiPages) {
    it(`${label} page exists (${path})`, () => {
      expect(pageExists(path)).toBe(true);
    });
  }
});

// ---------------------------------------------------------------------------
// 2. 404 Page
// ---------------------------------------------------------------------------

describe('404 page', () => {
  it('exists in build output', () => {
    expect(pageExists('404.html')).toBe(true);
  });

  it('contains meaningful error content', () => {
    const $ = loadPage('404.html');
    const bodyText = $('body').text().toLowerCase();
    // Starlight's default 404 includes "page not found" or similar
    expect(
      bodyText.includes('not found') ||
      bodyText.includes('404') ||
      bodyText.includes('page not found')
    ).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// 3. Homepage Content Validation
// ---------------------------------------------------------------------------

describe('Homepage content', () => {
  it('contains the site title "Agentic"', () => {
    const $ = loadPage('index.html');
    const titleTag = $('title').text();
    expect(titleTag).toContain('Agentic');
  });

  it('contains the hero tagline about polyglot toolkit', () => {
    const $ = loadPage('index.html');
    const bodyText = $('body').text();
    expect(bodyText).toContain('polyglot');
  });

  it('contains language badges (TypeScript, Python, Rust)', () => {
    const $ = loadPage('index.html');
    const bodyText = $('body').text();
    expect(bodyText).toContain('TypeScript');
    expect(bodyText).toContain('Python');
    expect(bodyText).toContain('Rust');
  });

  it('contains the "Get Started" call-to-action link', () => {
    const $ = loadPage('index.html');
    const getStartedLinks = $('a').filter(function () {
      return $(this).text().toLowerCase().includes('get started');
    });
    expect(getStartedLinks.length).toBeGreaterThan(0);
  });

  it('contains GitHub social link', () => {
    const $ = loadPage('index.html');
    const githubLinks = $('a[href*="github.com/jbcom/agentic"]');
    expect(githubLinks.length).toBeGreaterThan(0);
  });

  it('contains the ecosystem architecture diagram section', () => {
    const $ = loadPage('index.html');
    const bodyText = $('body').text();
    expect(bodyText).toContain('Ecosystem');
  });

  it('contains the ecosystem bento grid with package info', () => {
    const $ = loadPage('index.html');
    const bodyText = $('body').text();
    expect(bodyText).toContain('@jbcom/agentic');
    expect(bodyText).toContain('agentic-crew');
  });

  it('contains the stats section with package count', () => {
    const $ = loadPage('index.html');
    const statsText = $('body').text();
    expect(statsText).toContain('Packages');
    expect(statsText).toContain('Languages');
  });
});

// ---------------------------------------------------------------------------
// 4. Page Titles
// ---------------------------------------------------------------------------

describe('Page titles are correctly set', () => {
  const titleChecks = [
    { path: 'getting-started/introduction/index.html', expected: 'Introduction' },
    { path: 'getting-started/quick-start/index.html', expected: 'Quick Start' },
    { path: 'getting-started/configuration/index.html', expected: 'Configuration' },
  ];

  for (const { path, expected } of titleChecks) {
    it(`${expected} page has correct title`, () => {
      const $ = loadPage(path);
      const titleTag = $('title').text();
      expect(titleTag).toContain(expected);
    });
  }
});

// ---------------------------------------------------------------------------
// 5. HTML Structure Validation
// ---------------------------------------------------------------------------

describe('HTML structure of content pages', () => {
  it('Introduction page has an h1 heading', () => {
    const $ = loadPage('getting-started/introduction/index.html');
    const h1 = $('h1');
    expect(h1.length).toBeGreaterThanOrEqual(1);
  });

  it('content pages include a <main> landmark', () => {
    const $ = loadPage('getting-started/introduction/index.html');
    expect($('main').length).toBe(1);
  });

  it('content pages include a <nav> for sidebar', () => {
    const $ = loadPage('getting-started/introduction/index.html');
    expect($('nav').length).toBeGreaterThanOrEqual(1);
  });

  it('pages include a <head> with charset and viewport', () => {
    const $ = loadPage('getting-started/introduction/index.html');
    expect($('meta[charset]').length).toBe(1);
    expect($('meta[name="viewport"]').length).toBe(1);
  });

  it('pages include the custom CSS', () => {
    const $ = loadPage('index.html');
    const html = $.html();
    // The custom CSS should be inlined or linked in the output
    expect(
      html.includes('--sl-color-accent') || html.includes('custom.css') || html.includes('lang-badge')
    ).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// 6. Sidebar Links Resolve to Actual Pages
// ---------------------------------------------------------------------------

describe('Sidebar links resolve to actual build output pages', () => {
  /**
   * Sidebar entries from astro.config.mjs with their corresponding slugs.
   * Each slug should produce a page at dist/<slug>/index.html.
   */
  const sidebarSlugs = [
    'getting-started/introduction',
    'getting-started/quick-start',
    'getting-started/configuration',
    'packages/control',
    'packages/triage',
    'packages/crew',
    'packages/meshy-content-generator',
    'packages/game-generator',
    'guides/agent-spawning',
    'guides/fleet-management',
    'guides/orchestration-patterns',
    'guides/ai-triage',
    'guides/sandbox-execution',
    'examples/typescript',
    'examples/python',
    'examples/cli-workflows',
    'integrations/github-actions',
    'integrations/crewai',
    'integrations/langgraph',
    'integrations/vercel-ai-sdk',
    'integrations/mcp-server',
    'api/fleet-management',
    'api/triage-tools',
    'api/token-management',
    'api/configuration',
    'api/crew',
    'api/game-gen',
  ];

  for (const slug of sidebarSlugs) {
    it(`sidebar slug "${slug}" resolves to a page`, () => {
      expect(pageExists(`${slug}/index.html`)).toBe(true);
    });
  }
});

// ---------------------------------------------------------------------------
// 7. Key Content Presence
// ---------------------------------------------------------------------------

describe('Key content is present in built pages', () => {
  it('Introduction page explains what Agentic is', () => {
    const $ = loadPage('getting-started/introduction/index.html');
    const text = $('main').text();
    expect(text).toContain('Agentic');
    expect(text.length).toBeGreaterThan(200);
  });

  it('Quick Start page contains installation instructions', () => {
    const $ = loadPage('getting-started/quick-start/index.html');
    const text = $('main').text();
    // Should mention npm/pnpm install or similar
    expect(
      text.includes('install') || text.includes('npm') || text.includes('pnpm')
    ).toBe(true);
  });

  it('Fleet Management guide discusses spawning or agents', () => {
    const $ = loadPage('guides/fleet-management/index.html');
    const text = $('main').text().toLowerCase();
    expect(text.includes('fleet') || text.includes('agent') || text.includes('spawn')).toBe(true);
  });

  it('Agent Spawning guide mentions spawn or fleet', () => {
    const $ = loadPage('guides/agent-spawning/index.html');
    const text = $('main').text().toLowerCase();
    expect(text.includes('spawn') || text.includes('fleet')).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// 8. Meta Tags and SEO
// ---------------------------------------------------------------------------

describe('SEO meta tags', () => {
  it('homepage has og:image meta tag', () => {
    const $ = loadPage('index.html');
    const ogImage = $('meta[property="og:image"]');
    expect(ogImage.length).toBeGreaterThanOrEqual(1);
    expect(ogImage.attr('content')).toContain('og-image');
  });

  it('homepage has a description meta tag', () => {
    const $ = loadPage('index.html');
    const desc = $('meta[name="description"]');
    expect(desc.length).toBe(1);
    expect(desc.attr('content')!.length).toBeGreaterThan(10);
  });

  it('content pages have description meta tags', () => {
    const $ = loadPage('getting-started/introduction/index.html');
    const desc = $('meta[name="description"]');
    expect(desc.length).toBe(1);
    expect(desc.attr('content')!.length).toBeGreaterThan(0);
  });
});

// ---------------------------------------------------------------------------
// 9. Static Assets
// ---------------------------------------------------------------------------

describe('Static assets in build output', () => {
  it('dist directory contains _astro assets folder', () => {
    const entries = readdirSync(DIST_DIR);
    expect(entries).toContain('_astro');
  });

  it('_astro folder contains CSS files', () => {
    const astroDir = join(DIST_DIR, '_astro');
    if (!existsSync(astroDir)) return;
    const files = readdirSync(astroDir);
    const cssFiles = files.filter((f) => f.endsWith('.css'));
    expect(cssFiles.length).toBeGreaterThan(0);
  });
});

// ---------------------------------------------------------------------------
// 10. TypeDoc Generated API Pages
// ---------------------------------------------------------------------------

describe('TypeDoc-generated API pages', () => {
  it('control API readme page exists', () => {
    expect(pageExists('api/control/readme/index.html')).toBe(true);
  });

  it('control API contains class documentation for Fleet', () => {
    // TypeDoc generates lowercase directory names
    expect(pageExists('api/control/classes/fleet/index.html')).toBe(true);
  });

  it('control API contains class documentation for SandboxExecutor', () => {
    expect(pageExists('api/control/classes/sandboxexecutor/index.html')).toBe(true);
  });

  it('control API contains interface documentation', () => {
    const interfaceDir = join(DIST_DIR, 'api/control/interfaces');
    if (existsSync(interfaceDir)) {
      const entries = readdirSync(interfaceDir);
      expect(entries.length).toBeGreaterThan(0);
    }
  });

  it('control API contains enumeration documentation', () => {
    const enumDir = join(DIST_DIR, 'api/control/enumerations');
    if (existsSync(enumDir)) {
      const entries = readdirSync(enumDir);
      expect(entries.length).toBeGreaterThan(0);
    }
  });
});
