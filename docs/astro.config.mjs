// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://agentic.coach',
	integrations: [
		starlight({
			title: 'Agentic',
			description: 'Unified AI agent fleet management, orchestration, and triage toolkit - powering strata.game',
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
						content: 'https://agentic.coach/og-image.png',
					},
				},
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
					label: 'Core Packages',
					items: [
						{ label: '@agentic/control', slug: 'packages/control' },
						{ label: '@agentic/crew', slug: 'packages/crew' },
						{ label: '@agentic/triage', slug: 'packages/triage' },
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
					],
				},
				{
					label: 'Live Examples',
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
					],
				},
				{
					label: 'Python API (Generated)',
					autogenerate: { directory: 'api/_generated' },
				},
				{
					label: 'Enterprise',
					items: [
						{ label: 'jbcom Hub', link: 'https://jbcom.github.io' },
						{ label: 'Strata (Games)', link: 'https://strata.game' },
						{ label: 'Extended Data', link: 'https://extendeddata.dev' },
					],
				},
			],
		}),
	],
});
