#!/bin/bash
# =============================================================================
# agentic-control entrypoint
# =============================================================================
# Runs the agentic-control CLI in a containerized environment
# =============================================================================

set -euo pipefail

# Pass all arguments directly to the agentic CLI
exec agentic "$@"
