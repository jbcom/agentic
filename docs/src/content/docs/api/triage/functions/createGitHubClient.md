---
editUrl: false
next: false
prev: false
title: "createGitHubClient"
---

> **createGitHubClient**(): `Promise`\<`MCPClient`\>

Defined in: [packages/triage/src/mcp.ts:270](https://github.com/jbcom/agentic/blob/3e0fc0dae3f16f8cce14665d77f819f138b021fa/packages/triage/src/mcp.ts#L270)

Create GitHub MCP client for issue/PR/project operations

Provides access to:
- create_issue, update_issue, get_issue
- create_pull_request, merge_pull_request
- add_label, remove_label
- get_file_contents, create_or_update_file
- search_issues, search_repositories

## Returns

`Promise`\<`MCPClient`\>
