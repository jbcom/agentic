---
title: MCP Server Integration
description: Use @agentic/triage as an MCP server for Claude Desktop and Cursor
---

# MCP Server Integration

`@agentic/triage` includes a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server, enabling integration with Claude Desktop, Cursor, and other MCP-compatible clients.

## What is MCP?

The Model Context Protocol is an open standard for connecting AI models to external tools and data sources. With MCP, you can give Claude Desktop or Cursor access to triage tools without writing any code.

## Quick Start

### For Claude Desktop

Add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server"]
    }
  }
}
```

### For Cursor

Add to your Cursor MCP configuration:

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server"]
    }
  }
}
```

### With Environment Variables

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server"],
      "env": {
        "GH_TOKEN": "your-github-token",
        "GITHUB_REPOSITORY": "owner/repo"
      }
    }
  }
}
```

## Available Tools

Once configured, these tools are available to the AI:

### Issue Management

| Tool | Description |
|------|-------------|
| `list_issues` | List issues with filtering |
| `get_issue` | Get a specific issue |
| `create_issue` | Create a new issue |
| `update_issue` | Update an existing issue |
| `close_issue` | Close an issue |
| `search_issues` | Search issues by query |
| `add_labels` | Add labels to an issue |
| `remove_labels` | Remove labels from an issue |

### Code Review

| Tool | Description |
|------|-------------|
| `get_pr_comments` | Get comments on a PR |
| `add_pr_comment` | Add a comment to a PR |
| `approve_pr` | Approve a pull request |
| `request_changes` | Request changes on a PR |

### Project Management

| Tool | Description |
|------|-------------|
| `get_sprints` | Get all sprints |
| `get_current_sprint` | Get the current sprint |
| `get_sprint_issues` | Get issues in a sprint |
| `move_to_sprint` | Move issue to a sprint |

## Usage Examples

### In Claude Desktop

After configuring the MCP server, you can ask Claude:

> "List all open bugs in my repository and prioritize them by severity"

Claude will use the `list_issues` and `search_issues` tools to fetch issues and provide analysis.

> "Create an issue for the login bug we discussed, label it as 'critical' and assign it to @developer"

Claude will use `create_issue` and `add_labels` to create the issue.

> "Review PR #123 and add a comment about the authentication changes"

Claude will use `get_pr_comments` and `add_pr_comment` to review and comment.

### In Cursor

In Cursor's AI chat, you can:

> "Triage all open issues and create a sprint plan"

Cursor will use the triage tools to analyze issues and suggest a sprint plan.

## Configuration Options

### Using Local Installation

If you prefer to install globally:

```bash
npm install -g @jbcom/agentic
```

Then configure:

```json
{
  "mcpServers": {
    "triage": {
      "command": "agentic-triage",
      "args": ["mcp-server"]
    }
  }
}
```

### With Specific Provider

```json
{
  "mcpServers": {
    "triage": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server", "--provider", "github"],
      "env": {
        "GH_TOKEN": "your-token",
        "GITHUB_REPOSITORY": "owner/repo"
      }
    }
  }
}
```

### For Jira

```json
{
  "mcpServers": {
    "triage-jira": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server", "--provider", "jira"],
      "env": {
        "JIRA_HOST": "your-company.atlassian.net",
        "JIRA_EMAIL": "your-email@company.com",
        "JIRA_API_TOKEN": "your-api-token",
        "JIRA_PROJECT": "PROJ"
      }
    }
  }
}
```

### For Linear

```json
{
  "mcpServers": {
    "triage-linear": {
      "command": "npx",
      "args": ["@jbcom/agentic", "mcp-server", "--provider", "linear"],
      "env": {
        "LINEAR_API_KEY": "your-api-key",
        "LINEAR_TEAM_ID": "your-team-id"
      }
    }
  }
}
```

## Running the Server Manually

For development or testing:

```bash
# Run the MCP server
npx @jbcom/agentic mcp-server

# With debug logging
DEBUG=triage:* npx @jbcom/agentic mcp-server

# With specific provider
npx @jbcom/agentic mcp-server --provider github
```

## Debugging

### Check Server Status

```bash
# Test the server starts correctly
npx @jbcom/agentic mcp-server --test

# List available tools
npx @jbcom/agentic mcp-server --list-tools
```

### View Logs

In Claude Desktop, check the logs at:
- macOS: `~/Library/Logs/Claude/`
- Windows: `%APPDATA%\Claude\logs\`

### Common Issues

**Server not starting:**
```bash
# Check Node.js version (requires 18+)
node --version

# Reinstall the package
npm install -g @jbcom/agentic
```

**Authentication errors:**
```bash
# Verify your token works
GH_TOKEN=your-token gh api user
```

**Repository not found:**
```bash
# Ensure GITHUB_REPOSITORY is set correctly
export GITHUB_REPOSITORY="owner/repo"
```

## Security Considerations

### Token Scopes

For GitHub, your token needs:
- `repo` - Full repository access
- `read:org` - Read organization data (optional)

### Best Practices

1. **Use fine-grained tokens** - Create tokens with minimal necessary permissions
2. **Rotate regularly** - Update tokens periodically
3. **Don't commit tokens** - Use environment variables or secure storage
4. **Review tool calls** - Claude will ask before executing destructive operations

## Next Steps

- [Vercel AI SDK Integration](/integrations/vercel-ai-sdk/) - Programmatic usage
- [@agentic/triage Package](/packages/triage/) - Full package reference
- [GitHub Actions Integration](/integrations/github-actions/) - CI/CD integration
