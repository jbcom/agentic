/**
 * Navigation E2E Tests
 *
 * These tests verify the Agentic documentation site's navigation behavior
 * in a real browser. They cover homepage rendering, sidebar navigation,
 * search functionality, responsive layout, and external link handling.
 *
 * Prerequisites: The docs must be built (astro build) -- the Playwright
 * webServer config starts `astro preview` automatically.
 */

import { test, expect } from '@playwright/test';

// ---------------------------------------------------------------------------
// 1. Homepage
// ---------------------------------------------------------------------------

test.describe('Homepage', () => {
  test('loads and displays the site title', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/Agentic/);
  });

  test('displays the hero section with tagline', async ({ page }) => {
    await page.goto('/');
    const body = page.locator('body');
    await expect(body).toContainText('polyglot');
  });

  test('displays the "Get Started" call-to-action', async ({ page }) => {
    await page.goto('/');
    const ctaLink = page.locator('a', { hasText: /get started/i });
    await expect(ctaLink.first()).toBeVisible();
  });

  test('"Get Started" link navigates to introduction page', async ({ page }) => {
    await page.goto('/');
    const ctaLink = page.locator('a', { hasText: /get started/i }).first();
    await ctaLink.click();
    await page.waitForURL(/getting-started\/introduction/);
    await expect(page).toHaveTitle(/Introduction/);
  });

  test('displays the language badges', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.lang-badge--ts').first()).toBeVisible();
    await expect(page.locator('.lang-badge--py').first()).toBeVisible();
    await expect(page.locator('.lang-badge--rs').first()).toBeVisible();
  });

  test('displays the stats section', async ({ page }) => {
    await page.goto('/');
    const stats = page.locator('.stats-row');
    await expect(stats).toBeVisible();
    await expect(stats).toContainText('Packages');
    await expect(stats).toContainText('Languages');
  });

  test('displays the ecosystem section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('body')).toContainText('Ecosystem');
  });
});

// ---------------------------------------------------------------------------
// 2. Sidebar Navigation
// ---------------------------------------------------------------------------

test.describe('Sidebar navigation', () => {
  test('sidebar is visible on desktop', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    // Starlight sidebar content pane inside <nav aria-label="Main">
    const sidebarPane = page.locator('#starlight__sidebar');
    await expect(sidebarPane).toBeVisible();
  });

  test('sidebar contains "Getting Started" group', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    await expect(sidebarPane).toContainText('Getting Started');
  });

  test('sidebar contains "Guides" group', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    await expect(sidebarPane).toContainText('Guides');
  });

  test('sidebar contains "Packages" group', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    await expect(sidebarPane).toContainText('Packages');
  });

  test('clicking sidebar link navigates to correct page', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    const quickStartLink = sidebarPane.locator('a', { hasText: 'Quick Start' }).first();
    await quickStartLink.click();
    await page.waitForURL(/getting-started\/quick-start/);
    await expect(page).toHaveTitle(/Quick Start/);
  });

  test('clicking through to a guide page works', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    const guideLink = sidebarPane.locator('a', { hasText: 'Agent Spawning' }).first();
    await guideLink.click();
    await page.waitForURL(/guides\/agent-spawning/);
    await expect(page.locator('h1').first()).toContainText(/spawn/i);
  });

  test('clicking through to an integration page works', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    const integrationLink = sidebarPane.locator('a', { hasText: 'GitHub Actions' }).first();
    await integrationLink.click();
    await page.waitForURL(/integrations\/github-actions/);
    await expect(page).toHaveTitle(/GitHub Actions/);
  });

  test('current page is highlighted in sidebar', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const sidebarPane = page.locator('#starlight__sidebar');
    const currentLink = sidebarPane.locator('a[aria-current="page"]');
    await expect(currentLink).toBeVisible();
    await expect(currentLink).toContainText('Introduction');
  });
});

// ---------------------------------------------------------------------------
// 3. Content Pages
// ---------------------------------------------------------------------------

