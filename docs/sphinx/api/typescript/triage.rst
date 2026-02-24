Triage Module
=============

The Triage module provides issue analysis, routing, and resolution capabilities.

.. contents:: Contents
   :local:
   :depth: 2

Overview
--------

The triage module analyzes and routes issues, providing:

- Automated issue analysis
- Priority classification
- Assignment recommendations
- Resolution suggestions
- PR triage capabilities

Triage Agent
------------

The ``TriageAgent`` class handles automated issue triage.

**Key Methods:**

- ``analyzeIssue(issue)`` - Analyze an issue and provide recommendations
- ``classifyPriority(issue)`` - Classify issue priority
- ``suggestAssignment(issue)`` - Suggest team/person assignment
- ``generateResolution(issue)`` - Generate resolution suggestions

**Example:**

.. code-block:: typescript

   import { TriageAgent } from 'agentic-control/triage';

   const agent = new TriageAgent();
   const result = await agent.analyzeIssue(issue);
   console.log(result.priority, result.suggestedAssignment);

Analyzer
--------

The ``Analyzer`` class provides deep analysis of code and issues.

**Capabilities:**

- Code pattern analysis
- Impact assessment
- Dependency tracking
- Risk evaluation

PR Triage Agent
---------------

The ``PRTriageAgent`` specializes in pull request analysis.

**Features:**

- Review feedback analysis
- CI status tracking
- Merge readiness assessment
- Blocker identification

**Example:**

.. code-block:: typescript

   import { PRTriageAgent } from 'agentic-control/triage';

   const agent = new PRTriageAgent({ repo: 'owner/repo' });
   const status = await agent.analyzePR(123);

Resolver
--------

The ``Resolver`` class handles automated issue resolution.

**Capabilities:**

- Automated fixes for common issues
- Code suggestions
- Documentation updates
- Test generation

Security
--------

The ``Security`` module provides security analysis capabilities.

**Features:**

- Vulnerability detection
- Security best practices checking
- Sensitive data detection
- Access control validation

Types
-----

Common types used throughout the triage module:

- ``FeedbackSeverity`` - Issue severity levels (critical, high, medium, low, info)
- ``FeedbackStatus`` - Feedback status (unaddressed, addressed, dismissed, wont_fix)
- ``BlockerType`` - Types of blockers (ci_failure, review_feedback, merge_conflict, etc.)
- ``PRStatus`` - PR status (needs_work, needs_review, ready_to_merge, etc.)
