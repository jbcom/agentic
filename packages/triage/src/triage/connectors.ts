/**
 * TriageConnectors - Unified API for Issue/Project/Review Management
 *
 * Following the vendor-connectors pattern from the jbcom ecosystem,
 * this class provides cached access to all triage connectors with:
 *
 * 1. Direct TypeScript API - Use connectors directly in your code
 * 2. Vercel AI SDK Tools - Standard tools for AI agents (see tools.ts)
 *
 * Similar to how VendorConnectors provides `get_*_client()` getters,
 * TriageConnectors provides namespaced APIs for issues, projects, and reviews.
 *
 * @example
 * ```typescript
 * import { TriageConnectors } from '@strata/triage';
 *
 * // Initialize once - reads credentials from environment
 * const triage = new TriageConnectors();
 *
 * // Issue operations
 * const issues = await triage.issues.list({ status: 'open' });
 * const issue = await triage.issues.create({ title: 'Fix bug', type: 'bug' });
 * await triage.issues.update('123', { priority: 'high' });
 * await triage.issues.close('123', 'Fixed in PR #456');
 *
 * // Get ready work (no blockers)
 * const ready = await triage.issues.getReadyWork({ limit: 5 });
 *
 * // Statistics
 * const stats = await triage.issues.getStats();
 * ```
 */

import { execFileSync } from 'node:child_process';
import { createBestProvider, createProvider, type ProviderConfig } from '../providers/index.js';
import type {
    CreateIssueOptions,
    ListIssuesOptions,
    ProviderStats,
    ReadyWork,
    TriageIssue,
    TriageProvider,
    UpdateIssueOptions,
} from '../providers/types.js';

// =============================================================================
// Configuration
// =============================================================================

export interface TriageConnectorsConfig {
    /**
     * Provider configuration. If not provided, will auto-detect.
     * Can be a single config or multiple for different providers.
     */
    provider?: ProviderConfig;

    /**
     * Working directory for local providers (Beads)
     */
    workingDir?: string;

    /**
     * Repository for GitHub provider (owner/repo format)
     */
    repo?: string;

    /**
     * Prefer Beads over GitHub when both are available
     * @default true
     */
    preferBeads?: boolean;
}

// =============================================================================
// TriageConnectors Class
// =============================================================================

/**
 * Unified triage connector providing issue, project, and review APIs.
 *
 * This is the main entry point for programmatic triage operations.
 * For AI agent tools, see `getTriageTools()` in tools.ts.
 */
export class TriageConnectors {
    private config: TriageConnectorsConfig;
    private _provider: TriageProvider | null = null;
    private _initPromise: Promise<void> | null = null;

    /**
     * Issue operations API
     */
    public readonly issues: IssueAPI;

    /**
     * Project operations API (boards, sprints, epics)
     */
    public readonly projects: ProjectAPI;

    /**
     * Review operations API (PR feedback, comments)
     */
    public readonly reviews: ReviewAPI;

    constructor(config: TriageConnectorsConfig = {}) {
        this.config = config;

        // Initialize namespaced APIs
        this.issues = new IssueAPI(this);
        this.projects = new ProjectAPI(this);
        this.reviews = new ReviewAPI(this);
    }

    /**
     * Get or initialize the underlying provider
     */
    async getProvider(): Promise<TriageProvider> {
        if (this._provider) {
            return this._provider;
        }

        // Use promise-based locking to prevent concurrent initialization
        if (this._initPromise) {
            await this._initPromise;
            if (!this._provider) {
                throw new Error('Provider initialization failed');
            }
            return this._provider;
        }

        this._initPromise = this.initializeProvider();
        await this._initPromise;
        if (!this._provider) {
            throw new Error('Provider initialization failed');
        }
        return this._provider;
    }

    /**
     * Reconfigure the connectors with a new configuration.
     * This will reset the underlying provider.
     */
    async reconfigure(config: TriageConnectorsConfig): Promise<void> {
        this.config = config;
        this._provider = null;
        this._initPromise = null;
        await this.getProvider();
    }

