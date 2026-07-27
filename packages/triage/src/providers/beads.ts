/**
 * Beads Issue Tracker Provider
 *
 * A local-first, AI-native issue tracker that stores issues in memory.
 * Designed for standalone local workflows where a full issue tracker
 * is not needed. Issues persist for the lifetime of the process.
 *
 * Beads issues use a "bd-" prefix for IDs (e.g., "bd-a1b2c3").
 */

import {
    type BeadsProviderConfig,
    type CreateIssueOptions,
    type IssuePriority,
    type ListIssuesOptions,
    type ProviderStats,
    priorityToNumber,
    type ReadyWork,
    type TriageIssue,
    type TriageProvider,
    type UpdateIssueOptions,
} from './types.js';

/** Generate a short random hex ID */
function generateBeadId(): string {
    const hex = Math.random().toString(16).substring(2, 8);
    return `bd-${hex}`;
}

function filterByStatus(issues: TriageIssue[], status?: ListIssuesOptions['status']): TriageIssue[] {
    if (!status) {
        return issues;
    }

    const statuses = Array.isArray(status) ? status : [status];
    return issues.filter((issue) => statuses.includes(issue.status));
}

function filterByPriority(issues: TriageIssue[], priority?: ListIssuesOptions['priority']): TriageIssue[] {
    if (!priority) {
        return issues;
    }

    const priorities = Array.isArray(priority) ? priority : [priority];
    return issues.filter((issue) => priorities.includes(issue.priority));
}

function filterByType(issues: TriageIssue[], type?: ListIssuesOptions['type']): TriageIssue[] {
    if (!type) {
        return issues;
    }

    const types = Array.isArray(type) ? type : [type];
    return issues.filter((issue) => types.includes(issue.type));
}

function filterByLabels(issues: TriageIssue[], labels?: string[]): TriageIssue[] {
    if (!labels?.length) {
        return issues;
    }

    return issues.filter((issue) => labels.every((label) => issue.labels.includes(label)));
}

function filterByAnyLabels(issues: TriageIssue[], labelsAny?: string[]): TriageIssue[] {
    if (!labelsAny?.length) {
        return issues;
    }

    return issues.filter((issue) => labelsAny.some((label) => issue.labels.includes(label)));
}

function filterByAssignee(issues: TriageIssue[], assignee?: string): TriageIssue[] {
    if (!assignee) {
        return issues;
    }

    return issues.filter((issue) => issue.assignee === assignee);
}

function filterByTitle(issues: TriageIssue[], titleContains?: string): TriageIssue[] {
    if (!titleContains) {
        return issues;
    }

    const search = titleContains.toLowerCase();
    return issues.filter((issue) => issue.title.toLowerCase().includes(search));
}

function filterByDescription(issues: TriageIssue[], descriptionContains?: string): TriageIssue[] {
    if (!descriptionContains) {
        return issues;
    }

    const search = descriptionContains.toLowerCase();
    return issues.filter((issue) => issue.description?.toLowerCase().includes(search));
}

function filterByCreatedAfter(issues: TriageIssue[], createdAfter?: string): TriageIssue[] {
    if (!createdAfter) {
        return issues;
    }

    const after = new Date(createdAfter);
    return issues.filter((issue) => new Date(issue.createdAt) > after);
}

function filterByCreatedBefore(issues: TriageIssue[], createdBefore?: string): TriageIssue[] {
    if (!createdBefore) {
        return issues;
    }

    const before = new Date(createdBefore);
    return issues.filter((issue) => new Date(issue.createdAt) < before);
}

function sortIssues(issues: TriageIssue[], options?: ListIssuesOptions): TriageIssue[] {
    const sorted = [...issues];

    if (options?.sortBy === 'priority') {
        sorted.sort((a, b) => priorityToNumber(a.priority) - priorityToNumber(b.priority));
    } else if (options?.sortBy === 'updated') {
        sorted.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime());
    } else {
        sorted.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
    }

    return options?.sortOrder === 'asc' ? sorted.reverse() : sorted;
}

export class BeadsProvider implements TriageProvider {
    readonly name = 'beads';
    readonly displayName = 'Beads';

    /** In-memory issue store */
    private issues = new Map<string, TriageIssue>();

    constructor(public readonly config: BeadsProviderConfig = { type: 'beads' }) {}

    async isReady(): Promise<boolean> {
        return true;
    }

    async createIssue(options: CreateIssueOptions): Promise<TriageIssue> {
        const id = generateBeadId();
        const now = new Date().toISOString();

        const issue: TriageIssue = {
            id,
            title: options.title,
            description: options.description,
            status: 'open',
            priority: options.priority || 'medium',
            type: options.type || 'task',
            labels: options.labels || [],
            assignee: options.assignee,
            createdAt: now,
            updatedAt: now,
            metadata: options.metadata,
        };

        this.issues.set(id, issue);
        return { ...issue };
    }

    async getIssue(id: string): Promise<TriageIssue | null> {
        const issue = this.issues.get(id);
        return issue ? { ...issue } : null;
    }

