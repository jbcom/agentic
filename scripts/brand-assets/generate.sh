#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MESHY_PKG="$REPO_ROOT/packages/meshy-content-generator"

# Require MESHY_API_KEY
if [ -z "${MESHY_API_KEY:-}" ]; then
  echo "Error: MESHY_API_KEY is required. Set it in your environment or via Doppler."
  echo "  doppler run -- bash $0"
  exit 1
fi

echo "=== Generating Agentic Brand Assets ==="
echo ""

for manifest_dir in "$SCRIPT_DIR"/*/; do
  [ -f "$manifest_dir/manifest.json" ] || continue
  name=$(basename "$manifest_dir")
  echo "--- Generating: $name ---"

  npx content-gen run brand-image "$manifest_dir" \
    --pipelines "$MESHY_PKG/pipelines/definitions" \
    --tasks "$MESHY_PKG/tasks/definitions" \
    --meshy-api-key "$MESHY_API_KEY"

  echo "--- Done: $name ---"
  echo ""
done

echo "=== All brand assets generated ==="
echo "Check scripts/brand-assets/*/concept-*.png for results"