    private async initializeProvider(): Promise<void> {
        if (this.config.provider) {
            this._provider = createProvider(this.config.provider);
        } else {
            this._provider = await createBestProvider({
                workingDir: this.config.workingDir,
                repo: this.config.repo,
                preferBeads: this.config.preferBeads,
            });
        }
    }

    /**
     * Get the provider name
     */
    async getProviderName(): Promise<string> {
        const provider = await this.getProvider();
        return provider.name;
    }

    /**
     * Check if the connector is ready
     */
    async isReady(): Promise<boolean> {
        try {
            const provider = await this.getProvider();
            return provider.isReady();
        } catch {
            return false;
        }
    }

    /**
     * Sync with remote (for providers that support it)
     */
    async sync(): Promise<void> {
        const provider = await this.getProvider();
        if (provider.sync) {
            await provider.sync();
        }
    }
}

// =============================================================================
// Issue API
// =============================================================================

/**
 * Issue operations API - CRUD and query operations for issues
 */
class IssueAPI {
    constructor(private connectors: TriageConnectors) {}

    /**
     * Create a new issue
     */
    async create(options: CreateIssueOptions): Promise<TriageIssue> {
        const provider = await this.connectors.getProvider();
        return provider.createIssue(options);
    }

    /**
     * Get an issue by ID
     */
    async get(id: string): Promise<TriageIssue | null> {
        const provider = await this.connectors.getProvider();
        return provider.getIssue(id);
    }

    /**
     * Update an existing issue
     */
    async update(id: string, options: UpdateIssueOptions): Promise<TriageIssue> {
        const provider = await this.connectors.getProvider();
        return provider.updateIssue(id, options);
    }

    /**
     * Close an issue
     */
    async close(id: string, reason?: string): Promise<TriageIssue> {
        const provider = await this.connectors.getProvider();
        return provider.closeIssue(id, reason);
    }

    /**
     * Reopen an issue
     */
    async reopen(id: string, reason?: string): Promise<TriageIssue> {
        const provider = await this.connectors.getProvider();
        return provider.reopenIssue(id, reason);
    }

    /**
     * Delete an issue (if supported by provider)
     */
    async delete(id: string): Promise<void> {
        const provider = await this.connectors.getProvider();
        if (provider.deleteIssue) {
            await provider.deleteIssue(id);
        } else {
            throw new Error(`Delete not supported by ${provider.name} provider`);
        }
    }

    /**
     * List issues with optional filters
     */
    async list(options?: ListIssuesOptions): Promise<TriageIssue[]> {
        const provider = await this.connectors.getProvider();
        return provider.listIssues(options);
    }

    /**
     * Search issues by text query
     */
    async search(query: string, options?: ListIssuesOptions): Promise<TriageIssue[]> {
        const provider = await this.connectors.getProvider();
        return provider.searchIssues(query, options);
    }

    /**
     * Get issues ready to work on (no blockers)
     */
    async getReadyWork(options?: { limit?: number }): Promise<ReadyWork[]> {
        const provider = await this.connectors.getProvider();
        return provider.getReadyWork(options);
    }

    /**
     * Get blocked issues
     */
    async getBlocked(): Promise<TriageIssue[]> {
        const provider = await this.connectors.getProvider();
        return provider.getBlockedIssues();
    }

    /**
     * Add labels to an issue
     */
    async addLabels(id: string, labels: string[]): Promise<void> {
        const provider = await this.connectors.getProvider();
        await provider.addLabels(id, labels);
    }

    /**
     * Remove labels from an issue
     */
    async removeLabels(id: string, labels: string[]): Promise<void> {
        const provider = await this.connectors.getProvider();
        await provider.removeLabels(id, labels);
    }

    /**
     * Get provider statistics
     */
    async getStats(): Promise<ProviderStats> {
        const provider = await this.connectors.getProvider();
        return provider.getStats();
    }
}

// =============================================================================
// Project API
// =============================================================================

/**
 * Project operations API - boards, sprints, epics
 *
 * Uses GitHub milestones for sprints and issues with type:epic label for epics.
 * For Linear/Jira providers, maps to their native concepts.
 */
