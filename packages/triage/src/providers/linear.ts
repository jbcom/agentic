import { type Issue, type IssueLabel, LinearClient } from '@linear/sdk';
import {
    type CreateIssueOptions,
    type IssuePriority,
    type IssueStatus,
    type ListIssuesOptions,
    normalizeStatus,
    normalizeType,
    type ProviderStats,
    priorityToNumber,
    type ReadyWork,
    type TriageIssue,
    type TriageProvider,
    type UpdateIssueOptions,
} from './types.js';

export interface LinearConfig {
    apiKey: string;
    teamId: string;
}

export class LinearProvider implements TriageProvider {
    readonly name = 'linear';
    readonly displayName = 'Linear';

    private client: LinearClient;
    private teamId: string;

    constructor(config: LinearConfig) {
        if (!config.apiKey || !config.apiKey.startsWith('lin_api_')) {
            throw new Error('Invalid Linear API key. Must start with lin_api_');
        }
        if (!config.teamId || !/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{12,13}/i.test(config.teamId)) {
            // Linear IDs can be UUIDs or shorter strings depending on the object
            // but teamId is usually a UUID.
        }
        this.client = new LinearClient({ apiKey: config.apiKey });
        this.teamId = config.teamId;
    }

    async isReady(): Promise<boolean> {
        try {
            await this.client.team(this.teamId);
            return true;
        } catch {
            return false;
        }
    }

    private normalizeLabelName(label: string): string {
        return label.trim().toLowerCase();
    }

    private async getIssueOrThrow(id: string): Promise<Issue> {
        const issue = await this.client.issue(id);
        if (!issue) {
            throw new Error(`Issue ${id} not found`);
        }
        return issue;
    }

    private async getTeamLabels(): Promise<Map<string, IssueLabel>> {
        const team = await this.client.team(this.teamId);
        const labels = await team.labels();

        return new Map(
            labels.nodes.map((label) => [this.normalizeLabelName(label.name), label] as const)
        );
    }

    private async resolveLabelIds(labels: string[]): Promise<string[]> {
        const existingLabels = await this.getTeamLabels();
        const resolvedIds: string[] = [];

        for (const rawLabel of labels) {
            const trimmed = rawLabel.trim();
            if (!trimmed) {
                continue;
            }

            const normalized = this.normalizeLabelName(trimmed);
            let label = existingLabels.get(normalized);

            if (!label) {
                const created = await this.client.createIssueLabel({
                    teamId: this.teamId,
                    name: trimmed,
                    color: '#6B7280',
                });
                label = await created.issueLabel;
                if (!label) {
                    throw new Error(`Failed to create Linear label "${trimmed}"`);
                }
                existingLabels.set(normalized, label);
            }

            resolvedIds.push(label.id);
        }

        return Array.from(new Set(resolvedIds));
    }

    private async mapIssue(issue: Issue): Promise<TriageIssue> {
        const state = await issue.state;
        const labels = await issue.labels();
        const assignee = await issue.assignee;

        const labelNames = labels.nodes.map((l) => l.name);

        // Derive type from labels if not already clear
        const typeLabel = labelNames.find((l) => ['bug', 'feature', 'task', 'epic', 'chore'].includes(l.toLowerCase()));

        return {
            id: issue.id,
            title: issue.title,
            description: issue.description || undefined,
            status: normalizeStatus(state?.name || 'open'),
            priority: this.mapPriorityFromLinear(issue.priority),
            type: normalizeType(typeLabel || 'task'),
            labels: labelNames,
            assignee: assignee?.name || assignee?.displayName,
            createdAt: (issue.createdAt as any).toISOString?.() || String(issue.createdAt),
            updatedAt: (issue.updatedAt as any).toISOString?.() || String(issue.updatedAt),
            closedAt:
                (issue.completedAt as any)?.toISOString?.() ||
                (issue.completedAt ? String(issue.completedAt) : undefined),
            url: issue.url,
            metadata: { raw: issue },
        };
    }

    private mapPriorityFromLinear(priority: number): IssuePriority {
        switch (priority) {
            case 1:
                return 'critical';
            case 2:
                return 'high';
            case 3:
                return 'medium';
            case 4:
                return 'low';
            default:
                return 'backlog';
        }
    }

    private mapPriorityToLinear(priority?: IssuePriority): number {
        switch (priority) {
            case 'critical':
                return 1;
            case 'high':
                return 2;
            case 'medium':
                return 3;
            case 'low':
                return 4;
            case 'backlog':
                return 0; // In Linear, 0 is no priority/backlog
            default:
                return 0;
        }
    }

    private mapStatusToLinear(status?: IssueStatus): string | undefined {
        switch (status) {
            case 'open':
                return 'Todo';
            case 'in_progress':
                return 'In Progress';
            case 'blocked':
                return 'Blocked';
            case 'closed':
                return 'Done';
            default:
                return undefined;
        }
    }

    async listIssues(options?: ListIssuesOptions): Promise<TriageIssue[]> {
        const filter: any = {
            team: { id: { eq: this.teamId } },
        };

        if (options?.status) {
            const statuses = Array.isArray(options.status) ? options.status : [options.status];
            filter.state = { name: { in: statuses.map((s) => this.mapStatusToLinear(s)).filter(Boolean) } };
        }

        if (options?.priority) {
            const priorities = Array.isArray(options.priority) ? options.priority : [options.priority];
            filter.priority = { in: priorities.map((p) => this.mapPriorityToLinear(p)) };
        }

        if (options?.assignee) {
            filter.assignee = { name: { eq: options.assignee } };
        }

        const issues = await this.client.issues({
            filter,
            first: options?.limit || 50,
        });

        return Promise.all(issues.nodes.map((issue) => this.mapIssue(issue)));
    }