    async updateIssue(id: string, options: UpdateIssueOptions): Promise<TriageIssue> {
        const issue = this.issues.get(id);
        if (!issue) {
            throw new Error(`Issue ${id} not found`);
        }

        const updated: TriageIssue = {
            ...issue,
            title: options.title ?? issue.title,
            description: options.description ?? issue.description,
            status: options.status ?? issue.status,
            priority: options.priority ?? issue.priority,
            type: options.type ?? issue.type,
            labels: options.labels ?? issue.labels,
            assignee: options.assignee ?? issue.assignee,
            updatedAt: new Date().toISOString(),
            closedAt: options.status === 'closed' ? new Date().toISOString() : issue.closedAt,
            metadata: { ...issue.metadata, ...options.metadata },
        };

        this.issues.set(id, updated);
        return { ...updated };
    }

    async closeIssue(id: string, reason?: string): Promise<TriageIssue> {
        const issue = this.issues.get(id);
        if (!issue) {
            throw new Error(`Issue ${id} not found`);
        }

        return this.updateIssue(id, {
            status: 'closed',
            metadata: reason ? { closeReason: reason } : undefined,
        });
    }

    async reopenIssue(id: string, reason?: string): Promise<TriageIssue> {
        const issue = this.issues.get(id);
        if (!issue) {
            throw new Error(`Issue ${id} not found`);
        }

        const updated: TriageIssue = {
            ...issue,
            status: 'open',
            closedAt: undefined,
            updatedAt: new Date().toISOString(),
            metadata: { ...issue.metadata, ...(reason ? { reopenReason: reason } : {}) },
        };

        this.issues.set(id, updated);
        return { ...updated };
    }

    async deleteIssue(id: string): Promise<void> {
        if (!this.issues.has(id)) {
            throw new Error(`Issue ${id} not found`);
        }
        this.issues.delete(id);
    }

    async listIssues(options?: ListIssuesOptions): Promise<TriageIssue[]> {
        let issues = Array.from(this.issues.values());

        issues = filterByStatus(issues, options?.status);
        issues = filterByPriority(issues, options?.priority);
        issues = filterByType(issues, options?.type);
        issues = filterByLabels(issues, options?.labels);
        issues = filterByAnyLabels(issues, options?.labelsAny);
        issues = filterByAssignee(issues, options?.assignee);
        issues = filterByTitle(issues, options?.titleContains);
        issues = filterByDescription(issues, options?.descriptionContains);
        issues = filterByCreatedAfter(issues, options?.createdAfter);
        issues = filterByCreatedBefore(issues, options?.createdBefore);
        issues = sortIssues(issues, options);

        if (options?.limit) {
            issues = issues.slice(0, options.limit);
        }

        return issues.map((i) => ({ ...i }));
    }

    async getReadyWork(options?: { limit?: number; priority?: IssuePriority }): Promise<ReadyWork[]> {
        const issues = await this.listIssues({
            status: 'open',
            limit: options?.limit || 20,
            priority: options?.priority,
        });

        // Filter out blocked issues
        const ready = issues
            .filter((i) => i.status !== 'blocked' && !i.labels.some((l) => l.toLowerCase().includes('blocked')))
            .sort((a, b) => priorityToNumber(a.priority) - priorityToNumber(b.priority));

        return ready.map((issue) => ({ issue }));
    }

    async getBlockedIssues(): Promise<TriageIssue[]> {
        const issues = await this.listIssues({ status: 'blocked' });
        const blockedByLabel = (await this.listIssues({ status: 'open' })).filter((i) =>
            i.labels.some((l) => l.toLowerCase().includes('blocked'))
        );

        // Merge and deduplicate
        const seen = new Set(issues.map((i) => i.id));
        for (const issue of blockedByLabel) {
            if (!seen.has(issue.id)) {
                issues.push(issue);
                seen.add(issue.id);
            }
        }

        return issues;
    }

    async searchIssues(query: string, options?: ListIssuesOptions): Promise<TriageIssue[]> {
        const all = await this.listIssues(options);
        const lower = query.toLowerCase();
        return all.filter(
            (i) => i.title.toLowerCase().includes(lower) || i.description?.toLowerCase().includes(lower)
        );
    }

    async addLabels(id: string, labels: string[]): Promise<void> {
        const issue = this.issues.get(id);
        if (!issue) {
            throw new Error(`Issue ${id} not found`);
        }

        const merged = Array.from(new Set([...issue.labels, ...labels]));
        issue.labels = merged;
        issue.updatedAt = new Date().toISOString();
    }

    async removeLabels(id: string, labels: string[]): Promise<void> {
        const issue = this.issues.get(id);
        if (!issue) {
            throw new Error(`Issue ${id} not found`);
        }

        issue.labels = issue.labels.filter((l) => !labels.includes(l));
        issue.updatedAt = new Date().toISOString();
    }

    async getStats(): Promise<ProviderStats> {
        const issues = Array.from(this.issues.values());
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
            switch (issue.status) {
                case 'open': stats.open++; break;
                case 'in_progress': stats.inProgress++; break;
                case 'blocked': stats.blocked++; break;
                case 'closed': stats.closed++; break;
            }
            stats.byPriority[issue.priority]++;
            stats.byType[issue.type]++;
        }

        return stats;
    }

    /**
     * Get all available labels across all issues
     */
    async getAvailableLabels(): Promise<string[]> {
        const labels = new Set<string>();
        for (const issue of this.issues.values()) {
            for (const label of issue.labels) {
                labels.add(label);
            }
        }
        return Array.from(labels).sort();
    }
}
