import { describe, expect, it, beforeEach } from 'vitest';
import { TriageConnectors } from '../src/triage/connectors.js';

/**
 * Tests for TriageConnectors ProjectAPI and ReviewAPI implementations.
 *
 * These tests use the Beads provider (in-memory) since it requires no
 * external services and exercises the connector layer thoroughly.
 */
describe('TriageConnectors', () => {
    let connectors: TriageConnectors;

    beforeEach(() => {
        connectors = new TriageConnectors({
            provider: { type: 'beads' },
        });
    });

    describe('initialization', () => {
        it('should initialize with beads provider', async () => {
            const name = await connectors.getProviderName();
            expect(name).toBe('beads');
        });

        it('should report ready', async () => {
            const ready = await connectors.isReady();
            expect(ready).toBe(true);
        });
    });

    describe('IssueAPI via connectors', () => {
        it('should create and retrieve an issue', async () => {
            const created = await connectors.issues.create({
                title: 'Test issue',
                type: 'bug',
                priority: 'high',
            });

            expect(created.title).toBe('Test issue');
            expect(created.type).toBe('bug');

            const retrieved = await connectors.issues.get(created.id);
            expect(retrieved).not.toBeNull();
            expect(retrieved!.title).toBe('Test issue');
        });

        it('should list issues', async () => {
            await connectors.issues.create({ title: 'Issue 1' });
            await connectors.issues.create({ title: 'Issue 2' });

            const list = await connectors.issues.list();
            expect(list).toHaveLength(2);
        });

        it('should search issues', async () => {
            await connectors.issues.create({ title: 'Login bug' });
            await connectors.issues.create({ title: 'Dashboard feature' });

            const results = await connectors.issues.search('login');
            expect(results).toHaveLength(1);
            expect(results[0].title).toBe('Login bug');
        });

        it('should close and reopen issues', async () => {
            const issue = await connectors.issues.create({ title: 'Close me' });
            const closed = await connectors.issues.close(issue.id, 'Done');
            expect(closed.status).toBe('closed');

            const reopened = await connectors.issues.reopen(issue.id);
            expect(reopened.status).toBe('open');
        });

        it('should get ready work', async () => {
            await connectors.issues.create({ title: 'Ready', priority: 'high' });
            await connectors.issues.create({ title: 'Also ready', priority: 'low' });

            const ready = await connectors.issues.getReadyWork();
            expect(ready).toHaveLength(2);
            // Should be sorted by priority (high first)
            expect(ready[0].issue.priority).toBe('high');
        });

        it('should get stats', async () => {
            await connectors.issues.create({ title: 'Bug', type: 'bug' });
            await connectors.issues.create({ title: 'Feature', type: 'feature' });

            const stats = await connectors.issues.getStats();
            expect(stats.total).toBe(2);
            expect(stats.open).toBe(2);
            expect(stats.byType.bug).toBe(1);
            expect(stats.byType.feature).toBe(1);
        });

        it('should add and remove labels', async () => {
            const issue = await connectors.issues.create({ title: 'Labels test' });
            await connectors.issues.addLabels(issue.id, ['bug', 'urgent']);

            let updated = await connectors.issues.get(issue.id);
            expect(updated!.labels).toContain('bug');
            expect(updated!.labels).toContain('urgent');

            await connectors.issues.removeLabels(issue.id, ['urgent']);
            updated = await connectors.issues.get(issue.id);
            expect(updated!.labels).toEqual(['bug']);
        });

        it('should get blocked issues', async () => {
            await connectors.issues.create({ title: 'Open' });
            const toBlock = await connectors.issues.create({ title: 'Block me' });
            await connectors.issues.update(toBlock.id, { status: 'blocked' });

            const blocked = await connectors.issues.getBlocked();
            expect(blocked).toHaveLength(1);
            expect(blocked[0].title).toBe('Block me');
        });

        it('should delete an issue via beads provider', async () => {
            const issue = await connectors.issues.create({ title: 'Delete me' });
            await connectors.issues.delete(issue.id);

            const found = await connectors.issues.get(issue.id);
            expect(found).toBeNull();
        });
    });

    describe('ProjectAPI', () => {
        it('should return empty sprints for beads provider', async () => {
            const sprints = await connectors.projects.getSprints();
            expect(sprints).toEqual([]);
        });

        it('should return null current sprint for beads provider', async () => {
            const sprint = await connectors.projects.getCurrentSprint();
            expect(sprint).toBeNull();
        });

        it('should return empty epics when no epic issues exist', async () => {
            await connectors.issues.create({ title: 'Normal issue', type: 'task' });
            const epics = await connectors.projects.getEpics();
            expect(epics).toEqual([]);
        });

        it('should return epics from epic-type issues', async () => {
            await connectors.issues.create({
                title: 'Auth Epic',
                type: 'epic',
                priority: 'high',
            });
            await connectors.issues.create({
                title: 'Normal task',
                type: 'task',
            });

            const epics = await connectors.projects.getEpics();
            expect(epics).toHaveLength(1);
            expect(epics[0].title).toBe('Auth Epic');
            // Open epic should have 0% progress
            expect(epics[0].progress).toBe(0);
        });

        it('should compute epic progress based on status', async () => {
            const epic = await connectors.issues.create({
                title: 'Closed Epic',
                type: 'epic',
            });
            await connectors.issues.close(epic.id);

            const epics = await connectors.projects.getEpics();
            expect(epics).toHaveLength(1);
            expect(epics[0].progress).toBe(100);
        });

        it('should compute in-progress epic progress', async () => {
            const epic = await connectors.issues.create({
                title: 'In Progress Epic',
                type: 'epic',
            });
            await connectors.issues.update(epic.id, { status: 'in_progress' });

            const epics = await connectors.projects.getEpics();
            expect(epics).toHaveLength(1);
            expect(epics[0].progress).toBe(50);
        });
    });

    describe('ReviewAPI', () => {
        it('should return empty comments for non-github provider', async () => {
            const comments = await connectors.reviews.getPRComments(1);
            expect(comments).toEqual([]);
        });

        it('should return empty feedback for non-github provider', async () => {
            const feedback = await connectors.reviews.getUnresolvedFeedback(1);
            expect(feedback).toEqual([]);
        });

        it('should throw when replying on non-github provider', async () => {
            await expect(
                connectors.reviews.replyToComment('123', 'response')
            ).rejects.toThrow('Reply to comment not supported by beads provider');
        });
    });

    describe('reconfigure', () => {
        it('should reset and reinitialize with new config', async () => {
            // Create an issue with original provider
            await connectors.issues.create({ title: 'Original' });
            let issues = await connectors.issues.list();
            expect(issues).toHaveLength(1);

            // Reconfigure with fresh beads provider
            await connectors.reconfigure({ provider: { type: 'beads' } });

            // New provider should be empty
            issues = await connectors.issues.list();
            expect(issues).toHaveLength(0);
        });
    });
});
