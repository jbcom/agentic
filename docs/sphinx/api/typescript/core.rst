Core Module
===========

The Core module provides shared utilities, configuration, and type definitions.

.. contents:: Contents
   :local:
   :depth: 2

Overview
--------

The core module provides:

- Configuration management
- AI provider integration
- Token counting utilities
- Type definitions

Configuration
-------------

The ``loadConfig`` function loads and validates configuration.

**Supported Formats:**

- ``agentic.config.js``
- ``agentic.config.ts``
- ``agentic.config.json``
- ``package.json`` (``agentic`` field)

**Example:**

.. code-block:: typescript

   import { loadConfig } from 'agentic-control/core';

   const config = await loadConfig();
   console.log(config.fleet, config.triage);

**Configuration Schema:**

.. code-block:: typescript

   interface AgenticConfig {
     fleet?: {
       maxAgents?: number;
       healthCheckInterval?: number;
       defaultTimeout?: number;
     };
     triage?: {
       autoAssign?: boolean;
       priorityRules?: PriorityRule[];
       defaultSeverity?: string;
     };
     github?: {
       token?: string;
       baseUrl?: string;
     };
   }

Providers
---------

The ``providers`` module integrates AI providers.

**Supported Providers:**

- Anthropic (Claude)
- OpenAI (GPT-4, GPT-3.5)
- Google (Gemini)
- Azure OpenAI
- Mistral

**Example:**

.. code-block:: typescript

   import { createProvider } from 'agentic-control/core';

   const provider = createProvider({
     type: 'anthropic',
     model: 'claude-sonnet-4-20250514',
     apiKey: process.env.ANTHROPIC_API_KEY
   });
   
   const response = await provider.complete({
     prompt: 'Analyze this code...',
     maxTokens: 1000
   });

Tokens
------

The ``tokens`` module provides token counting utilities.

**Functions:**

- ``countTokens(text, model)`` - Count tokens in text
- ``estimateCost(tokens, model)`` - Estimate API cost
- ``truncateToTokenLimit(text, limit, model)`` - Truncate to token limit

**Example:**

.. code-block:: typescript

   import { countTokens, estimateCost } from 'agentic-control/core';

   const code = await readFile('src/main.ts', 'utf-8');
   const tokens = countTokens(code, 'claude-sonnet-4-20250514');
   const cost = estimateCost(tokens, 'claude-sonnet-4-20250514');
   
   console.log(`Tokens: ${tokens}, Estimated cost: $${cost.toFixed(4)}`);

Types
-----

Common types exported from the core module:

**Agent Types:**

- ``Agent`` - Agent definition
- ``AgentStatus`` - Agent status information
- ``AgentConfig`` - Agent configuration

**Task Types:**

- ``Task`` - Task definition
- ``TaskStatus`` - Task status
- ``TaskResult`` - Task execution result

**Provider Types:**

- ``ProviderConfig`` - Provider configuration
- ``CompletionOptions`` - Completion request options
- ``CompletionResult`` - Completion response
