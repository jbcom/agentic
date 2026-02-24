# Example vendor-connectors Integration

This directory contains example configurations showing how `vendor-connectors` can integrate with `agentic-crew`.

## Structure

This represents the `.crewai/` directory that would be created in the vendor-connectors repository:

```
vendor-connectors/
├── .crewai/
│   ├── manifest.yaml          # Crew definitions (this file)
│   ├── crews/
│   │   └── connector_builder/
│   │       ├── agents.yaml
│   │       └── tasks.yaml
│   └── knowledge/
│       └── api_patterns/
│           └── rest_api_best_practices.md
└── src/
    └── vendor_connectors/
        └── ... (generated connectors)
```

## Usage

1. Copy this structure to vendor-connectors repository:
   ```bash
   cp -r examples/vendor-connectors/.crewai vendor-connectors/
   ```

2. Install agentic-crew in vendor-connectors:
   ```bash
   cd vendor-connectors
   # Add to pyproject.toml optional-dependencies
   # Then:
   uv sync --extra crews
   ```

3. Run the connector_builder crew:
   ```bash
   agentic-crew run vendor-connectors connector_builder \
     --input '{"api_docs_url": "https://docs.meshy.ai/en", "vendor_name": "meshy"}'
   ```

## Expected Output

The crew will:
1. Analyze the API documentation
2. Generate Python connector code
3. Create tools for CrewAI/LangGraph/Strands
4. Output to `/tmp/connector-output/{vendor_name}/`

You can then review and integrate the generated code into vendor-connectors.
