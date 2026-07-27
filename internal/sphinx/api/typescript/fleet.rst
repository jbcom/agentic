Fleet Module
============

The Fleet module provides agent fleet management and orchestration capabilities.

.. contents:: Contents
   :local:
   :depth: 2

Overview
--------

The fleet module manages AI agent fleets, providing:

- Agent registration and discovery
- Task distribution and load balancing
- Health monitoring and status tracking
- Cursor API integration for IDE agents

Fleet Manager
-------------

The ``FleetManager`` class is the main entry point for fleet operations.

**Key Methods:**

- ``registerAgent(agent)`` - Register a new agent with the fleet
- ``getAgentStatus(agentId)`` - Get the status of a specific agent
- ``assignTask(taskId, agentId)`` - Assign a task to an agent
- ``broadcastMessage(message)`` - Send a message to all agents

**Example:**

.. code-block:: typescript

   import { FleetManager } from 'agentic-control/fleet';

   const fleet = new FleetManager();
   fleet.registerAgent({ id: 'agent-1', type: 'cursor' });

Cursor API
----------

The ``CursorAPI`` class provides integration with Cursor IDE agents.

**Key Features:**

- Session management
- Command execution
- Status monitoring
- Error handling

**Example:**

.. code-block:: typescript

   import { CursorAPI } from 'agentic-control/fleet';

   const cursor = new CursorAPI({ sessionId: 'session-123' });
   await cursor.executeCommand('runTests');