test.describe('Content pages', () => {
  test('Introduction page renders with expected content', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page.locator('main')).toContainText('Agentic');
  });

  test('Configuration page renders with expected content', async ({ page }) => {
    await page.goto('/getting-started/configuration/');
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page).toHaveTitle(/Configuration/);
  });

  test('Fleet Management API page renders', async ({ page }) => {
    await page.goto('/api/fleet-management/');
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page.locator('main')).toContainText(/fleet/i);
  });

  test('TypeScript examples page renders', async ({ page }) => {
    await page.goto('/examples/typescript/');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('Python examples page renders', async ({ page }) => {
    await page.goto('/examples/python/');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('all package pages are accessible', async ({ page }) => {
    const packagePaths = [
      '/packages/control/',
      '/packages/triage/',
      '/packages/crew/',
      '/packages/meshy-content-generator/',
      '/packages/game-generator/',
    ];

    for (const path of packagePaths) {
      const response = await page.goto(path);
      expect(response?.status()).toBe(200);
      await expect(page.locator('h1').first()).toBeVisible();
    }
  });
});

// ---------------------------------------------------------------------------
// 4. Search Functionality
// ---------------------------------------------------------------------------

test.describe('Search', () => {
  test('search button is visible', async ({ page }) => {
    await page.goto('/');
    // Starlight search: <button data-open-modal aria-label="Search">
    const searchButton = page.locator('button[data-open-modal]');
    await expect(searchButton).toBeVisible();
  });

  test('clicking search opens the search dialog', async ({ page }) => {
    await page.goto('/');
    const searchButton = page.locator('button[data-open-modal]');
    await searchButton.click();

    // Starlight opens a <dialog> for search
    const dialog = page.locator('dialog[aria-label="Search"]');
    await expect(dialog).toBeVisible({ timeout: 5000 });
  });

  test('search input accepts text', async ({ page }) => {
    await page.goto('/');
    const searchButton = page.locator('button[data-open-modal]');
    await searchButton.click();

    const dialog = page.locator('dialog[aria-label="Search"]');
    await expect(dialog).toBeVisible({ timeout: 5000 });

    const searchInput = dialog.locator('input[type="search"]').or(dialog.locator('input'));
    const input = searchInput.first();
    await expect(input).toBeVisible({ timeout: 5000 });
    await input.fill('fleet');
    await expect(input).toHaveValue('fleet');
  });

  test('keyboard shortcut opens search (Ctrl+K)', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    // Starlight supports Ctrl+K (or Meta+K on Mac) to open search
    await page.keyboard.press('Control+k');

    const dialog = page.locator('dialog[aria-label="Search"]');
    await expect(dialog).toBeVisible({ timeout: 5000 });
  });
});

// ---------------------------------------------------------------------------
// 5. Subpage Navigation (Deep Links)
// ---------------------------------------------------------------------------

test.describe('Subpage deep links', () => {
  test('navigating to a subpage directly works', async ({ page }) => {
    const response = await page.goto('/guides/ai-triage/');
    expect(response?.status()).toBe(200);
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('navigating to sandbox execution page works', async ({ page }) => {
    const response = await page.goto('/guides/sandbox-execution/');
    expect(response?.status()).toBe(200);
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('navigating to CLI workflows page works', async ({ page }) => {
    const response = await page.goto('/examples/cli-workflows/');
    expect(response?.status()).toBe(200);
    await expect(page.locator('h1').first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// 6. External Links
// ---------------------------------------------------------------------------

test.describe('External links', () => {
  test('GitHub link in header/social points to correct repo', async ({ page }) => {
    await page.goto('/');
    const githubLink = page.locator('a[href*="github.com/jbcom/agentic"]').first();
    await expect(githubLink).toBeVisible();
    const href = await githubLink.getAttribute('href');
    expect(href).toContain('github.com/jbcom/agentic');
  });

  test('"View on GitHub" hero link targets correct repository', async ({ page }) => {
    await page.goto('/');
    const viewOnGitHub = page.locator('a', { hasText: /view on github/i }).first();
    if (await viewOnGitHub.isVisible()) {
      const href = await viewOnGitHub.getAttribute('href');
      expect(href).toContain('github.com/jbcom/agentic');
    }
  });
});

// ---------------------------------------------------------------------------
// 7. Mobile Responsive Menu
// ---------------------------------------------------------------------------

test.describe('Mobile responsive menu', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('sidebar is behind a menu button on mobile', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    // Starlight: <starlight-menu-button> contains the hamburger
    const menuButton = page.locator('starlight-menu-button button');
    await expect(menuButton).toBeVisible();
  });

  test('menu toggle button opens sidebar on mobile', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const menuButton = page.locator('starlight-menu-button button');
    await menuButton.click();

    // After clicking, the sidebar pane should become visible
    const sidebarPane = page.locator('#starlight__sidebar');
    await expect(sidebarPane).toBeVisible({ timeout: 3000 });
  });

  test('mobile menu contains navigation links', async ({ page }) => {
    await page.goto('/getting-started/introduction/');
    const menuButton = page.locator('starlight-menu-button button');
    await menuButton.click();

    const sidebarPane = page.locator('#starlight__sidebar');
    await expect(sidebarPane).toBeVisible({ timeout: 3000 });
    await expect(sidebarPane).toContainText('Getting Started');
    await expect(sidebarPane).toContainText('Guides');
  });
});

// ---------------------------------------------------------------------------
// 8. TypeDoc API Pages
// ---------------------------------------------------------------------------

test.describe('TypeDoc API pages', () => {
  test('control API readme page is accessible', async ({ page }) => {
    const response = await page.goto('/api/control/readme/');
    expect(response?.status()).toBe(200);
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('API page contains class or interface documentation', async ({ page }) => {
    await page.goto('/api/control/readme/');
    const mainContent = page.locator('main');
    await expect(mainContent).toBeVisible();
    const text = await mainContent.textContent();
    expect(text!.length).toBeGreaterThan(50);
  });

  test('Fleet class documentation page is accessible', async ({ page }) => {
    const response = await page.goto('/api/control/classes/fleet/');
    expect(response?.status()).toBe(200);
    await expect(page.locator('h1').first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// 9. 404 Handling
// ---------------------------------------------------------------------------

test.describe('404 handling', () => {
  test('non-existent page returns 404 content', async ({ page }) => {
    await page.goto('/this-page-definitely-does-not-exist/');
    const body = page.locator('body');
    const text = await body.textContent();
    expect(
      text!.toLowerCase().includes('not found') ||
      text!.includes('404')
    ).toBeTruthy();
  });
});

// ---------------------------------------------------------------------------
// 10. Page Load Performance
// ---------------------------------------------------------------------------

test.describe('Page load performance', () => {
  test('homepage loads within 5 seconds', async ({ page }) => {
    const start = Date.now();
    await page.goto('/', { waitUntil: 'domcontentloaded' });
    const elapsed = Date.now() - start;
    expect(elapsed).toBeLessThan(5000);
  });

  test('content page loads within 5 seconds', async ({ page }) => {
    const start = Date.now();
    await page.goto('/getting-started/introduction/', { waitUntil: 'domcontentloaded' });
    const elapsed = Date.now() - start;
    expect(elapsed).toBeLessThan(5000);
  });
});
