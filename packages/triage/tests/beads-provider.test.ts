import { describe, expect, it, beforeEach } from 'vitest';
import { BeadsProvider } from '../src/providers/beads.js';

describe('BeadsProvider', () => {
    let provider: BeadsProvider;

    beforeEach(() => {
        provider = new BeadsProvider();
    });

    describe('isReady', () => {
        it('should return true', async () => {
            expect(await provider.isReady()).toBe(true);
        });
    });

    describe('createIssue', () => {
        it('should create an issue with defaults', async () => {
            const issue = await provider.createIssue({
                title: 'Test issue',
            });

            expect(issue.id).toMatch(/^bd-[0-9a-f]{6}$/);
            expect(issue.title).toBe('Test issue');
            expect(issue.status).toBe('open');
            expect(issue.priority).toBe('medium');
            expect(issue.type).toBe('task');
            expect(issue.labels).toEqual([]);
            expect(issue.createdAt).toBeTruthy();
            expect(issue.updatedAt).toBeTruthy();
        });

        it('should create an issue with all options', async () => {
            const issue = await provider.createIssue({
                title: 'Critical bug',
                description: 'Something is broken',
                type: 'bug',
                priority: 'critical',
                labels: ['urgent', 'production'],
                assignee: 'developer1',
            });

            expect(issue.title).toBe('Critical bug');
            expect(issue.description).toBe('Something is broken');
            expect(issue.type).toBe('bug');
            expect(issue.priority).toBe('critical');
            expect(issue.labels).toEqual(['urgent', 'production']);
            expect(issue.assignee).toBe('developer1');
        });

        it('should generate unique IDs', async () => {
            const issue1 = await provider.createIssue({ title: 'Issue 1' });
            const issue2 = await provider.createIssue({ title: 'Issue 2' });
            expect(issue1.id).not.toBe(issue2.id);
        });
    });

    describe('getIssue', () => {
        it('should return an existing issue', async () => {
            const created = await provider.createIssue({ title: 'Find me' });
            const found = await provider.getIssue(created.id);

            expect(found).not.toBeNull();
            expect(found!.title).toBe('Find me');
            expect(found!.id).toBe(created.id);
        });

        it('should return null for non-existent issue', async () => {
            const found = await provider.getIssue('bd-nonexistent');
            expect(found).toBeNull();
        });

        it('should return a copy (not a reference)', async () => {
            const created = await provider.createIssue({ title: 'Original' });
            const found = await provider.getIssue(created.id);
            found!.title = 'Modified';

            const refetch = await provider.getIssue(created.id);
            expect(refetch!.title).toBe('Original');
        });
    });

    describe('updateIssue', () => {
        it('should update the title', async () => {
            const created = await provider.createIssue({ title: 'Old title' });
            const updated = await provider.updateIssue(created.id, { title: 'New title' });

            expect(updated.title).toBe('New title');
            expect(updated.id).toBe(created.id);
        });

        it('should update multiple fields', async () => {
            const created = await provider.createIssue({ title: 'Test' });
            const updated = await provider.updateIssue(created.id, {
                title: 'Updated',
                status: 'in_progress',
                priority: 'high',
                assignee: 'dev1',
            });

            expect(updated.title).toBe('Updated');
            expect(updated.status).toBe('in_progress');
            expect(updated.priority).toBe('high');
            expect(updated.assignee).toBe('dev1');
        });

        it('should set closedAt when status becomes closed', async () => {
            const created = await provider.createIssue({ title: 'Test' });
            const updated = await provider.updateIssue(created.id, { status: 'closed' });

            expect(updated.status).toBe('closed');
            expect(updated.closedAt).toBeTruthy();
        });

        it('should throw for non-existent issue', async () => {
            await expect(
                provider.updateIssue('bd-nonexistent', { title: 'Nope' })
            ).rejects.toThrow('Issue bd-nonexistent not found');
        });

        it('should update the updatedAt timestamp', async () => {
            const created = await provider.createIssue({ title: 'Test' });
            // Add a small delay to ensure timestamp difference
            await new Promise((r) => setTimeout(r, 10));
            const updated = await provider.updateIssue(created.id, { title: 'Updated' });

            expect(new Date(updated.updatedAt).getTime()).toBeGreaterThanOrEqual(
                new Date(created.updatedAt).getTime()
            );
        });
    });

    describe('closeIssue', () => {
        it('should close an issue', async () => {
            const created = await provider.createIssue({ title: 'Close me' });
            const closed = await provider.closeIssue(created.id, 'Fixed');

            expect(closed.status).toBe('closed');
            expect(closed.closedAt).toBeTruthy();
        });

        it('should throw for non-existent issue', async () => {
            await expect(provider.closeIssue('bd-nonexistent')).rejects.toThrow();
        });
    });

    describe('reopenIssue', () => {
        it('should reopen a closed issue', async () => {
            const created = await provider.createIssue({ title: 'Reopen me' });
            await provider.closeIssue(created.id);
            const reopened = await provider.reopenIssue(created.id, 'Not actually fixed');

            expect(reopened.status).toBe('open');
            expect(reopened.closedAt).toBeUndefined();
        });

        it('should throw for non-existent issue', async () => {
            await expect(provider.reopenIssue('bd-nonexistent')).rejects.toThrow();
        });
    });

    describe('deleteIssue', () => {
        it('should delete an issue', async () => {
            const created = await provider.createIssue({ title: 'Delete me' });
            await provider.deleteIssue(created.id);
            const found = await provider.getIssue(created.id);
            expect(found).toBeNull();
        });

        it('should throw for non-existent issue', async () => {
            await expect(provider.deleteIssue('bd-nonexistent')).rejects.toThrow();
        });
    });

    describe('listIssues', () => {
        beforeEach(async () => {
            await provider.createIssue({ title: 'Bug 1', type: 'bug', priority: 'high', labels: ['production'] });
            await provider.createIssue({ title: 'Feature 1', type: 'feature', priority: 'medium' });
            await provider.createIssue({ title: 'Task 1', type: 'task', priority: 'low', labels: ['tech-debt'] });
        });

        it('should list all issues', async () => {
            const issues = await provider.listIssues();
            expect(issues).toHaveLength(3);
        });

        it('should filter by status', async () => {
            const created = await provider.createIssue({ title: 'Closed one' });
            await provider.closeIssue(created.id);

            const open = await provider.listIssues({ status: 'open' });
            expect(open).toHaveLength(3);

            const closed = await provider.listIssues({ status: 'closed' });
            expect(closed).toHaveLength(1);
        });

        it('should filter by priority', async () => {
            const high = await provider.listIssues({ priority: 'high' });
            expect(high).toHaveLength(1);
            expect(high[0].title).toBe('Bug 1');
        });

        it('should filter by type', async () => {
            const bugs = await provider.listIssues({ type: 'bug' });
            expect(bugs).toHaveLength(1);
            expect(bugs[0].title).toBe('Bug 1');
        });

        it('should filter by labels (AND logic)', async () => {
            const production = await provider.listIssues({ labels: ['production'] });
            expect(production).toHaveLength(1);
            expect(production[0].title).toBe('Bug 1');
        });

        it('should filter by labelsAny (OR logic)', async () => {
            const either = await provider.listIssues({ labelsAny: ['production', 'tech-debt'] });
            expect(either).toHaveLength(2);
        });

        it('should filter by assignee', async () => {
            await provider.createIssue({ title: 'Assigned', assignee: 'dev1' });
            const assigned = await provider.listIssues({ assignee: 'dev1' });
            expect(assigned).toHaveLength(1);
            expect(assigned[0].title).toBe('Assigned');
        });

        it('should filter by titleContains', async () => {
            const bugs = await provider.listIssues({ titleContains: 'Bug' });
            expect(bugs).toHaveLength(1);
        });

        it('should limit results', async () => {
            const limited = await provider.listIssues({ limit: 2 });
            expect(limited).toHaveLength(2);
        });

        it('should sort by priority', async () => {
            const sorted = await provider.listIssues({ sortBy: 'priority' });
            expect(sorted[0].priority).toBe('high');
            expect(sorted[sorted.length - 1].priority).toBe('low');
        });
    });

    describe('searchIssues', () => {
        it('should search by title', async () => {
            await provider.createIssue({ title: 'Fix login bug' });
            await provider.createIssue({ title: 'Add feature' });

            const results = await provider.searchIssues('login');
            expect(results).toHaveLength(1);
            expect(results[0].title).toBe('Fix login bug');
        });

        it('should search by description', async () => {
            await provider.createIssue({
                title: 'Issue 1',
                description: 'The authentication flow is broken',
            });
            await provider.createIssue({ title: 'Issue 2', description: 'UI improvements' });

            const results = await provider.searchIssues('authentication');
            expect(results).toHaveLength(1);
        });

        it('should be case-insensitive', async () => {
            await provider.createIssue({ title: 'UPPERCASE BUG' });
            const results = await provider.searchIssues('uppercase');
            expect(results).toHaveLength(1);
        });
    });

    describe('getReadyWork', () => {
        it('should return open non-blocked issues sorted by priority', async () => {
            await provider.createIssue({ title: 'Low', priority: 'low' });
            await provider.createIssue({ title: 'High', priority: 'high' });
            await provider.createIssue({ title: 'Critical', priority: 'critical' });

            const ready = await provider.getReadyWork();
            expect(ready).toHaveLength(3);
            expect(ready[0].issue.title).toBe('Critical');
            expect(ready[1].issue.title).toBe('High');
            expect(ready[2].issue.title).toBe('Low');
        });

        it('should exclude blocked issues', async () => {
            await provider.createIssue({ title: 'Ready', priority: 'high' });
            const blocked = await provider.createIssue({ title: 'Blocked', priority: 'critical' });
            await provider.updateIssue(blocked.id, { status: 'blocked' });

            const ready = await provider.getReadyWork();
            expect(ready).toHaveLength(1);
            expect(ready[0].issue.title).toBe('Ready');
        });

        it('should exclude issues with blocked label', async () => {
            await provider.createIssue({ title: 'Ready' });
            await provider.createIssue({ title: 'Label Blocked', labels: ['blocked'] });

            const ready = await provider.getReadyWork();
            expect(ready).toHaveLength(1);
            expect(ready[0].issue.title).toBe('Ready');
        });

        it('should respect limit', async () => {
            await provider.createIssue({ title: 'Issue 1' });
            await provider.createIssue({ title: 'Issue 2' });
            await provider.createIssue({ title: 'Issue 3' });

            const ready = await provider.getReadyWork({ limit: 2 });
            expect(ready).toHaveLength(2);
        });
    });

    describe('getBlockedIssues', () => {
        it('should return issues with blocked status', async () => {
            await provider.createIssue({ title: 'Open' });
            const blocked = await provider.createIssue({ title: 'Blocked' });
            await provider.updateIssue(blocked.id, { status: 'blocked' });

            const blockedIssues = await provider.getBlockedIssues();
            expect(blockedIssues).toHaveLength(1);
            expect(blockedIssues[0].title).toBe('Blocked');
        });

        it('should return issues with blocked label', async () => {
            await provider.createIssue({ title: 'Open' });
            await provider.createIssue({ title: 'Blocked by label', labels: ['blocked'] });

            const blockedIssues = await provider.getBlockedIssues();
            expect(blockedIssues).toHaveLength(1);
            expect(blockedIssues[0].title).toBe('Blocked by label');
        });
    });

    describe('addLabels', () => {
        it('should add labels to an issue', async () => {
            const created = await provider.createIssue({ title: 'Test' });
            await provider.addLabels(created.id, ['bug', 'urgent']);

            const issue = await provider.getIssue(created.id);
            expect(issue!.labels).toContain('bug');
            expect(issue!.labels).toContain('urgent');
        });

        it('should not duplicate existing labels', async () => {
            const created = await provider.createIssue({ title: 'Test', labels: ['bug'] });
            await provider.addLabels(created.id, ['bug', 'urgent']);

            const issue = await provider.getIssue(created.id);
            expect(issue!.labels.filter((l) => l === 'bug')).toHaveLength(1);
        });

        it('should throw for non-existent issue', async () => {
            await expect(provider.addLabels('bd-nonexistent', ['bug'])).rejects.toThrow();
        });
    });

    describe('removeLabels', () => {
        it('should remove labels from an issue', async () => {
            const created = await provider.createIssue({
                title: 'Test',
                labels: ['bug', 'urgent', 'production'],
            });
            await provider.removeLabels(created.id, ['urgent']);

            const issue = await provider.getIssue(created.id);
            expect(issue!.labels).toEqual(['bug', 'production']);
        });

        it('should throw for non-existent issue', async () => {
            await expect(provider.removeLabels('bd-nonexistent', ['bug'])).rejects.toThrow();
        });
    });

    describe('getStats', () => {
        it('should return correct statistics', async () => {
            await provider.createIssue({ title: 'Bug', type: 'bug', priority: 'high' });
            await provider.createIssue({ title: 'Feature', type: 'feature', priority: 'medium' });
            const toClose = await provider.createIssue({ title: 'Done', type: 'task', priority: 'low' });
            await provider.closeIssue(toClose.id);

            const stats = await provider.getStats();
            expect(stats.total).toBe(3);
            expect(stats.open).toBe(2);
            expect(stats.closed).toBe(1);
            expect(stats.byPriority.high).toBe(1);
            expect(stats.byPriority.medium).toBe(1);
            expect(stats.byPriority.low).toBe(1);
            expect(stats.byType.bug).toBe(1);
            expect(stats.byType.feature).toBe(1);
            expect(stats.byType.task).toBe(1);
        });

        it('should return zero stats for empty provider', async () => {
            const stats = await provider.getStats();
            expect(stats.total).toBe(0);
            expect(stats.open).toBe(0);
        });
    });

    describe('getAvailableLabels', () => {
        it('should return unique sorted labels', async () => {
            await provider.createIssue({ title: 'A', labels: ['bug', 'urgent'] });
            await provider.createIssue({ title: 'B', labels: ['bug', 'feature'] });

            const labels = await provider.getAvailableLabels();
            expect(labels).toEqual(['bug', 'feature', 'urgent']);
        });

        it('should return empty array when no issues', async () => {
            const labels = await provider.getAvailableLabels();
            expect(labels).toEqual([]);
        });
    });
});