    async getIssue(id: string): Promise<TriageIssue | null> {
        try {
            const issue = await this.client.issue(id);
            if (!issue) return null;
            return this.mapIssue(issue);
        } catch {
            return null;
        }
    }

    async createIssue(options: CreateIssueOptions): Promise<TriageIssue> {
        const labelIds = options.labels?.length ? await this.resolveLabelIds(options.labels) : undefined;
        const response = await this.client.createIssue({
            teamId: this.teamId,
            title: options.title,
            description: options.description,
            priority: this.mapPriorityToLinear(options.priority),
            labelIds,
        });

        const newIssue = await response.issue;
        if (!newIssue) {
            throw new Error('Failed to create Linear issue');
        }

        return this.mapIssue(newIssue);
    }

    async updateIssue(id: string, options: UpdateIssueOptions): Promise<TriageIssue> {
        const existingIssue = options.labels?.length ? await this.getIssueOrThrow(id) : null;
        const existingIssueLabels = existingIssue ? await existingIssue.labels() : null;
        const mergedLabelIds =
            options.labels?.length && existingIssueLabels
                ? Array.from(
                      new Set([
                          ...existingIssueLabels.nodes.map((label) => label.id),
                          ...(await this.resolveLabelIds(options.labels)),
                      ])
                  )
                : undefined;

        const response = await this.client.updateIssue(id, {
            title: options.title,
            description: options.description,
            priority: options.priority ? this.mapPriorityToLinear(options.priority) : undefined,
            labelIds: mergedLabelIds,
        });

        const updatedIssue = await response.issue;
        if (!updatedIssue) {
            throw new Error(`Failed to update Linear issue ${id}`);
        }

        if (options.status) {
            const stateName = this.mapStatusToLinear(options.status);
            if (stateName) {
                // We'd need to find the state ID for the name in this team
                const team = await this.client.team(this.teamId);
                const states = await team.states();
                const state = states.nodes.find((s) => s.name === stateName);
                if (state) {
                    await this.client.updateIssue(id, { stateId: state.id });
                }
            }
        }

        return this.mapIssue(updatedIssue);
    }

    async closeIssue(id: string, reason?: string): Promise<TriageIssue> {
        if (reason) {
            await this.client.createComment({ issueId: id, body: reason });
        }
        return this.updateIssue(id, { status: 'closed' });
    }

    async reopenIssue(id: string, reason?: string): Promise<TriageIssue> {
        if (reason) {
            await this.client.createComment({ issueId: id, body: reason });
        }
        return this.updateIssue(id, { status: 'open' });
    }

    async searchIssues(query: string, options?: ListIssuesOptions): Promise<TriageIssue[]> {
        const issues = await this.client.issues({
            filter: {
                team: { id: { eq: this.teamId } },
                or: [{ title: { contains: query } }, { description: { contains: query } }],
            },
            first: options?.limit,
        });

        return Promise.all(issues.nodes.map((issue) => this.mapIssue(issue)));
    }

    async getReadyWork(options?: { limit?: number; priority?: IssuePriority }): Promise<ReadyWork[]> {
        const issues = await this.listIssues({
            status: 'open',
            limit: options?.limit || 20,
            priority: options?.priority,
        });

        // Linear doesn't have a direct "blocked" status in the same way,
        // but it has relationships. For now, simple priority sort.
        const sorted = issues.sort((a, b) => priorityToNumber(a.priority) - priorityToNumber(b.priority));
        return sorted.map((issue) => ({ issue }));
    }

    async getBlockedIssues(): Promise<TriageIssue[]> {
        return this.listIssues({ status: 'blocked' });
    }

    async addLabels(_id: string, _labels: string[]): Promise<void> {
        const issue = await this.getIssueOrThrow(_id);
        const currentLabels = await issue.labels();
        const labelIds = await this.resolveLabelIds(_labels);

        await this.client.updateIssue(_id, {
            labelIds: Array.from(new Set([...currentLabels.nodes.map((label) => label.id), ...labelIds])),
        });
    }

    async removeLabels(_id: string, _labels: string[]): Promise<void> {
        const issue = await this.getIssueOrThrow(_id);
        const currentLabels = await issue.labels();
        const labelsToRemove = new Set(_labels.map((label) => this.normalizeLabelName(label)).filter(Boolean));

        await this.client.updateIssue(_id, {
            labelIds: currentLabels.nodes
                .filter((label) => !labelsToRemove.has(this.normalizeLabelName(label.name)))
                .map((label) => label.id),
        });
    }

    async getStats(): Promise<ProviderStats> {
        const issues = await this.listIssues({ limit: 1000 });
        const stats: ProviderStats = {
            total: issues.length,
            open: 0,
            inProgress: 0,
            blocked: 0,
            closed: 0,
            byPriority: { critical: 0, high: 0, medium: 0, low: 0, backlog: 0 },
            byType: { bug: 0, feature: 0, task: 0, epic: 0, chore: 0, docs: 0 },
        };

        for (const issue of issues) {
            stats[issue.status === 'in_progress' ? 'inProgress' : issue.status]++;
            stats.byPriority[issue.priority]++;
            stats.byType[issue.type]++;
        }

        return stats;
    }
}
