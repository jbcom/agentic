import {
    addLabelsTool,
    closeIssueTool,
    createIssueTool,
    getIssueTool,
    listIssuesTool,
    removeLabelsTool,
    searchIssuesTool,
    triageIssueTool,
    updateIssueTool,
} from './issue.js';
import { analyzePRTool } from './pr.js';
import { submitReviewTool } from './review.js';
import { sageTool } from './sage.js';
import { visualReviewTool } from './visual.js';

export * from './issue.js';
export * from './pr.js';
export * from './review.js';
export * from './sage.js';
export * from './visual.js';

export const triageTools = {
    listIssues: listIssuesTool,
    getIssue: getIssueTool,
    createIssue: createIssueTool,
    updateIssue: updateIssueTool,
    closeIssue: closeIssueTool,
    searchIssues: searchIssuesTool,
    addLabels: addLabelsTool,
    removeLabels: removeLabelsTool,
    triageIssue: triageIssueTool,
    submitReview: submitReviewTool,
    analyzePR: analyzePRTool,
    sage: sageTool,
    visualReview: visualReviewTool,
};

export function getTriageTools() {
    return triageTools;
}