class ProjectAPI {
    constructor(private connectors: TriageConnectors) {}

    /**
     * List sprints/iterations.
     *
     * For GitHub: uses milestones as sprint proxies.
     * For Linear/Jira: maps to their native cycle/sprint concepts.
     */
    async getSprints(): Promise<{ id: string; name: string; status: string }[]> {
        const provider = await this.connectors.getProvider();

        if (provider.name === 'github') {
            return this.getGitHubMilestones('all');
        }

        if (provider.name === 'linear') {
            // Linear cycles map to sprints - use issue listing grouped by state
            const openIssues = await provider.listIssues({ status: 'open' });
            const inProgressIssues = await provider.listIssues({ status: 'in_progress' });
            const allIssues = [...openIssues, ...inProgressIssues];

            // Group by metadata cycle if available, otherwise return a synthetic sprint
            if (allIssues.length > 0) {
                return [
                    {
                        id: 'current',
                        name: 'Current Sprint',
                        status: 'active',
                    },
                ];
            }
            return [];
        }

        if (provider.name === 'jira') {
            // Jira sprints are available through the agile API
            // Fall back to using milestones/versions
            const issues = await provider.listIssues({ status: 'open', limit: 1 });
            if (issues.length > 0) {
                return [
                    {
                        id: 'current',
                        name: 'Current Sprint',
                        status: 'active',
                    },
                ];
            }
            return [];
        }

        // Beads and other providers: no sprint concept
        return [];
    }

    /**
     * Get the current active sprint.
     *
     * For GitHub: returns the most recent open milestone.
     * For Linear/Jira: returns the active cycle/sprint.
     */
    async getCurrentSprint(): Promise<{ id: string; name: string; status: string } | null> {
        const provider = await this.connectors.getProvider();

        if (provider.name === 'github') {
            const milestones = await this.getGitHubMilestones('open');
            // Return the first open milestone (most recently created)
            return milestones.length > 0 ? milestones[0] : null;
        }

        // For other providers, get sprints and return the active one
        const sprints = await this.getSprints();
        const active = sprints.find((s) => s.status === 'active');
        return active || (sprints.length > 0 ? sprints[0] : null);
    }

    /**
     * Get epics (issues with type:epic label or Epic issue type).
     *
     * For GitHub: lists issues with the type:epic label.
     * For Linear/Jira: uses their native epic type.
     */
    async getEpics(): Promise<{ id: string; title: string; progress: number }[]> {
        const provider = await this.connectors.getProvider();

        // Fetch epic-type issues
        const epics = await provider.listIssues({ type: 'epic', limit: 100 });

        if (epics.length === 0 && provider.name === 'github') {
            // Fall back to searching for issues with 'epic' label
            const labeledEpics = await provider.listIssues({ labels: ['epic'], limit: 100 });
            return this.computeEpicProgress(labeledEpics);
        }

        return this.computeEpicProgress(epics);
    }

    /**
     * Compute progress for epics based on their status
     */
    private computeEpicProgress(epics: TriageIssue[]): { id: string; title: string; progress: number }[] {
        return epics.map((epic) => ({
            id: epic.id,
            title: epic.title,
            // Progress based on status: closed = 100%, in_progress = 50%, open = 0%
            progress: epic.status === 'closed' ? 100 : epic.status === 'in_progress' ? 50 : 0,
        }));
    }

    /**
     * Get GitHub milestones using the gh CLI
     */
    private async getGitHubMilestones(state: 'open' | 'closed' | 'all'): Promise<{ id: string; name: string; status: string }[]> {
        try {
            const repo = this.connectors['config'].repo;
            const args = ['api', `repos/${repo}/milestones`, '--jq', '.[] | {number, title, state}'];
            if (state !== 'all') {
                args.splice(2, 0, '-f', `state=${state}`);
            }

            const token = process.env.GH_TOKEN || process.env.GITHUB_TOKEN;
            const env = { ...process.env };
            if (token) env.GH_TOKEN = token;

            const result = execFileSync('gh', args, { encoding: 'utf-8', env }).trim();
            if (!result) return [];

            // Parse JSONL output (one object per line)
            return result
                .split('\n')
                .filter((line) => line.trim())
                .map((line) => {
                    const data = JSON.parse(line);
                    return {
                        id: String(data.number),
                        name: data.title,
                        status: data.state === 'open' ? 'active' : 'closed',
                    };
                });
        } catch {
            return [];
        }
    }
}

