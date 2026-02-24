---
title: LangGraph Integration
description: Use agentic-crew with LangGraph for graph-based agent workflows
---

# LangGraph Integration

`agentic-crew` supports [LangGraph](https://github.com/langchain-ai/langgraph) as an alternative to CrewAI for graph-based agent workflows. LangGraph excels at complex, stateful workflows with conditional branching.

## Installation

```bash
# Install with LangGraph support
pip install agentic-crew[langgraph]
```

## When to Use LangGraph

| Use Case | Best Framework |
|----------|----------------|
| Simple sequential tasks | CrewAI |
| Complex branching logic | LangGraph |
| Stateful conversations | LangGraph |
| Human-in-the-loop | LangGraph |
| Parallel task execution | CrewAI |
| Hierarchical delegation | CrewAI |

## Quick Start

### 1. Define Your Crew

The same YAML format works for LangGraph:

```yaml
# .crewai/manifest.yaml
name: customer-support
version: "1.0"

crews:
  support_flow:
    description: Handle customer support requests
    agents: crews/support_flow/agents.yaml
    tasks: crews/support_flow/tasks.yaml
```

```yaml
# crews/support_flow/agents.yaml
classifier:
  role: Request Classifier
  goal: Classify incoming support requests
  backstory: Expert at understanding customer intent

technical_support:
  role: Technical Support Agent
  goal: Resolve technical issues
  backstory: Deep technical knowledge of the product

billing_support:
  role: Billing Support Agent
  goal: Handle billing inquiries
  backstory: Expert in billing and subscriptions
```

```yaml
# crews/support_flow/tasks.yaml
classify_request:
  description: Classify the customer request into a category
  agent: classifier
  expected_output: One of [technical, billing, general]

handle_technical:
  description: Resolve technical issue
  agent: technical_support
  expected_output: Resolution or escalation
  condition: classify_request == "technical"

handle_billing:
  description: Handle billing inquiry
  agent: billing_support
  expected_output: Resolution or escalation
  condition: classify_request == "billing"
```

### 2. Run with LangGraph

```python
from agentic_crew import run_crew

# Force LangGraph framework
result = run_crew(
    package="customer-support",
    crew="support_flow",
    inputs={"request": "I can't login to my account"},
    framework="langgraph"
)
```

Or from CLI:

```bash
agentic-crew run customer-support support_flow \
  --framework langgraph \
  --input "I can't login to my account"
```

## LangGraph-Specific Features

### State Management

LangGraph maintains state across the workflow:

```python
from agentic_crew.core.decomposer import get_runner
from typing import TypedDict

class SupportState(TypedDict):
    request: str
    category: str
    resolution: str
    escalated: bool

runner = get_runner("langgraph")
config = runner.load_config("./customer-support", "support_flow")

# Build with custom state
graph = runner.build_graph(config, state_schema=SupportState)

# Run with initial state
result = graph.invoke({
    "request": "I can't login",
    "category": "",
    "resolution": "",
    "escalated": False
})

print(result["resolution"])
```

### Conditional Edges

Define branching logic:

```yaml
# tasks.yaml
classify:
  description: Classify the request
  agent: classifier
  expected_output: category string
  transitions:
    technical: handle_technical
    billing: handle_billing
    general: handle_general
```

### Checkpointing

Enable checkpoints for recovery:

```python
from langgraph.checkpoint.sqlite import SqliteSaver

runner = get_runner("langgraph")
config = runner.load_config("./my-project", "my_crew")

# Add checkpointing
memory = SqliteSaver.from_conn_string(":memory:")
graph = runner.build_graph(config, checkpointer=memory)

# Run with thread ID
result = graph.invoke(
    {"input": "..."},
    config={"configurable": {"thread_id": "user-123"}}
)
```

### Human-in-the-Loop

Add human approval steps:

```yaml
# tasks.yaml
propose_solution:
  description: Propose a solution
  agent: solver
  expected_output: Proposed solution

await_approval:
  description: Wait for human approval
  type: human_input  # Special task type
  expected_output: approved or rejected

implement_solution:
  description: Implement approved solution
  agent: implementer
  condition: await_approval == "approved"
```

```python
from agentic_crew.core.decomposer import get_runner

runner = get_runner("langgraph")
graph = runner.build_graph(config)

# Run until human input needed
result = graph.invoke({"input": "..."})

if result.get("awaiting_human"):
    # Get human decision
    decision = input("Approve? (y/n): ")
    
    # Continue with decision
    result = graph.invoke(
        {"approval": "approved" if decision == "y" else "rejected"},
        config={"configurable": {"thread_id": result["thread_id"]}}
    )
```

## Building Custom Graphs

For complex workflows, build LangGraph directly:

```python
from langgraph.graph import StateGraph, END
from langchain_anthropic import ChatAnthropic
from agentic_crew import get_crew_config

# Load agent definitions
config = get_crew_config("./my-project", "my_crew")

# Build custom graph
graph = StateGraph(dict)

# Add nodes from config
for agent in config.agents:
    llm = ChatAnthropic(model="claude-sonnet-4-20250514")
    
    def create_node(agent_config):
        def node(state):
            response = llm.invoke(f"""
                Role: {agent_config.role}
                Goal: {agent_config.goal}
                
                Task: {state.get('current_task')}
                Input: {state.get('input')}
            """)
            return {"output": response.content}
        return node
    
    graph.add_node(agent.name, create_node(agent))

# Add edges
graph.add_edge("classifier", "router")
graph.add_conditional_edges(
    "router",
    lambda x: x.get("category"),
    {
        "technical": "tech_support",
        "billing": "billing_support",
        "general": END
    }
)

# Compile
app = graph.compile()
result = app.invoke({"input": "Help me with billing"})
```

## Streaming

LangGraph supports streaming responses:

```python
from agentic_crew.core.decomposer import get_runner

runner = get_runner("langgraph")
graph = runner.build_graph(config)

# Stream events
for event in graph.stream({"input": "..."}):
    print(event)
```

## Best Practices

### 1. Define Clear State Schema

```python
from typing import TypedDict, Literal

class WorkflowState(TypedDict):
    input: str
    category: Literal["technical", "billing", "general"]
    messages: list[dict]
    resolved: bool
    escalation_level: int
```

### 2. Use Meaningful Node Names

```python
# Good
graph.add_node("classify_intent", classify_node)
graph.add_node("resolve_technical", tech_node)

# Bad
graph.add_node("node1", classify_node)
graph.add_node("node2", tech_node)
```

### 3. Handle Errors Gracefully

```python
def safe_node(state):
    try:
        result = process(state)
        return {"output": result, "error": None}
    except Exception as e:
        return {"output": None, "error": str(e)}

graph.add_conditional_edges(
    "process",
    lambda x: "error_handler" if x.get("error") else "next_step"
)
```

## Comparison with CrewAI

| Feature | CrewAI | LangGraph |
|---------|--------|-----------|
| Setup complexity | Low | Medium |
| Branching logic | Limited | Excellent |
| State management | Basic | Advanced |
| Human-in-the-loop | Manual | Built-in |
| Streaming | Limited | Full support |
| Checkpointing | No | Yes |
| Learning curve | Easy | Moderate |

## Next Steps

- [CrewAI Integration](/integrations/crewai/) - CrewAI alternative
- [Python Examples](/examples/python/) - More examples
- [@agentic/crew Package](/packages/crew/) - Package reference
