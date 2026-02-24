// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightTypeDoc, { typeDocSidebarGroup } from 'starlight-typedoc';

// https://astro.build/config
export default defineConfig({
	site: 'https://agentic.coach',
	integrations: [
		starlight({
			title: 'Agentic',
			description: 'The polyglot AI agent toolkit — fleet orchestration, crew management, triage, and game generation across TypeScript, Python, and Rust.',
			logo: {
				light: './src/assets/logo-light.svg',
				dark: './src/assets/logo-dark.svg',
				replacesTitle: false,
			},
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/jbcom/agentic' },
			],
			customCss: [
				'./src/styles/custom.css',
			],
			head: [
				{
					tag: 'meta',
					attrs: {
						property: 'og:image',
						content: 'https://agentic.coach/og-image.svg',
					},
				},
				{
					tag: 'meta',
					attrs: {
						name: 'theme-color',
						content: '#D97706',
					},
				},
				{
					tag: 'meta',
					attrs: {
						property: 'og:title',
						content: 'Agentic — Command Your AI Fleet',
					},
				},
				{
					tag: 'meta',
					attrs: {
						property: 'og:description',
						content: 'The polyglot AI agent toolkit. Fleet orchestration, crew management, triage, and game generation across TypeScript, Python, and Rust.',
					},
				},
			],
			plugins: [
				starlightTypeDoc({
					entryPoints: ['../packages/agentic-control/src/index.ts'],
					tsconfig: '../packages/agentic-control/tsconfig.json',
					output: 'api/control',
					sidebar: { label: '@jbcom/agentic API', collapsed: true },
					typeDoc: { skipErrorChecking: true },
				}),
				starlightTypeDoc({
					entryPoints: ['../packages/triage/src/index.ts'],
					tsconfig: '../packages/triage/tsconfig.json',
					output: 'api/triage',
					sidebar: { label: '@jbcom/agentic-triage API', collapsed: true },
					typeDoc: { skipErrorChecking: true },
				}),
				starlightTypeDoc({
					entryPoints: ['../packages/meshy-content-generator/src/index.ts'],
					tsconfig: '../packages/meshy-content-generator/tsconfig.json',
					output: 'api/meshy',
					sidebar: { label: '@jbcom/agentic-meshy API', collapsed: true },
					typeDoc: { skipErrorChecking: true },
				}),
				starlightTypeDoc({
					entryPoints: ['../packages/providers/src/index.ts'],
					tsconfig: '../packages/providers/tsconfig.json',
					output: 'api/providers',
					sidebar: { label: '@jbcom/agentic-providers API', collapsed: true },
					typeDoc: { skipErrorChecking: true },
				}),
				starlightTypeDoc({
					entryPoints: ['../packages/vitest-agentic-control/src/index.ts'],
					tsconfig: '../packages/vitest-agentic-control/tsconfig.json',
					output: 'api/vitest',
					sidebar: { label: '@jbcom/vitest-agentic API', collapsed: true },
					typeDoc: { skipErrorChecking: true },
				}),
			],
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'Introduction', slug: 'getting-started/introduction' },
						{ label: 'Quick Start', slug: 'getting-started/quick-start' },
						{ label: 'Configuration', slug: 'getting-started/configuration' },
					],
				},
				{
					label: 'Packages',
					items: [
						{ label: '@jbcom/agentic (Control)', slug: 'packages/control' },
						{ label: '@jbcom/agentic (Triage)', slug: 'packages/triage' },
						{ label: 'agentic-crew', slug: 'packages/crew' },
						{ label: '@agentic/meshy', slug: 'packages/meshy-content-generator' },
						{ label: 'game-generator', slug: 'packages/game-generator' },
					],
				},
				{
					label: 'Guides',
					items: [
						{ label: 'Agent Spawning', slug: 'guides/agent-spawning' },
						{ label: 'Fleet Management', slug: 'guides/fleet-management' },
						{ label: 'Orchestration Patterns', slug: 'guides/orchestration-patterns' },
						{ label: 'AI Triage', slug: 'guides/ai-triage' },
						{ label: 'Sandbox Execution', slug: 'guides/sandbox-execution' },
						{ label: 'Architecture', slug: 'guides/architecture' },
						{ label: 'Troubleshooting', slug: 'guides/troubleshooting' },
					],
				},
				{
					label: 'Examples',
					items: [
						{ label: 'TypeScript Examples', slug: 'examples/typescript' },
						{ label: 'Python Examples', slug: 'examples/python' },
						{ label: 'CLI Workflows', slug: 'examples/cli-workflows' },
					],
				},
				{
					label: 'Integrations',
					items: [
						{ label: 'GitHub Actions', slug: 'integrations/github-actions' },
						{ label: 'CrewAI', slug: 'integrations/crewai' },
						{ label: 'LangGraph', slug: 'integrations/langgraph' },
						{ label: 'Vercel AI SDK', slug: 'integrations/vercel-ai-sdk' },
						{ label: 'MCP Server', slug: 'integrations/mcp-server' },
					],
				},
				{
					label: 'API Reference',
					items: [
						{ label: 'Fleet API', slug: 'api/fleet-management' },
						{ label: 'Triage Tools', slug: 'api/triage-tools' },
						{ label: 'Token Management', slug: 'api/token-management' },
						{ label: 'Configuration', slug: 'api/configuration' },
						{ label: 'Python (agentic-crew)', slug: 'api/crew' },
						{ label: 'Rust (game-gen)', slug: 'api/game-gen' },
					],
				},
				typeDocSidebarGroup,
			],
		}),
	],
});
