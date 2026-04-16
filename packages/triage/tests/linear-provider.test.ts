import { beforeEach, describe, expect, it, vi } from 'vitest';
import { LinearProvider } from '../src/providers/linear.js';

describe('LinearProvider', () => {
    let provider: LinearProvider;
    let mockClient: {
        issue: ReturnType<typeof vi.fn>;
        team: ReturnType<typeof vi.fn>;
        createIssueLabel: ReturnType<typeof vi.fn>;
        updateIssue: ReturnType<typeof vi.fn>;
    };

    beforeEach(() => {
        provider = new LinearProvider({
            apiKey: 'lin_api_test',
            teamId: '12345678-1234-1234-1234-123456789012',
        });

        const issueLabels = [
            { id: 'label-bug', name: 'bug' },
            { id: 'label-urgent', name: 'urgent' },
        ];

        const issue = {
            labels: vi.fn().mockResolvedValue({
                nodes: [issueLabels[0]],
            }),
        };

        mockClient = {
            issue: vi.fn().mockResolvedValue(issue),
            team: vi.fn().mockResolvedValue({
                labels: vi.fn().mockResolvedValue({
                    nodes: issueLabels,
                }),
            }),
            createIssueLabel: vi.fn().mockImplementation(async ({ name }: { name: string }) => ({
                issueLabel: Promise.resolve({
                    id: `label-${name.toLowerCase()}`,
                    name,
                }),
            })),
            updateIssue: vi.fn().mockResolvedValue({}),
        };

        (provider as unknown as { client: typeof mockClient }).client = mockClient;
    });

    it('adds existing and newly created labels by id', async () => {
        await provider.addLabels('issue-1', ['urgent', 'Docs']);

        expect(mockClient.createIssueLabel).toHaveBeenCalledWith({
            teamId: '12345678-1234-1234-1234-123456789012',
            name: 'Docs',
            color: '#6B7280',
        });
        expect(mockClient.updateIssue).toHaveBeenCalledWith('issue-1', {
            labelIds: ['label-bug', 'label-urgent', 'label-docs'],
        });
    });

    it('removes labels by normalized name', async () => {
        mockClient.issue.mockResolvedValue({
            labels: vi.fn().mockResolvedValue({
                nodes: [
                    { id: 'label-bug', name: 'bug' },
                    { id: 'label-urgent', name: 'urgent' },
                ],
            }),
        });

        await provider.removeLabels('issue-1', ['URGENT']);

        expect(mockClient.updateIssue).toHaveBeenCalledWith('issue-1', {
            labelIds: ['label-bug'],
        });
    });

    it('throws when the target issue does not exist', async () => {
        mockClient.issue.mockResolvedValue(null);

        await expect(provider.addLabels('missing', ['bug'])).rejects.toThrow('Issue missing not found');
    });
});