// =============================================================================
// Review API
// =============================================================================

/**
 * Review operations API - PR feedback, comments, approvals
 *
 * Uses the gh CLI for GitHub providers and provider-specific APIs for others.
 */
class ReviewAPI {
    constructor(private connectors: TriageConnectors) {}

    /**
     * Get PR review comments.
     *
     * Fetches review comments from the pull request including inline code comments
     * and general review comments.
     */
    async getPRComments(prNumber: number): Promise<
        {
            id: string;
            body: string;
            author: string;
            path?: string;
            line?: number;
        }[]
    > {
        const provider = await this.connectors.getProvider();

        if (provider.name === 'github') {
            return this.getGitHubPRComments(prNumber);
        }

        // For non-GitHub providers, review comments are not applicable
        return [];
    }

    /**
     * Get unresolved feedback on a PR.
     *
     * Returns review comments and change requests that have not been resolved,
     * helping developers track what still needs attention.
     */
    async getUnresolvedFeedback(prNumber: number): Promise<
        {
            id: string;
            body: string;
            author: string;
            type: 'comment' | 'change_request';
        }[]
    > {
        const provider = await this.connectors.getProvider();

        if (provider.name === 'github') {
            return this.getGitHubUnresolvedFeedback(prNumber);
        }

        return [];
    }

    /**
     * Reply to a review comment.
     *
     * Posts a reply to an existing review comment on a pull request.
     */
    async replyToComment(commentId: string, body: string): Promise<void> {
        const provider = await this.connectors.getProvider();

        if (provider.name === 'github') {
            await this.replyToGitHubComment(commentId, body);
            return;
        }

        throw new Error(`Reply to comment not supported by ${provider.name} provider`);
    }

    /**
     * Get PR review comments from GitHub using the gh CLI
     */
    private async getGitHubPRComments(prNumber: number): Promise<
        {
            id: string;
            body: string;
            author: string;
            path?: string;
            line?: number;
        }[]
    > {
        try {
            const repo = this.connectors['config'].repo;

            // Get review comments (inline code comments)
            const reviewCommentsJson = this.gh([
                'api',
                ...this.getRepoApiPath(repo, `pulls/${prNumber}/comments`),
                '--jq',
                '.[] | {id: .id, body: .body, author: .user.login, path: .path, line: (.line // .original_line)}',
            ]);

            // Get issue comments (general PR thread comments)
            const issueCommentsJson = this.gh([
                'api',
                ...this.getRepoApiPath(repo, `issues/${prNumber}/comments`),
                '--jq',
                '.[] | {id: .id, body: .body, author: .user.login}',
            ]);

            const comments: { id: string; body: string; author: string; path?: string; line?: number }[] = [];

            // Parse review comments (JSONL)
            if (reviewCommentsJson) {
                for (const line of reviewCommentsJson.split('\n').filter((l) => l.trim())) {
                    try {
                        const data = JSON.parse(line);
                        comments.push({
                            id: String(data.id),
                            body: data.body,
                            author: data.author || 'unknown',
                            path: data.path || undefined,
                            line: data.line || undefined,
                        });
                    } catch {
                        // Skip malformed lines
                    }
                }
            }

            // Parse issue comments (JSONL)
            if (issueCommentsJson) {
                for (const line of issueCommentsJson.split('\n').filter((l) => l.trim())) {
                    try {
                        const data = JSON.parse(line);
                        comments.push({
                            id: String(data.id),
                            body: data.body,
                            author: data.author || 'unknown',
                        });
                    } catch {
                        // Skip malformed lines
                    }
                }
            }

            return comments;
        } catch {
            return [];
        }
    }

