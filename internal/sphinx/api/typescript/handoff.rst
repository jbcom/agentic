Handoff Module
==============

The Handoff module provides agent handoff management capabilities.

.. contents:: Contents
   :local:
   :depth: 2

Overview
--------

The handoff module manages transitions between agents:

- Context preservation during handoffs
- State synchronization
- Handoff protocols
- Rollback capabilities

Handoff Manager
---------------

The ``HandoffManager`` class orchestrates agent handoffs.

**Key Methods:**

- ``initiateHandoff(fromAgent, toAgent, context)`` - Start a handoff
- ``completeHandoff(handoffId)`` - Complete a pending handoff
- ``rollbackHandoff(handoffId)`` - Rollback a failed handoff
- ``getHandoffStatus(handoffId)`` - Get handoff status

**Example:**

.. code-block:: typescript

   import { HandoffManager } from 'agentic-control/handoff';

   const manager = new HandoffManager();
   
   const handoff = await manager.initiateHandoff({
     fromAgent: 'cursor-agent-1',
     toAgent: 'cursor-agent-2',
     context: {
       task: 'code-review',
       files: ['src/main.ts'],
       state: { progress: 50 }
     }
   });
   
   // Wait for handoff to complete
   await manager.completeHandoff(handoff.id);

Context Preservation
--------------------

Handoffs preserve:

- Task state and progress
- Agent memory and context
- File modifications
- Error history

**Example:**

.. code-block:: typescript

   const context = {
     task: {
       id: 'task-123',
       type: 'review',
       progress: 75
     },
     memory: {
       analysisResults: [...],
       pendingActions: [...]
     },
     files: {
       modified: ['src/feature.ts'],
       created: ['src/test.ts']
     }
   };
   
   await manager.initiateHandoff({
     fromAgent: 'agent-1',
     toAgent: 'agent-2',
     context
   });

Handoff Protocols
-----------------

Supported protocols:

- **Immediate** - Direct handoff without waiting
- **Graceful** - Wait for current task completion
- **Checkpoint** - Create checkpoint before handoff
- **Rollback** - Support rollback on failure

**Example:**

.. code-block:: typescript

   await manager.initiateHandoff({
     fromAgent: 'agent-1',
     toAgent: 'agent-2',
     protocol: 'graceful',
     timeout: 30000
   });
