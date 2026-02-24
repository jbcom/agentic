GitHub Module
=============

The GitHub module provides integration with the GitHub API.

.. contents:: Contents
   :local:
   :depth: 2

Overview
--------

The GitHub module provides:

- Repository operations
- Issue and PR management
- Webhook handling
- GitHub Actions integration

GitHub Client
-------------

The ``GitHubClient`` class is the main interface for GitHub API operations.

**Key Methods:**

- ``getRepository(owner, repo)`` - Get repository information
- ``listIssues(owner, repo, options)`` - List issues with filtering
- ``getPullRequest(owner, repo, number)`` - Get PR details
- ``createComment(owner, repo, number, body)`` - Create a comment
- ``getChecks(owner, repo, ref)`` - Get CI check status

**Example:**

.. code-block:: typescript

   import { GitHubClient } from 'agentic-control/github';

   const client = new GitHubClient({ token: process.env.GITHUB_TOKEN });
   
   const pr = await client.getPullRequest('owner', 'repo', 123);
   console.log(pr.title, pr.state);
   
   const checks = await client.getChecks('owner', 'repo', pr.head.sha);
   const allPassing = checks.every(c => c.conclusion === 'success');

Configuration
-------------

The GitHub client requires authentication:

.. code-block:: typescript

   // Using environment variable
   const client = new GitHubClient({
     token: process.env.GITHUB_TOKEN
   });
   
   // Using GitHub App authentication
   const client = new GitHubClient({
     appId: 'app-id',
     privateKey: 'private-key',
     installationId: 'installation-id'
   });

Rate Limiting
-------------

The client handles rate limiting automatically:

- Automatic retry on rate limit errors
- Exponential backoff
- Request queuing

**Configuration:**

.. code-block:: typescript

   const client = new GitHubClient({
     token: process.env.GITHUB_TOKEN,
     rateLimit: {
       maxRetries: 3,
       retryDelay: 1000
     }
   });