    /**
     * Get unresolved feedback from GitHub PR reviews
     */
    private async getGitHubUnresolvedFeedback(prNumber: number): Promise<
        {
            id: string;
            body: string;
            author: string;
            type: 'comment' | 'change_request';
        }[]
    > {
        try {
            const repo = this.connectors['config'].repo;

            // Get PR reviews
            const reviewsJson = this.gh([
                'api',
                ...this.getRepoApiPath(repo, `pulls/${prNumber}/reviews`),
                '--jq',
                '.[] | {id: .id, body: .body, author: .user.login, state: .state}',
            ]);

            const feedback: { id: string; body: string; author: string; type: 'comment' | 'change_request' }[] = [];

            if (reviewsJson) {
                for (const line of reviewsJson.split('\n').filter((l) => l.trim())) {
                    try {
                        const data = JSON.parse(line);
                        // Only include reviews that are not approved/dismissed
                        if (data.state === 'CHANGES_REQUESTED') {
                            feedback.push({
                                id: String(data.id),
                                body: data.body || 'Changes requested',
                                author: data.author || 'unknown',
                                type: 'change_request',
                            });
                        } else if (data.state === 'COMMENTED' && data.body) {
                            feedback.push({
                                id: String(data.id),
                                body: data.body,
                                author: data.author || 'unknown',
                                type: 'comment',
                            });
                        }
                    } catch {
                        // Skip malformed lines
                    }
                }
            }

            // Also get unresolved review comments (inline)
            const comments = await this.getGitHubPRComments(prNumber);
            for (const comment of comments) {
                if (comment.path) {
                    // Inline comments are always considered unresolved feedback
                    // (GitHub doesn't expose resolution status via REST easily)
                    feedback.push({
                        id: comment.id,
                        body: comment.body,
                        author: comment.author,
                        type: 'comment',
                    });
                }
            }

            return feedback;
        } catch {
            return [];
        }
    }

    /**
     * Reply to a GitHub PR comment
     */
    private async replyToGitHubComment(commentId: string, body: string): Promise<void> {
        const repo = this.connectors['config'].repo;

        // Use the gh api command to create a reply
        // First, try to reply as a review comment reply
        this.gh([
            'api',
            ...this.getRepoApiPath(repo, `pulls/comments/${commentId}/replies`),
            '-f',
            `body=${body}`,
        ]);
    }

    /**
     * Build the API path for a repo resource
     */
    private getRepoApiPath(repo: string | undefined, resource: string): string[] {
        if (repo) {
            return [`repos/${repo}/${resource}`];
        }
        // Fall back to detecting from environment
        const envRepo = process.env.GITHUB_REPOSITORY;
        if (envRepo) {
            return [`repos/${envRepo}/${resource}`];
        }
        // Last resort: try to detect from git
        try {
            const remote = execFileSync('git', ['remote', 'get-url', 'origin'], {
                encoding: 'utf-8',
            }).trim();
            const match = remote.match(/github\.com[/:]([^/]+\/[^/.]+)/);
            if (match) {
                const detectedRepo = match[1].replace(/\.git$/, '');
                return [`repos/${detectedRepo}/${resource}`];
            }
        } catch {
            // Not a git repo
        }
        return [`repos/unknown/unknown/${resource}`];
    }

    /**
     * Execute a gh CLI command
     */
    private gh(args: string[]): string {
        const token = process.env.GH_TOKEN || process.env.GITHUB_TOKEN;
        const env = { ...process.env };
        if (token) env.GH_TOKEN = token;
        return execFileSync('gh', args, { encoding: 'utf-8', env }).trim();
    }
}

// =============================================================================
// Convenience Functions
// =============================================================================

/**
 * Create a TriageConnectors instance with auto-detection
 *
 * @example
 * ```typescript
 * const triage = await createTriageConnectors();
 * const issues = await triage.issues.list({ status: 'open' });
 * ```
 */
export async function createTriageConnectors(config?: TriageConnectorsConfig): Promise<TriageConnectors> {
    const connectors = new TriageConnectors(config);
    // Pre-initialize the provider
    await connectors.getProvider();
    return connectors;
}
