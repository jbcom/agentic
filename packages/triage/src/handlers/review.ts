import type { CodeReview } from '../schemas/review.js';
import { addIssueLabels, commentOnPR, getPullRequest, submitPRReview } from '../octokit.js';

function mapReviewStatus(status: CodeReview['status']): 'APPROVE' | 'REQUEST_CHANGES' | 'COMMENT' {
    switch (status) {
        case 'approve':
            return 'APPROVE';
        case 'request_changes':
            return 'REQUEST_CHANGES';
        default:
            return 'COMMENT';
    }
}

function buildReviewBody(review: CodeReview): string {
    const lines = [
        review.summary,
        '',
        `Impact: ${review.impact}`,
    ];

    if (review.comments.length > 0) {
        lines.push('', 'Findings:');
        for (const comment of review.comments) {
            const location = comment.line ? ` (${comment.file}:${comment.line})` : ` (${comment.file})`;
            const severity = comment.severity ? ` [${comment.severity}]` : '';
            lines.push(`- ${comment.type}${severity}${location}: ${comment.content}`);
        }
    }

    return lines.join('\n');
}

/**
 * Handler for submitting a code review
 */
export async function handleSubmitReview(prNumber: number, review: CodeReview) {
    try {
        const pullRequest = await getPullRequest(prNumber);
        const body = buildReviewBody(review);
        const event = mapReviewStatus(review.status);

        await submitPRReview(prNumber, event, body);

        if (review.suggestedLabels.length > 0) {
            await addIssueLabels(prNumber, review.suggestedLabels);
        }

        if (review.comments.length > 0) {
            await commentOnPR(prNumber, body);
        }

        return {
            success: true,
            message: `Review for PR #${prNumber} submitted with status: ${review.status}`,
            pullRequest: {
                number: pullRequest.number,
                title: pullRequest.title,
                state: pullRequest.state,
            },
            labelsApplied: review.suggestedLabels,
            submittedEvent: event,
            review,
        };
    } catch (error) {
        return {
            success: false,
            message: `Failed to submit review for PR #${prNumber}: ${error instanceof Error ? error.message : String(error)}`,
            error,
        };
    }
}
