import type { PRAnalysis } from '../schemas/pr.js';
import { addIssueLabels, commentOnPR, getPullRequest } from '../octokit.js';

function buildAnalysisLabels(analysis: PRAnalysis): string[] {
    return [
        `scope:${analysis.scope}`,
        `risk:${analysis.riskLevel}`,
        `tests:${analysis.testingCoverage}`,
        ...(analysis.breakingChanges.length > 0 ? ['breaking-change'] : []),
    ];
}

function formatAnalysisComment(prNumber: number, analysis: PRAnalysis): string {
    const lines = [
        `## Automated PR Analysis for #${prNumber}`,
        '',
        `**Suggested title:** ${analysis.title}`,
        '',
        `**Summary:** ${analysis.summary}`,
        '',
        `- Scope: ${analysis.scope}`,
        `- Risk: ${analysis.riskLevel}`,
        `- Testing coverage: ${analysis.testingCoverage}`,
    ];

    if (analysis.breakingChanges.length > 0) {
        lines.push('', '**Breaking changes:**');
        for (const change of analysis.breakingChanges) {
            lines.push(`- ${change}`);
        }
    }

    if (analysis.relatedIssues.length > 0) {
        lines.push('', '**Related issues:**');
        for (const issue of analysis.relatedIssues) {
            lines.push(`- ${issue}`);
        }
    }

    return lines.join('\n');
}

/**
 * Handler for analyzing a PR
 */
export async function handleAnalyzePR(prNumber: number, analysis: PRAnalysis) {
    try {
        const pullRequest = await getPullRequest(prNumber);
        const labels = buildAnalysisLabels(analysis);
        const comment = formatAnalysisComment(prNumber, analysis);

        await commentOnPR(prNumber, comment);
        await addIssueLabels(prNumber, labels);

        return {
            success: true,
            message: `Analysis for PR #${prNumber} completed`,
            pullRequest: {
                number: pullRequest.number,
                title: pullRequest.title,
                state: pullRequest.state,
            },
            labelsApplied: labels,
            commentPosted: true,
            analysis,
        };
    } catch (error) {
        return {
            success: false,
            message: `Failed to analyze PR #${prNumber}: ${error instanceof Error ? error.message : String(error)}`,
            error,
        };
    }
}
