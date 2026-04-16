import { experimental_createMCPClient as createMCPClient } from '@ai-sdk/mcp';
import { Experimental_StdioMCPTransport as StdioMCPTransport } from '@ai-sdk/mcp/mcp-stdio';

export type MCPClient = Awaited<ReturnType<typeof createMCPClient>>;

/**
 * Create GitHub MCP client for issue/PR/project operations
 *
 * Provides access to:
 * - create_issue, update_issue, get_issue
 * - create_pull_request, merge_pull_request
 * - add_label, remove_label
 * - get_file_contents, create_or_update_file
 * - search_issues, search_repositories
 */
export async function createGitHubClient(): Promise<MCPClient> {
    const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;
    if (!token) {
        throw new Error('GITHUB_TOKEN or GH_TOKEN required for GitHub MCP');
    }

    const transport = new StdioMCPTransport({
        command: 'npx',
        args: ['-y', '@modelcontextprotocol/server-github'],
        env: {
            ...process.env,
            GITHUB_PERSONAL_ACCESS_TOKEN: token,
        },
    });

    return createMCPClient({ transport });
}

/**
 * Create GraphQL MCP client for GitHub GraphQL API operations
 *
 * Uses mcp-graphql to execute GraphQL queries/mutations against GitHub's API.
 * Required for: Projects V2, review threads, draft PR conversion, auto-merge, etc.
 */
export async function createGraphQLClient(): Promise<MCPClient> {
    const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;
    if (!token) {
        throw new Error('GITHUB_TOKEN or GH_TOKEN required for GraphQL MCP');
    }

    // mcp-graphql v1.0.0+ uses environment variables instead of CLI args
    const transport = new StdioMCPTransport({
        command: 'npx',
        args: ['-y', 'mcp-graphql'],
        env: {
            ...process.env,
            ENDPOINT: 'https://api.github.com/graphql',
            HEADERS: JSON.stringify({
                Authorization: `Bearer ${token}`,
            }),
        },
    });

    return createMCPClient({
        transport,
        name: 'strata-triage-graphql',
        version: '1.0.0',
    });
}
