# vendor-connectors Integration - Quick Reference

This is a quick reference for using agentic-crew with vendor-connectors.

## 🚀 Quick Start

### 1. Setup in vendor-connectors

```bash
# In vendor-connectors/pyproject.toml, add:
[project.optional-dependencies]
crews = ["agentic-crew[crewai]>=0.1.0"]

# Install
uv sync --extra crews
```

### 2. Copy Example Configuration

```bash
# Copy example .crewai/ directory to vendor-connectors
cp -r agentic-crew/examples/vendor-connectors/.crewai vendor-connectors/
```

### 3. Generate a Connector

```bash
cd vendor-connectors
agentic-crew run vendor-connectors connector_builder \
  --input '{"api_docs_url": "https://docs.meshy.ai/en", "vendor_name": "meshy"}'
```

### 4. Review and Integrate

```bash
# Generated code is in /tmp/connector-output/meshy/
ls -la /tmp/connector-output/meshy/

# Copy to vendor-connectors
cp -r /tmp/connector-output/meshy/ src/vendor_connectors/

# Add tests and submit PR
```

## 📋 Key Commands

```bash
# List all crews
agentic-crew list vendor-connectors

# Show crew details
agentic-crew info vendor-connectors connector_builder

# Run with JSON output
agentic-crew run vendor-connectors connector_builder \
  --input '{"api_docs_url": "..."}' \
  --json | jq '.'

# Run from file
agentic-crew run vendor-connectors connector_builder \
  --file api_spec.yaml
```

## 🔧 Directory Structure

```
vendor-connectors/
├── .crewai/
│   ├── manifest.yaml           # Crew definitions
│   ├── crews/
│   │   └── connector_builder/
│   │       ├── agents.yaml     # 4 agents
│   │       └── tasks.yaml      # 5 tasks
│   └── knowledge/
│       └── api_patterns/       # REST API patterns
└── src/vendor_connectors/
    └── {vendor}/               # Generated connectors
```

## 🎯 What Gets Generated

For each connector, the crew generates:

- **client.py** - Main HTTP client using httpx
- **models.py** - Pydantic models for requests/responses
- **exceptions.py** - Custom exceptions
- **tools.py** - Framework-agnostic tools (CrewAI/LangGraph/Strands)
- **tests/** - Comprehensive test suite
- **__init__.py** - Package exports

## 🔄 Tool Sharing Pattern

vendor-connectors can export tools for use in agentic-crew crews:

```python
# In vendor_connectors/meshy/tools.py
def get_tools(framework: Literal["crewai", "langgraph", "strands"] = "crewai"):
    """Return tools in framework-native format."""
    # ... implementation
```

Use in crews:

```yaml
# In any manifest.yaml
crews:
  my_crew:
    tools:
      - vendor_connectors.meshy.tools:get_tools
```

## 📚 Documentation

- **Full Guide**: [docs/VENDOR_CONNECTORS_INTEGRATION.md](../docs/VENDOR_CONNECTORS_INTEGRATION.md)
- **Examples**: [examples/vendor-connectors/](.)
- **Main README**: [README.md](../README.md)

## 🐛 Troubleshooting

| Issue | Solution |
|-------|----------|
| Package not found | Ensure `.crewai/manifest.yaml` exists |
| Crew not found | Check crew name matches manifest exactly |
| No framework available | Install: `uv sync --extra crews` |
| Tool import fails | Install: `pip install vendor-connectors[meshy]` |

## 🔑 Environment Variables

```bash
# Required for crews
export ANTHROPIC_API_KEY="sk-ant-api03-..."

# Optional for vendor-specific tools
export MESHY_API_KEY="msy_..."
```

## ✅ Exit Codes

- `0` - Success
- `1` - Crew execution failed
- `2` - Configuration error (package/crew not found)

Use in scripts:
```bash
if agentic-crew run vendor-connectors connector_builder --input '...' --json > result.json; then
  echo "Success!"
  jq '.output' result.json
else
  echo "Failed!"
  jq '.error' result.json
  exit 1
fi
```
